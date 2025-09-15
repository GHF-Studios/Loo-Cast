param(
    [ValidateSet("dev","fastdev","release")]
    [Alias("profile")]
    [string]$BuildProfile = "fastdev"
)

########## DEFINE PATHS ##########

$workspaceRoot  = (Get-Location).Path
$targetDir      = "$workspaceRoot\target\$BuildProfile"
$buildTargetDir = "$workspaceRoot\build\$BuildProfile"

$assetCrates = @("core_lib", "core_mod")

########## CLEAN TARGET ##########
if (Test-Path $buildTargetDir) {
    Remove-Item -Path $buildTargetDir -Recurse -Force
}
New-Item -Path $buildTargetDir -ItemType Directory -Force

########## BUILD ##########
cargo build --$BuildProfile

########## COPY ENGINE EXECUTABLES ##########
$engineExe = Get-ChildItem -Path $targetDir -Filter "core_engine.exe" -Recurse
Copy-Item -Path $engineExe.FullName -Destination $buildTargetDir -Force

$engineDll = Get-ChildItem -Path $targetDir -Filter "core_engine.dll" -Recurse
if ($engineDll) {
    Copy-Item -Path $engineDll.FullName -Destination $buildTargetDir -Force
}

$enginePdb = Get-ChildItem -Path $targetDir -Filter "core_engine.pdb" -Recurse
if ($enginePdb) {
    Copy-Item -Path $enginePdb.FullName -Destination $buildTargetDir -Force
}

########## COPY ASSETS ##########
foreach ($crate in $assetCrates) {
    $src = "$workspaceRoot\$crate\assets"
    $dst = "$buildTargetDir\assets\$crate"
    if (Test-Path $src) {
        Copy-Item -Path $src -Destination $dst -Recurse -Force
        Write-Host "Assets from $crate copied successfully."
    }
}
