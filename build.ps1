param(
    [ValidateSet("dev","fastdev","release")]
    [Alias("profile")]
    [string]$BuildProfile = "fastdev"
)

########## DEFINE PATHS ##########

$workspaceRoot  = (Get-Location).Path
$targetDir      = "$workspaceRoot\target\$BuildProfile"
$buildTargetDir = "$workspaceRoot\build\$BuildProfile"

$assetCrates = @("core_mod", "base_mod")
$modCrates   = @("base_mod")   # later you’ll add more mods here

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

# Join excludes into one string
$excludeArgs = $modCrates | ForEach-Object { "--exclude $_" } | Out-String
$excludeArgs = $excludeArgs -replace "\r?\n", " "  # flatten line breaks
$extraFeatures = ""
# if ($BuildProfile -ne "release") {
    # $extraFeatures = "--features bevy_dynamic_linking"
# }

# Build the non-mod part of the workspace
Write-Host "Building main executable..."
Invoke-Expression "cargo +nightly build $cargoArgs --workspace $excludeArgs $extraFeatures"

# Build mods separately with required features
foreach ($crate in $modCrates) {
    Write-Host "Building mod crate: $crate with init_api feature..."
    cargo +nightly build $cargoArgs --manifest-path "$crate/Cargo.toml" --features init_api
}

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
