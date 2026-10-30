$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "Rust fehlt. Bitte Rust über https://rustup.rs installieren und das Terminal neu öffnen."
}
if (-not (Get-Command trunk -ErrorAction SilentlyContinue)) {
    throw "Trunk fehlt. Bitte zuerst ausführen: cargo install trunk --version 0.21.14 --locked"
}
Push-Location Frontend
try {
    trunk build --locked
    if ($LASTEXITCODE -ne 0) { throw "Frontend-Build fehlgeschlagen." }
} finally { Pop-Location }
Write-Host "Öffne im Browser: http://localhost:3000"
cargo run --locked -p rust-rechner-backend
if ($LASTEXITCODE -ne 0) { throw "Backend konnte nicht gestartet werden." }
