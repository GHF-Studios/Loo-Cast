param(
    [ValidateSet("dev", "fastdev", "release")]
    [Alias("profile")]
    [string]$BuildProfile = "fastdev"
)

$env:BUILD_PROFILE = $BuildProfile

$exePath = Join-Path -Path ".\build\$BuildProfile" -ChildPath "core_engine.exe"
& $exePath