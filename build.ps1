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
$modCrates   = @("core_mod")   # later you’ll add more mods here

########## CLEAN TARGET ##########
if (Test-Path $buildTargetDir) {
    Remove-Item -Path $buildTargetDir -Recurse -Force
}
New-Item -Path $buildTargetDir -ItemType Directory -Force

########## BUILD ##########
if ($BuildProfile -eq "release") {
    $cargoArgs  = "--release"
    $targetName = "release"
} elseif ($BuildProfile -eq "dev") {
    $cargoArgs  = ""
    $targetName = "debug"
} else {
    $cargoArgs  = "--profile $BuildProfile"
    $targetName = $BuildProfile
}

$targetDir      = "$workspaceRoot\target\$targetName"
$buildTargetDir = "$workspaceRoot\build\$BuildProfile"

cargo build $cargoArgs

########## COPY ENGINE EXECUTABLES ##########
$engineExe = Get-ChildItem -Path $targetDir -Filter "core_engine.exe" -Recurse
Copy-Item -Path $engineExe.FullName -Destination $buildTargetDir -Force

$enginePdb = Get-ChildItem -Path $targetDir -Filter "core_engine.pdb" -Recurse
if ($enginePdb) {
    Copy-Item -Path $enginePdb.FullName -Destination $buildTargetDir -Force
}

########## COPY MOD DLLs ##########
foreach ($crate in $modCrates) {
    $modDll = Get-ChildItem -Path $targetDir -Filter "$crate.dll" -Recurse
    if ($modDll) {
        Copy-Item -Path $modDll.FullName -Destination $buildTargetDir -Force
        Write-Host "Mod $crate copied successfully."
    } else {
        Write-Host "Warning: Mod $crate not found in $targetDir"
    }
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
