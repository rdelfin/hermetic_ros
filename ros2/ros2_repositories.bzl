load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def _ros2_repositories_impl(_module_ctx):
    http_archive(
        name = "rcutils",
        build_file = "//ros2:rcutils.BUILD.bazel",
        strip_prefix = "rcutils-7.1.4",
        url = "https://github.com/ros2/rcutils/archive/refs/tags/7.1.4.tar.gz",
        integrity = "sha256-Vx8yi1i9PPczEn0pJVgVoLiQuZbsVHYWKX2k/Z6B+WU="
    )

ros2_repositories = module_extension(implementation = _ros2_repositories_impl)
