#!/usr/bin/env bash

set -e

BUILD_PROFILE="${1:-fastdev}"

case "$BUILD_PROFILE" in
  dev|fastdev|release) ;;
  *)
    echo "Invalid build profile: $BUILD_PROFILE"
    echo "Valid profiles are: dev, fastdev, release"
    exit 1
    ;;
esac

export RUST_BACKTRACE=1
# export RUST_MIN_STACK=335544320

export BUILD_PROFILE="$BUILD_PROFILE"

exe_path="./build/$BUILD_PROFILE/core_engine"
if [[ ! -x "$exe_path" ]]; then
  echo "Executable not found at $exe_path"
  exit 1
fi

# Add dynamic lib path for dev builds
if [[ "$BUILD_PROFILE" == "dev" || "$BUILD_PROFILE" == "fastdev" ]]; then
  export LD_LIBRARY_PATH="$(pwd)/target/debug/deps:$(rustc --print target-libdir):$LD_LIBRARY_PATH"
fi

"$exe_path"
