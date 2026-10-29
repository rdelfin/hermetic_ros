#!/usr/bin/env bash
# Regenerates the checked-in opaque_types_<triple>.rs,
# zenoh_configure_<triple>.h and zenoh_opaque_<triple>.h files with the
# upstream zenoh-c build.rs. Run it from the workspace after a zenoh-c bump, a
# Rust toolchain bump, or a change to ZENOHC_FEATURES in zenoh_c.BUILD.bazel.
#
# Needs network access, rustup, and clang.
set -euo pipefail

out_dir="$(dirname "$(realpath "$0")")"
# Type layouts can change between rustc releases, so this must match the Rust
# toolchain version in MODULE.bazel.
rust_version="1.99.1"
triples=(x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu)
# Must match ZENOHC_FEATURES in zenoh_c.BUILD.bazel, minus the defaults.
features="shared-memory,unstable,transport_serial"

glibc_repo() {
  local triple="$1"
  echo "llvm++glibc+glibc_headers_${triple%%-*}-linux-gnu.2.28"
}

fetch_targets=("@zenoh_c//:Cargo.toml")
for triple in "${triples[@]}"; do
  fetch_targets+=("@@$(glibc_repo "${triple}")//:gnu_libc_headers")
done
bazel fetch "${fetch_targets[@]}"
external_dir="$(bazel info output_base)/external"

rustup toolchain install "${rust_version}" --profile minimal \
  --target "$(IFS=,; echo "${triples[*]}")"
# Overrides rust-toolchain.toml. build.rs also passes it on to its nested cargo.
export RUSTUP_TOOLCHAIN="${rust_version}"

work_dir="$(mktemp -d)"
trap 'rm -rf "${work_dir}"' EXIT

# build.rs writes into its source tree, so it runs on a copy. -L because the
# external repo can be a symlink into Bazel's repo contents cache.
cp -rL "${external_dir}/+ros2_repositories+zenoh_c" "${work_dir}/zenoh-c"
chmod -R u+w "${work_dir}/zenoh-c"
# zenoh_c.patch makes mod.rs include a file next to it, but cargo only has the
# copy that build.rs writes to OUT_DIR.
echo 'include!(concat!(env!("OUT_DIR"), "/opaque_types.rs"));' \
  >"${work_dir}/zenoh-c/src/opaque_types/mod.rs"

for triple in "${triples[@]}"; do
  echo "Generating opaque types for ${triple}" >&2
  # Some build scripts (e.g. ring) compile C code. cc-rs passes --target to
  # clang, and the glibc headers come from the hermetic llvm module.
  env \
    "CC_${triple//-/_}=clang" \
    "CFLAGS_${triple//-/_}=-nostdlibinc -isystem ${external_dir}/$(glibc_repo "${triple}")/include" \
    cargo check \
    --manifest-path "${work_dir}/zenoh-c/Cargo.toml" \
    --target "${triple}" \
    --features "${features}" \
    --target-dir "${work_dir}/target"

  generated=("${work_dir}"/target/"${triple}"/debug/build/zenoh-c-*/out/opaque_types.rs)
  if [[ ${#generated[@]} -ne 1 || ! -f ${generated[0]} ]]; then
    echo "Expected one opaque_types.rs for ${triple}, found: ${generated[*]}" >&2
    exit 1
  fi
  cp "${generated[0]}" "${out_dir}/opaque_types_${triple}.rs"
  # build.rs also writes the generated C headers into the source tree.
  cp "${work_dir}/zenoh-c/include/zenoh_configure.h" "${out_dir}/zenoh_configure_${triple}.h"
  cp "${work_dir}/zenoh-c/include/zenoh_opaque.h" "${out_dir}/zenoh_opaque_${triple}.h"

  # The other headers are used as committed upstream, so they must match what
  # build.rs generates with these features.
  for header in "${external_dir}"/+ros2_repositories+zenoh_c/include/*.h; do
    if ! cmp -s "${header}" "${work_dir}/zenoh-c/include/$(basename "${header}")"; then
      echo "build.rs output for $(basename "${header}") differs from upstream for ${triple}" >&2
      exit 1
    fi
  done
done
