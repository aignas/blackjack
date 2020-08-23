use cargo_metadata::{
    DependencyKind, Metadata, MetadataCommand, Node, NodeDep, Package, PackageId, Target,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};

const CARGO_TOML_RUNFILES_PATH: &'static str = "Cargo.toml";
const CARGO_RUNFILES_PATH: &'static str = "external/blackjack_cargo/cargo";

#[derive(Debug, Deserialize, Default)]
struct BlackjackMetadataWrapper {
    #[serde(default)]
    blackjack: BlackjackMetadata,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct CrateOpts {
    #[serde(default)]
    build_script: bool,
    #[serde(default)]
    rustc_flags: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
struct BlackjackMetadata {
    #[serde(default)]
    rustc_flags: HashMap<String, Vec<String>>,
    #[serde(default)]
    prefix: String,

    #[serde(flatten)]
    crate_opts: HashMap<String, CrateOpts>,
}

impl BlackjackMetadata {
    pub fn new(package: &Package) -> BlackjackMetadata {
        let mut blackjack_metadata = if package.metadata.is_null() {
            BlackjackMetadata::default()
        } else {
            serde_json::from_value::<BlackjackMetadataWrapper>(package.metadata.clone())
                .expect("Failed to parse metadata")
                .blackjack
        };
        eprintln!("{:#?}", blackjack_metadata);
        if blackjack_metadata.prefix == "" {
            blackjack_metadata.prefix = "crates_io".to_string();
        } else {
            eprintln!("Prefix: {}", blackjack_metadata.prefix);
        }
        blackjack_metadata
    }
}

struct Blackjack {
    metadata: Metadata,
    blackjack_metadata: BlackjackMetadata,
    packages: HashMap<PackageId, Package>,
    root_id: PackageId,
    root_dependencies: Vec<PackageId>,
}

impl Blackjack {
    pub fn new(mut metadata: Metadata) -> Blackjack {
        // Sort the nodes to make traversal deterministic.
        metadata
            .resolve
            .as_mut()
            .unwrap()
            .nodes
            .sort_by(|a, b| a.id.cmp(&b.id));
        let packages: HashMap<PackageId, Package> = metadata
            .packages
            .iter()
            .map(|p| (p.id.clone(), p.clone()))
            .collect();
        let resolve = metadata.resolve.as_ref().unwrap();
        let root_id = resolve.root.clone().unwrap();
        let blackjack_metadata = BlackjackMetadata::new(packages.get(&root_id).unwrap());
        let root_dependencies = resolve
            .nodes
            .iter()
            .find(|n| &n.id == &root_id)
            .unwrap()
            .deps
            .iter()
            .map(|d| d.pkg.clone())
            .collect();
        Blackjack {
            metadata,
            packages,
            blackjack_metadata,
            root_dependencies,
            root_id,
        }
    }

    pub fn is_root(&self, package: &PackageId) -> bool {
        &self.root_id == package
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> + '_ {
        self.metadata.resolve.as_ref().unwrap().nodes.iter()
    }

    fn crate_type(&self, package_id: &PackageId) -> CrateType {
        let package = self
            .metadata
            .packages
            .iter()
            .find(|p| &p.id == package_id)
            .unwrap();
        match package.targets[0].crate_types[0].as_ref() {
            "proc-macro" => CrateType::ProcMacro,
            _ => CrateType::Lib,
        }
    }

    pub fn render_archive(&self, node: &Node) -> String {
        let package = self.packages.get(&node.id).unwrap();
        let archive_name = if self.root_dependencies.contains(&node.id) {
            format!(
                "{prefix}_{name}",
                prefix = self.blackjack_metadata.prefix,
                name = sanitize_name(&package.name),
            )
        } else {
            format!(
                "{prefix}_{name}_{version}",
                prefix = self.blackjack_metadata.prefix,
                name = sanitize_name(&package.name),
                version = package.version,
            )
        };
        format!(
            r#"
    http_archive(
        name = "{archive_name}",
        url = "https://crates.io/api/v1/crates/{name}/{version}/download",
        strip_prefix = "{name}-{version}",
        type = "tar.gz",
        build_file_content = """{build_file_content}""",
    )
    "#,
            archive_name = archive_name,
            name = package.name,
            version = package.version,
            build_file_content = self.render_build_file(node, package),
        )
    }

    fn dep_label(&self, package: &Package) -> String {
        if self.root_dependencies.contains(&package.id) {
            format!(
                "@{prefix}_{name}//:{name}",
                prefix = self.blackjack_metadata.prefix,
                name = sanitize_name(&package.name)
            )
        } else {
            format!(
                "@{prefix}_{name}_{version}//:{name}",
                prefix = self.blackjack_metadata.prefix,
                name = sanitize_name(&package.name),
                version = package.version,
            )
        }
    }

    fn render_build_file(&self, node: &Node, package: &Package) -> String {
        let target = library_target(package);
        let all_deps: Vec<NodeDep> = node
            .deps
            .iter()
            .filter(|d| d.dep_kinds.iter().any(|k| k.kind == DependencyKind::Normal))
            .cloned()
            .collect();

        let aliases: HashMap<String, String> = all_deps
            .iter()
            .filter_map(|d| {
                let dep_name = sanitize_name(&d.name);
                let package = &self.packages.get(&d.pkg).unwrap();
                let package_name = sanitize_name(&package.name);
                if dep_name == package_name {
                    None
                } else {
                    Some((self.dep_label(package), dep_name))
                }
            })
            .collect();

        let mut deps: Vec<String> = all_deps
            .iter()
            .filter(|d| self.crate_type(&d.pkg) == CrateType::Lib)
            .map(|d| self.dep_label(self.packages.get(&d.pkg).unwrap()))
            .collect();
        let proc_macro_deps: Vec<String> = all_deps
            .iter()
            .filter(|d| self.crate_type(&d.pkg) == CrateType::ProcMacro)
            .map(|d| self.dep_label(self.packages.get(&d.pkg).unwrap()))
            .collect();
        let crate_opts = self
            .blackjack_metadata
            .crate_opts
            .get(&package.name)
            .cloned()
            .unwrap_or_default();
        let build_script = if crate_opts.build_script {
            deps.push(":build_script".to_string());
            format!(
                r#"
load("@io_bazel_rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")

cargo_build_script(
    name = "build_script",
    srcs = ["build.rs"],
)
                "#
            )
        } else {
            "".to_string()
        };
        format!(
            r#"
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")
{build_script}
rust_library(
    name = "{name}",
    aliases = {aliases:?},
    srcs = glob(["**/*.rs"]),
    crate_type = "{crate_type}",
    deps = {deps:?},
    proc_macro_deps = {proc_macro_deps:?},
    edition = "{edition}",
    crate_features = {crate_features:?},
    rustc_flags = ["--cap-lints=allow"] + {rustc_flags:?},
    visibility = ["//visibility:public"],
)
    "#,
            build_script = build_script,
            name = sanitize_name(&package.name),
            aliases = aliases,
            crate_type = target.crate_types[0],
            deps = deps,
            proc_macro_deps = proc_macro_deps,
            edition = target.edition,
            crate_features = node.features,
            rustc_flags = crate_opts.rustc_flags,
        )
    }
}

fn main() {
    // This is somewhat of an implementation detail
    let cargo_toml_path = std::fs::read_link(CARGO_TOML_RUNFILES_PATH)
        .unwrap_or(PathBuf::from(CARGO_TOML_RUNFILES_PATH));
    let mut output_path = cargo_toml_path.clone();
    output_path.pop();
    output_path.push("cargo_dependencies.bzl");

    let mut metadata = MetadataCommand::new();
    metadata.manifest_path(cargo_toml_path).other_options(vec![
        "--frozen".to_string(),
        "--offline".to_string(),
        // TODO make this configurable
        "--filter-platform".to_string(),
        "x86_64-unknown-linux-gnu".to_string(),
    ]);

    let cargo_runfiles_path = Path::new(CARGO_RUNFILES_PATH);
    if cargo_runfiles_path.exists() {
        eprintln!("Found cargo in runfiles: {}", cargo_runfiles_path.display());
        metadata.cargo_path(cargo_runfiles_path);
    } else {
        eprintln!(
            "Using default cargo in path. Working dir: {}",
            std::env::current_dir().unwrap().display()
        );
    }

    let metadata = metadata.exec().expect("cargo metadata failed");
    let blackjack = Blackjack::new(metadata);

    eprintln!("Writing output to {}", output_path.display());
    eprintln!("Press enter to continue, or Ctrl-C to abort");
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read stdin");
    let output = std::fs::File::create(output_path).expect("Could not open output file");
    let mut output = std::io::BufWriter::new(output);

    writeln!(
        output,
        r#""""
DO NOT EDIT!

This file is automatically generated by blackjack.
"""
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def cargo_dependencies():
"#
    )
    .expect("Failed to write to output file");

    for node in blackjack.nodes() {
        if blackjack.is_root(&node.id) {
            continue;
        }
        writeln!(output, "{}", blackjack.render_archive(node))
            .expect("Failed to write to output file");
    }

    eprintln!("Done.");
}

#[derive(PartialEq)]
enum CrateType {
    Lib,
    ProcMacro,
}

fn sanitize_name(s: &str) -> String {
    s.replace("-", "_")
}

fn library_target(package: &Package) -> &Target {
    package
        .targets
        .iter()
        .find(|target| {
            target
                .kind
                .iter()
                .any(|kind| kind == "lib" || kind == "proc-macro")
        })
        .expect("dependency provides not lib or proc-macro target")
}
