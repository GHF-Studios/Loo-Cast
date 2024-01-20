# Define paths
$engineBuildSourceDir = ".\spacetime_engine\target\release"
$engineBuildDataSourceDir = ".\spacetime_engine\data"
$engineBuildResourcesSourceDir = ".\spacetime_engine\resources"
$engineBuildTargetDir = ".\build"
$engineBuildDataTargetDir = ".\build\data"
$engineBuildResourcesTargetDir = ".\build\resources"

$baseModBuildSourceDir = ".\loo_cast_base_mod\target\release"
$baseModBuildDataSourceDir = ".\loo_cast_base_mod\data"
$baseModBuildResourcesSourceDir = ".\loo_cast_base_mod\resources"
$baseModBuildTargetDir = ".\build\resources\mods\loo_cast_base_mod"
$baseModBuildDataTargetDir = ".\build\resources\mods\loo_cast_base_mod\data"
$baseModBuildResourcesTargetDir = ".\build\resources\mods\loo_cast_base_mod\resources"

# TODO: Change bevy asset folder to "resources"
# TODO: Set persistent data folder to "data"
# TODO: Solve bevy not being able to specify multiple asset folders

# Clean build directory
if (Test-Path $buildDir) {
    Remove-Item -Path $buildDir -Recurse -Force
}

# Create necessary directories in build
New-Item -Path $buildDir\assets -ItemType Directory -Force
New-Item -Path $baseModTargetDir -ItemType Directory -Force

# Build spacetime_engine
Push-Location $engineDir
cargo build --release
Pop-Location

# Copy engine executable and assets to build directory
Copy-Item -Path "$engineDir\target\release\*" -Destination $buildDir -Recurse -Force
Copy-Item -Path $assetsDir -Destination "$buildDir\assets" -Recurse -Force

# Build and copy loo_cast_base_mod
Push-Location $baseModDir
cargo build --release
Copy-Item -Path ".\target\release\*" -Destination $baseModTargetDir -Recurse -Force
Pop-Location

# Copy other mods (if any) to build directory (excluding base mod)
Get-ChildItem -Path $modsDir -Directory -Exclude "loo_cast_base_mod" | ForEach-Object {
    Copy-Item -Path $_.FullName -Destination "$buildDir\assets\mods\$($_.Name)" -Recurse -Force
}

# Run spacetime_engine from the build directory
Push-Location $buildDir
.\spacetime_engine.exe
Pop-Location