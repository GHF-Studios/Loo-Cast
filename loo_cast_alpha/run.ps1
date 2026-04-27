param(
    [ValidateSet("dev", "fastdev", "release")]
    [Alias("profile")]
    [string]$BuildProfile = "fastdev"
)

$env:RUST_BACKTRACE = "1"
# $env:RUST_MIN_STACK = 335544320
$env:BUILD_PROFILE = $BuildProfile

$exePath = Join-Path -Path ".\build\$BuildProfile" -ChildPath "core_engine.exe"

if (-Not (Test-Path $exePath)) {
    Write-Error "Executable not found at $exePath"
    exit 1
}

# Add dynamic lib path for dev builds
if ($BuildProfile -in @("dev", "fastdev")) {
    $depsPath = Resolve-Path ".\target\debug\deps"
    $rustLib = & rustc --print target-libdir
    $env:PATH = "$($depsPath);$($rustLib);$($env:PATH)"
}

& $exePath
