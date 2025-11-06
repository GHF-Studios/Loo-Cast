Write-Host "Thank you for using the Cargo Registry Exterminator 9000"

$foldersToNuke = @(
    "C:\Users\lesli\.cargo\registry\cache\index.crates.io-1949cf8c6b5b557f",
    "C:\Users\lesli\.cargo\registry\index\index.crates.io-1949cf8c6b5b557f",
    "C:\Users\lesli\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f"
)

foreach ($folder in $foldersToNuke) {
    if (Test-Path $folder) {
        try {
            Remove-Item -Path $folder -Recurse -Force -ErrorAction Stop
            Write-Host "Deleted: $folder"
        } catch {
            Write-Host "Failed to delete: $folder. Reason: $($_.Exception.Message)"
        }
    } else {
        Write-Host "Folder not found (already gone?): $folder"
    }
}
