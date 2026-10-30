# Prüfstatus des Startpakets

Erfolgreich lokal geprüft:
- cargo fmt --all --check
- cargo test -p rust-rechner-backend: 2 Tests bestanden
- cargo check -p frontend --target wasm32-unknown-unknown
- trunk build --release --locked: Oberfläche vollständig gebaut
- cargo build --locked -p rust-rechner-backend
- HTTP-Test: HTML, Health-Endpunkt, negative Dezimalmultiplikation und
  eingebundene statische Ressourcen erfolgreich ausgeliefert.

Nicht hier ausgeführt:
- Interaktiver Browser-Test und visuelle Prüfung: Testbrowser nicht verfügbar.
- Docker-Build und Compose-Start: Docker nicht verfügbar.
- Windows-Startskript: kein Windows verfügbar.
- GitHub Actions und Hosting-Deployment: noch nicht auf GitHub eingerichtet.

Das Paket ist vorbereitet und lokal gebaut; die oben genannten offenen
Umgebungsprüfungen gehören zum nächsten gemeinsamen Einrichtungsschritt.
