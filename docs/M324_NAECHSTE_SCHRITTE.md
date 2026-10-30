# Eure nächsten Schritte für Modul 324

Dieses Paket ist eine technische Ausgangsbasis. Es ersetzt die Nachweise eurer
eigenen Teamarbeit nicht und ist noch keine fertige Modulabgabe.

## 1. Gemeinsames Repository

Am besten das ursprüngliche Repository verwenden und diese angepassten Dateien
auf einem eigenen Vorbereitungsbranch einbringen. Bestehende Dateien gezielt
ersetzen; die Git-Historie behalten. Alle drei Personen als Collaborators hinzufügen.
Jede Person klont dasselbe gemeinsame Repository und verwendet ihren eigenen
Git-Namen und ihre eigene E-Mail-Adresse auf ihrem Computer.

Bei einem neuen Repository die Herkunft des Codes im README belassen.

## 2. Anforderungen und Zuständigkeiten (A1)

- Issue-Vorlagen für User Stories und Fehler anlegen.
- Labels für Typ und Bearbeitungsstatus festlegen.
- Einen Meilenstein für Version 0.2.0 erstellen.
- Jede Aufgabe einer zuständigen Person zuweisen.
- Akzeptanzkriterien und Testfälle pro User Story erfassen.
- Die Label-Automatisierung aus dem Unterricht einrichten.
- Für Advanced geeignete Werkzeuge zur Anforderungsverwaltung vergleichen.

## 3. Drei kleine echte Erweiterungen

A: Als Benutzer möchte ich bei Division durch null eine verständliche Meldung
sehen, damit ich kein falsches Ergebnis erhalte.
Akzeptanz: API liefert Fehlerstatus und Nachricht; Oberfläche zeigt die Nachricht;
Tests prüfen Division durch null und weiterhin gültige Division.

B: Als Benutzer möchte ich auf fehlende oder ungültige Zahlen hingewiesen werden.
Akzeptanz: Leere Eingabe führt zu einer Meldung; gültige negative Zahlen und
Dezimalzahlen funktionieren; passende Tests dokumentieren das Verhalten.

C: Als Benutzer möchte ich meine Eingaben mit einem Knopf zurücksetzen können.
Akzeptanz: Beide Zahlen und Ergebnis werden zurückgesetzt; das Verhalten des
Speicherwerts wird in der Story ausdrücklich festgelegt und geprüft.

Pro Person: Issue anlegen, Branch erstellen, Änderung und Tests umsetzen,
Pull Request mit `Closes #ISSUE_NUMMER` öffnen. Eine andere Person prüft den PR.
Erst nach erfolgreichen Tests und Review mergen. Die Nummer durch die echte
Issue-Nummer ersetzen. Bei Konflikten gemeinsam sauber auflösen.

## 4. Entwicklungsumgebung (B1)

Alle können Anwendung und Tests selbst starten. Einrichtung dokumentieren.
Lokale pre-commit-Prüfungen entsprechend dem Unterricht ergänzen.

## 5. Versions- und Artefaktverwaltung (C1, 30 Prozent)

Branches, Pull Requests und Reviews als Nachweise behalten.
Versionen sinnvoll benennen. Eine Veröffentlichung mit Tag erstellen.
Automatischen Release und Container-Publishing nach GitHub Packages ergänzen.
Das bereits vorbereitete CI-Artefakt allein ersetzt diese Schritte nicht.

## 6. Continuous Integration (D1)

Die vorbereitete CI nach dem Upload unter Actions überprüfen.
Auch die neuen Anforderungen automatisch testen. Für Advanced einen vollständigen
Integrationstest nachweisen, zum Beispiel Anfrage, Speicherung und erneutes Lesen.
Die vorhandenen API-Tests bilden eine einfache Basis.

## 7. Continuous Deployment (E1)

Gemeinsames Hostingziel bestimmen, z.B. gemäss Azure-Unterrichtsvorgaben.
Zugangsdaten als Secrets konfigurieren. Nach erfolgreichen Prüfungen auf main
automatisch deployen. Öffentliche URL, erfolgreichen Lauf und Fehlerfall nachweisen.
Ein Docker-Build oder herunterladbares Artefakt ist noch kein automatisches Deployment.

## 8. Weitere Unterrichtsvorgaben

Falls LA_324_1650 ebenfalls am eigenen Projekt erfüllt werden muss:
Speicherung mit SQLite und PostgreSQL ergänzen. Dieses Startpaket enthält beides
noch nicht. Weitere Theorieaufgaben, Glossar und Erklärungen separat bearbeiten.

## Nachweise

Pro Bereich passende Links/Screenshots und eine kurze eigene Erklärung sammeln.
Festhalten, wer welche Änderung selbst umgesetzt und welchen PR geprüft hat.
