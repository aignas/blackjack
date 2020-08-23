"""
DO NOT EDIT!

This file is automatically @generated by blackjack.
"""
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def cargo_dependencies():


    http_archive(
        name = "crates_io_cc_1.0.59",
        url = "https://crates.io/api/v1/crates/cc/1.0.59/download",
        sha256 = "66120af515773fb005778dc07c261bd201ec8ce50bd6e7144c927753fe013381",
        strip_prefix = "cc-1.0.59",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "cc",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = [],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_cfg_if_0.1.10",
        url = "https://crates.io/api/v1/crates/cfg-if/0.1.10/download",
        sha256 = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822",
        strip_prefix = "cfg-if-0.1.10",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "cfg_if",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = [],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_crc32fast_1.2.0",
        url = "https://crates.io/api/v1/crates/crc32fast/1.2.0/download",
        sha256 = "ba125de2af0df55319f41944744ad91c71113bf74a4646efff39afe1f6842db1",
        strip_prefix = "crc32fast-1.2.0",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "crc32fast",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@crates_io_cfg_if_0.1.10//:cfg_if"],
    proc_macro_deps = [],
    edition = "2015",
    crate_features = ["default", "std"],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_flate2",
        url = "https://crates.io/api/v1/crates/flate2/1.0.17/download",
        sha256 = "766d0e77a2c1502169d4a93ff3b8c15a71fd946cd0126309752104e5f3c46d94",
        strip_prefix = "flate2-1.0.17",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "flate2",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@crates_io_cfg_if_0.1.10//:cfg_if", "@crates_io_crc32fast_1.2.0//:crc32fast", "@crates_io_libc_0.2.76//:libc", "@libz_sys"],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = ["any_zlib", "libz-sys", "zlib"],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_libc_0.2.76",
        url = "https://crates.io/api/v1/crates/libc/0.2.76/download",
        sha256 = "755456fae044e6fa1ebbbd1b3e902ae19e73097ed4ed87bb79934a867c007bc3",
        strip_prefix = "libc-0.2.76",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "libc",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = [],
    edition = "2015",
    crate_features = ["default", "std"],
    rustc_flags = ["--cap-lints=allow"] + ["--cfg=libc_priv_mod_use", "--cfg=libc_union", "--cfg=libc_const_size_of", "--cfg=libc_align", "--cfg=libc_core_cvoid", "--cfg=libc_packedN", "--cfg=libc_cfg_target_vendor"],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_libz_sys_1.1.0",
        url = "https://crates.io/api/v1/crates/libz-sys/1.1.0/download",
        sha256 = "af67924b8dd885cccea261866c8ce5b74d239d272e154053ff927dae839f5ae9",
        strip_prefix = "libz-sys-1.1.0",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "libz_sys",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = [],
    edition = "2015",
    crate_features = [],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_pkg_config_0.3.18",
        url = "https://crates.io/api/v1/crates/pkg-config/0.3.18/download",
        sha256 = "d36492546b6af1463394d46f0c834346f31548646f6ba10849802c9c9a27ac33",
        strip_prefix = "pkg-config-0.3.18",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "pkg_config",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = [],
    edition = "2015",
    crate_features = [],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    
