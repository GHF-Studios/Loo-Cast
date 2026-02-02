#!/usr/bin/env bash

set -e

# Accept profile as first arg, default to fastdev
BUILD_PROFILE="${1:-fastdev}"

case "$BUILD_PROFILE" in
  dev|fastdev|release) ;;
  *)
    echo "Invalid build profile: $BUILD_PROFILE"
    echo "Valid profiles are: dev, fastdev, release"
    exit 1
    ;;
esac

########## DEFINE PATHS ##########

workspace_root="$(pwd)"
target_name="$BUILD_PROFILE"
if [[ "$BUILD_PROFILE" == "release" ]]; then
  cargo_args="--release"
  target_name="release"
elif [[ "$BUILD_PROFILE" == "dev" ]]; then
  cargo_args=""
  target_name="debug"
else
  cargo_args="--profile $BUILD_PROFILE"
fi

target_dir="$workspace_root/target/$target_name"
build_target_dir="$workspace_root/build/$BUILD_PROFILE"

asset_crates=("core_mod" "base_mod")
mod_crates=("base_mod")  # Extend this list with more mods as needed

########## CLEAN TARGET ##########

if [[ -d "$build_target_dir" ]]; then
  rm -rf "$build_target_dir"
fi
mkdir -p "$build_target_dir"

########## BUILD ##########

# Join excludes into one string
exclude_args=()
for crate in "${mod_crates[@]}"; do
  exclude_args+=(--exclude "$crate")
done

extra_features=""
# if [[ "$BUILD_PROFILE" != "release" ]]; then
#   extra_features="--features bevy_dynamic_linking"
# fi

echo "Building main executable..."
cargo +nightly build $cargo_args --workspace "${exclude_args[@]}" $extra_features

# Build mod crates separately
for crate in "${mod_crates[@]}"; do
  echo "Building mod crate: $crate with init_api feature..."
  cargo +nightly build $cargo_args --manifest-path "$crate/Cargo.toml" --features init_api
done

########## COPY ENGINE EXECUTABLES ##########

engine_exe=$(find "$target_dir" -type f -name "core_engine" | head -n 1)
if [[ -n "$engine_exe" ]]; then
  cp "$engine_exe" "$build_target_dir"
fi

engine_debug=$(find "$target_dir" -type f -name "core_engine.debug" | head -n 1)
if [[ -n "$engine_debug" ]]; then
  cp "$engine_debug" "$build_target_dir"
fi

########## COPY MOD .so FILES ##########

for crate in "${mod_crates[@]}"; do
  mod_so=$(find "$target_dir" -type f -name "lib${crate}.so" | head -n 1)
  if [[ -n "$mod_so" ]]; then
    cp "$mod_so" "$build_target_dir"
    echo "Mod $crate copied successfully."
  else
    echo "Warning: Mod $crate not found in $target_dir"
  fi
done

########## COPY ASSETS ##########

for crate in "${asset_crates[@]}"; do
  src="$workspace_root/$crate/assets"
  dst="$build_target_dir/assets/$crate"
  if [[ -d "$src" ]]; then
    mkdir -p "$dst"
    cp -r "$src/"* "$dst"
    echo "Assets from $crate copied successfully."
  fi
done
