# Quick validation script for prompt-gen-common package
# Uses the actual CLI validator for comprehensive checking

$packagePath = "D:\workspaces\prompt-gen-spec\prompt-gen-common\prompt-gen-common.yaml"
$testPackagePath = "D:\workspaces\prompt-gen-spec\reference-impl\rpg-desktop\test-packages\prompt-gen-common.yaml"
$cliPath = "D:\workspaces\prompt-gen-spec\reference-impl\rpg-desktop\src-tauri"

Write-Host "Validating prompt-gen-common package..." -ForegroundColor Cyan
Write-Host ""

# Copy to test-packages
Write-Host "Copying to test-packages..." -ForegroundColor Yellow
Copy-Item $packagePath $testPackagePath -Force

# Run the CLI validator
Write-Host "Running CLI validator..." -ForegroundColor Yellow
Write-Host ""

Push-Location $cliPath
cargo run --quiet --bin rpg-cli -- validate "$packagePath" --warnings
Pop-Location

Write-Host ""
Write-Host "Package copied to: $testPackagePath" -ForegroundColor Cyan

