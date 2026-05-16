# Pre-commit check script for SynthPlayer
# Usage: .\check.ps1
# Place as .git/hooks/pre-commit for automatic execution

$exit = 0

function step($name, $cmd) {
    Write-Host "`n[$name]..." -ForegroundColor Yellow -NoNewline
    $out = Invoke-Expression "$cmd 2>&1" -ErrorAction SilentlyContinue
    if ($LASTEXITCODE -ne 0) {
        Write-Host " FAILED" -ForegroundColor Red
        Write-Host $out
        $script:exit = 1
    } else {
        Write-Host " OK" -ForegroundColor Green
    }
}

Write-Host "=== SynthPlayer pre-commit check ===" -ForegroundColor Cyan

step "cargo check  " "cargo check"
step "cargo test   " "cargo test"
step "cargo clippy " "cargo clippy -- -D warnings"

Write-Host "`n==============================" -ForegroundColor Cyan
if ($exit -eq 0) {
    Write-Host "All checks passed" -ForegroundColor Green
} else {
    Write-Host "Some checks FAILED - fix before commit" -ForegroundColor Red
}
exit $exit
