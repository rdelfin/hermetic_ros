load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# ROS packages from this registry:
# https://github.com/ros2/ros2/blob/8577ad203d60bdd0a49eba7048709d47efb73b97/ros2.repos
def _ros2_repositories_impl(_module_ctx):
    http_archive(
        name = "rcutils",
        build_file = "//ros2:rcutils.BUILD.bazel",
        strip_prefix = "rcutils-7.1.3",
        url = "https://github.com/ros2/rcutils/archive/refs/tags/7.1.3.tar.gz",
        integrity = "sha256-6Yw/d6AzvOXLEO2yx3uIHYjdIloNcFsq5hcK7s+z7ts=",
    )

    http_archive(
        name = "rosidl",
        build_file = "//ros2:rosidl.BUILD.bazel",
        strip_prefix = "rosidl-5.2.1",
        url = "https://github.com/ros2/rosidl/archive/refs/tags/5.2.1.tar.gz",
        integrity = "sha256-Zqiikhah2ac2gS9s3Tx4682TZejGdX14eSyz+RgpO3s=",
    )

ros2_repositories = module_extension(implementation = _ros2_repositories_impl)
