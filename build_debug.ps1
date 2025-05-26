########## DEFINE PATHS ##########

# Source and build paths
$looCastBuildSourceDir = ".\target\release"

# Engine paths
$engineProjectDir = ".\core_engine"
$engineBuildTargetDir = ".\build\release"

# General application assets folder
$engineAssetsSourceDir = "$engineProjectDir\assets"
$engineAssetsTargetDir = "$engineBuildTargetDir\assets"

########## PRE-PROCESS SOURCE AND TARGET DIRECTORIES ##########

# Clear and recreate the build target directory
if (Test-Path $engineBuildTargetDir) {
    Remove-Item -Path $engineBuildTargetDir -Recurse -Force
}
New-Item -Path $engineBuildTargetDir -ItemType Directory -Force

########## BUILD LOO CAST ##########

cargo build

########## COPY SPACETIME ENGINE FROM SOURCE TO TARGET ##########

# Copy static Rust library
$engineLibFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "libcore_engine.rlib" -Recurse
Copy-Item -Path $engineLibFile.FullName -Destination $engineBuildTargetDir -Force

# Copy C-compatible shared library
$engineDllFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "core_engine.dll" -Recurse
Copy-Item -Path $engineDllFile.FullName -Destination $engineBuildTargetDir -Force

# Copy executable
$engineExeFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "core_engine.exe" -Recurse
Copy-Item -Path $engineExeFile.FullName -Destination $engineBuildTargetDir -Force

# Copy program debug database
$enginePdbFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "core_engine.pdb" -Recurse
Copy-Item -Path $enginePdbFile.FullName -Destination $engineBuildTargetDir -Force

########## COPY APPLICATION ASSETS FROM SOURCE TO TARGET ##########

# Ensure the assets source directory exists
if (Test-Path $engineAssetsSourceDir) {
    # Copy the entire assets directory to the target
    Copy-Item -Path $engineAssetsSourceDir -Destination $engineAssetsTargetDir -Recurse -Force
    Write-Host "Application assets copied successfully."
} else {
    Write-Host "Source assets directory does not exist: $engineAssetsSourceDir"
}
