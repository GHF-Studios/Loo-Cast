param(
    [ValidateSet("dev","fastdev","release")]
    [Alias("profile")]
    [string]$BuildProfile = "fastdev"
)

Push-Location ".\build\$BuildProfile"
.\core_engine.exe
Pop-Location
