# Navigate to spacetime_engine and build
Push-Location .\spacetime_engine
cargo build

# Navigate to loo_cast_base_mod and build
Push-Location .\loo_cast_base_mod
cargo build

# Prepare destination path
$destinationPath = Join-Path -Path "..\spacetime_engine\assets\mods" -ChildPath "loo_cast_base_mod"

# Clear existing build artifacts in the destination
if (Test-Path $destinationPath) {
    Remove-Item -Path $destinationPath -Recurse -Force
}

# Copy new build artifacts
$sourcePath = Join-Path -Path .\target -ChildPath "*"
Copy-Item -Path $sourcePath -Destination $destinationPath -Recurse -Force

Pop-Location

# Run spacetime_engine
Push-Location .\spacetime_engine
cargo run
Pop-Location