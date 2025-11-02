param(
    [ValidateSet("dev", "fastdev", "release")]
    [Alias("profile")]
    [string]$BuildProfile = "fastdev"
)

$env:RUST_LIB_BACKTRACE = "1"
# $env:RUST_MIN_STACK = 335544320

$env:BUILD_PROFILE = $BuildProfile

$exePath = Join-Path -Path ".\build\$BuildProfile" -ChildPath "core_engine.exe"
& $exePath