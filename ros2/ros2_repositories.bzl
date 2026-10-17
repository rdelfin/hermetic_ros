load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def _ros2_repositories_impl(_module_ctx):
    http_archive(
        name = "rcutils",
        build_file = "//ros2:rcutils.BUILD.bazel",
        strip_prefix = "rcutils-6.7.6",
        url = "https://github.com/ros2/rcutils/archive/refs/tags/6.7.6.tar.gz",
        integrity = "sha256-T6ueMQwpmah1H+fk4D1co41DCLrAvjLnBf8KZZZeHkI=",
    )

ros2_repositories = module_extension(implementation = _ros2_repositories_impl)
