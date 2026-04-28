pub fn print_help() {
    println!(
        "Loo-Cast alpha xtask

Commands:
  help
  clean                      # SDK layer: clean runtime build artifacts only
  build                      # SDK layer: build with locked vendored SDK
  run                        # SDK layer: run launcher from SDK workspace
  debug                      # SDK layer: run game directly from SDK workspace
  contribute                 # SDK layer: commit/push allowed subset to develop (live repo only)
  clean_sdk                  # Admin layer: wipe vendored SDK/toolchain dir (live repo only)
  build_sdk                  # Admin layer: regenerate SDK bundle from live repo (mutating allowed)
  cloc
  gource
  deploy

Flags:
  --clean-sdk | --CLEAN_SDK
      Explicitly wipe build/<profile>/sdk before staging.
  --host-toolchain
      Use host cargo instead of vendored sdk/cargo-home/bin/cargo for this invocation.
  --contribute-apply
      Required for `contribute` to create commit/push (otherwise dry-run).
  --contribute-no-push
      With `--contribute-apply`, commit only; skip push."
    );
}
