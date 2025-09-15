param(
    [ValidateSet("dev","fastdev","release")]
    [Alias("profile")]
    [string]$BuildProfile = "fastdev"
)

$env:BUILD_PROFILE = $BuildProfile

Push-Location ".\build\$BuildProfile"
.\core_engine.exe
Pop-Location
