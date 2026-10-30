# Rust Taschenrechner – Grundlage für Modul 324

Ein kleiner Taschenrechner mit Rust, Axum und Yew. Er rechnet mit zwei Zahlen
und bietet einen Speicherwert mit Lesen, Speichern und Löschen.

Grundlage: https://github.com/erdbeerschoggi/Lernperiode-9
Der ursprüngliche Taschenrechner wurde für dieses Startpaket angepasst.
Der ursprüngliche Lernbericht ist unter docs/Original_README.md enthalten.

## Schnellstart mit Docker Desktop

Docker Desktop starten. ZIP entpacken und im Projektordner ein Terminal öffnen:

```powershell
docker compose up --build
```

Danach http://localhost:3000 öffnen. Beide Teile werden gemeinsam ausgeliefert.
Beenden mit Strg+C; Container entfernen mit `docker compose down`.
Der erste Build dauert wegen der Rust- und WebAssembly-Werkzeuge länger.

## Mit der enthaltenen Oberfläche starten

Das Paket enthält bereits die gebaute Oberfläche unter `Frontend/dist`.
Nach der Rust-Installation reicht im Hauptordner:

```powershell
cargo run --locked -p rust-rechner-backend
```

Danach http://localhost:3000 öffnen. Änderungen am Frontend müssen mit Trunk
neu gebaut werden. `Frontend/dist` ist ein Build-Ergebnis und wird von Git ignoriert.

## Start ohne Docker unter Windows

1. Rust über https://rustup.rs installieren. Falls angefordert, die Microsoft
   C++ Build Tools installieren. Terminal danach neu öffnen.
2. Im Projektordner einmal installieren:

```powershell
cargo install trunk --version 0.21.14 --locked
```

3. Im Projektordner starten:

```powershell
powershell -ExecutionPolicy Bypass -File .\Start.ps1
```

Das Skript baut die Oberfläche und startet den Server. Anschliessend
http://localhost:3000 öffnen. Zum Beenden Strg+C drücken.
Rust lädt beim ersten Start die im Projekt hinterlegten Komponenten nach.

## Während der Entwicklung: zwei Terminals

Terminal 1 im Hauptordner:

```powershell
cargo run --locked -p rust-rechner-backend
```

Terminal 2 im Hauptordner:

```powershell
cd Frontend
trunk serve
```

Dann http://localhost:8080 verwenden. Trunk leitet `/api/` an das Backend weiter.
Nach Frontend-Änderungen wird diese Entwicklungsansicht automatisch neu gebaut.

## Prüfen

```powershell
cargo test --locked -p rust-rechner-backend
cargo fmt --all --check
```

Vor dem Commit bei Formatierungsabweichungen `cargo fmt --all` ausführen.
Die vorhandenen Tests prüfen die vier Grundrechenarten über die API sowie
Speichern, Lesen und Löschen eines Speicherwertes.

## Aufbau

- `Backend/src/main.rs`: API, Rechenlogik, Speicher und API-Tests.
- `Frontend/src/main.rs`: Oberfläche und HTTP-Anfragen.
- `Frontend/index.html`: Einstieg für den WebAssembly-Build.
- `Frontend/Trunk.toml`: Frontend-Build und lokaler API-Proxy.
- `.github/workflows/ci.yml`: Tests, Builds und Build-Artefakt auf GitHub.
- `Dockerfile` und `compose.yaml`: gemeinsam startbare Anwendung.
- `docs/M324_NAECHSTE_SCHRITTE.md`: offene Aufgaben für eure Teamarbeit.

## Bewusst kleiner Funktionsumfang

Der Speicher ist gemeinsam für alle Besucher und wird beim Serverneustart gelöscht.
Es gibt keine Benutzerkonten und keine Datenbank.
Division durch null und unbekannte Operatoren liefern im Ausgangscode noch 0.
Leere Zahleneingaben werden im Ausgangscode noch als 0 behandelt.
Diese fachlichen Verbesserungen sind als echte Teamaufgaben offen.
Die Oberfläche meldet bereits fehlgeschlagene Serveranfragen.

## Auslieferung

Standardport: 3000. `PORT` kann für einen Hosting-Anbieter gesetzt werden.
`STATIC_DIR` bezeichnet den Ordner mit dem gebauten Frontend (Standard:
`Frontend/dist`, relativ zum aktuellen Arbeitsverzeichnis).
Der Server stellt Oberfläche und API unter derselben Adresse bereit.

Die GitHub-Automatisierung läuft erst nach dem Hochladen auf GitHub.
Sie baut und testet, deployt aber noch nicht automatisch auf einen Server.
Hosting, Secrets und Deployment richtet ihr im Modul als eigenen Schritt ein.
