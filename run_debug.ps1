########## DEFINE PATHS ##########

# Loo cast paths
$looCastBuildSourceDir = ".\target\debug"

# Engine paths
$engineProjectDir = ".\spacetime_engine"
$engineBuildTargetDir = ".\build\debug"

$engineBuildModsSourceDir = "$engineProjectDir\mods"
$engineBuildModsTargetDir = "$engineBuildTargetDir\mods"

# Base mod paths
$baseModProjectDir = ".\loo_cast_base_mod"
$baseModBuildTargetDir = "$engineBuildModsTargetDir\loo_cast_base_mod"

$baseModBuildDataSourceDir = "$baseModProjectDir\data"
$baseModBuildDataTargetDir = "$baseModBuildTargetDir\data"

$baseModBuildResourcesSourceDir = "$baseModProjectDir\resources"
$baseModBuildResourcesTargetDir = "$baseModBuildTargetDir\resources"




########## PRE-PROCESS SOURCE AND TARGET DIRECTORIES ##########

if (Test-Path $engineBuildTargetDir) {
    Remove-Item -Path $engineBuildTargetDir -Recurse -Force
}
New-Item -Path $engineBuildTargetDir -ItemType Directory -Force

if (Test-Path $baseModBuildTargetDir) {
    Remove-Item -Path $baseModBuildTargetDir -Recurse -Force
}
New-Item -Path $baseModBuildTargetDir -ItemType Directory -Force




########## BUILD LOO CAST ##########

cargo build




########## COPY SPACETIME ENGINE FROM SOURCE TO TARGET ##########

# Copy static rust library from build source to build target
$engineLibFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "libspacetime_engine.rlib" -Recurse
Copy-Item -Path $engineLibFile.FullName -Destination $engineBuildTargetDir -Force

# Copy C-compatible shared library from build source to build target
$engineDllFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "spacetime_engine.dll" -Recurse
Copy-Item -Path $engineDllFile.FullName -Destination $engineBuildTargetDir -Force

# Copy executable from build source to build target
$engineExeFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "spacetime_engine.exe" -Recurse
Copy-Item -Path $engineExeFile.FullName -Destination $engineBuildTargetDir -Force

# Copy program debug database from build source to build target
$enginePdbFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "spacetime_engine.pdb" -Recurse
Copy-Item -Path $enginePdbFile.FullName -Destination $engineBuildTargetDir -Force




########## COPY LOO CAST BASE MOD FROM SOURCE TO TARGET ##########

# Copy rust library from build source to build target
$baseModLibFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "loo_cast_base_mod.dll" -Recurse
Copy-Item -Path $baseModLibFile.FullName -Destination $baseModBuildTargetDir -Force

# Copy program debug database from build source to build target
$baseModPdbFile = Get-ChildItem -Path $looCastBuildSourceDir -Filter "loo_cast_base_mod.pdb" -Recurse
Copy-Item -Path $baseModPdbFile.FullName -Destination $baseModBuildTargetDir -Force

# Copy data directory from build source to build target
Copy-Item -Path $baseModBuildDataSourceDir -Destination $baseModBuildDataTargetDir -Recurse -Force

# Copy resources directory from build source to build target
Copy-Item -Path $baseModBuildResourcesSourceDir -Destination $baseModBuildResourcesTargetDir -Recurse -Force




########## COPY OTHER PRE-INCLUDED MODS FROM SOURCE TO TARGET ##########

# Ensure the mods source directory exists
if (Test-Path $engineBuildModsSourceDir) {
    # Get all mod directories in the source directory
    $modDirectories = Get-ChildItem -Path $engineBuildModsSourceDir -Directory

    # Iterate over each mod directory
    foreach ($modDir in $modDirectories) {
        # Define the target directory path for each mod
        $targetModDir = Join-Path -Path $engineBuildModsTargetDir -ChildPath $modDir.Name

        # Check for duplicate mod directories
        if (Test-Path $targetModDir) {
            Write-Host "Attempted to load mod '$($modDir.Name)' multiple times."
        } else {
            # Copy the entire mod directory to the target directory
            Copy-Item -Path $modDir.FullName -Destination $targetModDir -Recurse -Force
        }
    }
} else {
    Write-Host "Source mods directory does not exist: $engineBuildModsSourceDir"
}




########## RUN LOO CAST ##########

cd .\build\debug\

.\spacetime_engine.exe

cd ..\..