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

    http_archive(
        name = "rosidl_dynamic_typesupport",
        build_file = "//ros2:rosidl_dynamic_typesupport.BUILD.bazel",
        strip_prefix = "rosidl_dynamic_typesupport-0.4.1",
        url = "https://github.com/ros2/rosidl_dynamic_typesupport/archive/refs/tags/0.4.1.tar.gz",
        integrity = "sha256-HbnqjkivrT6F+kA/AySFpnIWmShsdHM3NsIjv6rHa6I=",
    )

    http_archive(
        name = "rmw",
        build_file = "//ros2:rmw.BUILD.bazel",
        strip_prefix = "rmw-7.10.2",
        url = "https://github.com/ros2/rmw/archive/refs/tags/7.10.2.tar.gz",
        integrity = "sha256-MqaRCehFJpnOoJfP+r8cpALOQEKmIYITylT9rN61S7E=",
    )

    http_archive(
        name = "rcpputils",
        build_file = "//ros2:rcpputils.BUILD.bazel",
        strip_prefix = "rcpputils-2.14.5",
        url = "https://github.com/ros2/rcpputils/archive/refs/tags/2.14.5.tar.gz",
        integrity = "sha256-HDsIbb+tfOauB51lOTJNIK6+IWnIUzg/ruJi+p38Gxw=",
    )

    http_archive(
        name = "rosidl_typesupport_fastrtps",
        build_file = "//ros2:rosidl_typesupport_fastrtps.BUILD.bazel",
        strip_prefix = "rosidl_typesupport_fastrtps-3.9.7",
        url = "https://github.com/ros2/rosidl_typesupport_fastrtps/archive/refs/tags/3.9.7.tar.gz",
        integrity = "sha256-xCe87EChM5odAbgMRmYTqmdUhkDwcP2mGULyR+YHzCc=",
    )

    http_archive(
        name = "ros2_tracing",
        build_file = "//ros2:ros2_tracing.BUILD.bazel",
        strip_prefix = "ros2_tracing-8.10.2",
        url = "https://github.com/ros2/ros2_tracing/archive/refs/tags/8.10.2.tar.gz",
        integrity = "sha256-s7pfXqiwIUlZtWeSs/qjnXAYsPPMpqi3WTdTZ0sEEqs=",
    )

    http_archive(
        name = "zenoh_c",
        build_file = "//ros2:zenoh_c.BUILD.bazel",
        patches = ["//ros2/zenoh_c:zenoh_c.patch"],
        patch_args = ["-p1"],
        strip_prefix = "zenoh-c-05bd370343b5161ca9269649b9a914c9c2dc4170",
        url = "https://github.com/eclipse-zenoh/zenoh-c/archive/05bd370343b5161ca9269649b9a914c9c2dc4170.tar.gz",
        integrity = "sha256-noz6bJNtGHhEIcGqq29D6hgGsV7EG5HESI44CtVddXI=",
    )

ros2_repositories = module_extension(implementation = _ros2_repositories_impl)
