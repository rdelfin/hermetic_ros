load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def _ros2_repositories_impl(_module_ctx):
    http_archive(
        name = "rcutils",
        build_file = "//ros2:rcutils.BUILD.bazel",
        strip_prefix = "rcutils-7.1.3",
        url = "https://github.com/ros2/rcutils/archive/refs/tags/7.1.3.tar.gz",
        integrity = "sha256-6Yw/d6AzvOXLEO2yx3uIHYjdIloNcFsq5hcK7s+z7ts=",
    )

ros2_repositories = module_extension(implementation = _ros2_repositories_impl)
