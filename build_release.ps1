########## DEFINE PATHS ##########

# Source and build paths
$looCastBuildSourceDir = ".\target\debug"

# Engine paths
$engineProjectDir = ".\spacetime_engine"
$engineBuildTargetDir = ".\build\debug"

# General application data folder
$engineDataSourceDir = "$engineProjectDir\data"
$engineDataTargetDir = "$engineBuildTargetDir\data"

########## PRE-PROCESS SOURCE AND TARGET DIRECTORIES ##########

# Clear and recreate the build target directory
if (Test-Path $engineBuildTargetDir) {
    Remove-Item -Path $engineBuildTargetDir -Recurse -Force
}
New-Item -Path $engineBuildTargetDir -ItemType Directory -Force

########## BUILD LOO CAST ##########

cargo build --release

########## COPY SPACETIME ENGINE FROM SOURCE TO TARGET ##########

# Copy static Rust library
$engineLibFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "libspacetime_engine.rlib" -Recurse
Copy-Item -Path $engineLibFile.FullName -Destination $engineBuildTargetDir -Force

# Copy C-compatible shared library
$engineDllFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "spacetime_engine.dll" -Recurse
Copy-Item -Path $engineDllFile.FullName -Destination $engineBuildTargetDir -Force

# Copy executable
$engineExeFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "spacetime_engine.exe" -Recurse
Copy-Item -Path $engineExeFile.FullName -Destination $engineBuildTargetDir -Force

# Copy program debug database
$enginePdbFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "spacetime_engine.pdb" -Recurse
Copy-Item -Path $enginePdbFile.FullName -Destination $engineBuildTargetDir -Force

########## COPY APPLICATION DATA FROM SOURCE TO TARGET ##########

# Ensure the data source directory exists
if (Test-Path $engineDataSourceDir) {
    # Copy the entire data directory to the target
    Copy-Item -Path $engineDataSourceDir -Destination $engineDataTargetDir -Recurse -Force
    Write-Host "Application data copied successfully."
} else {
    Write-Host "Source data directory does not exist: $engineDataSourceDir"
}
