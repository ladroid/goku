# Goku Engine

<p align="center">
    <img src="../docs/image/Goku_logo.png" width="400" height="400" />
</p>

![GitHub Workflow Status](https://img.shields.io/github/commit-activity/t/ladroid/goku)
[![GitHub Repo stars](https://img.shields.io/github/stars/ladroid/goku)](https://github.com/ladroid/goku)
[![Documentation](https://docs.rs/imgui-wgpu/badge.svg)](https://lados-organization.gitbook.io/goku/)
![Repo Size](https://img.shields.io/github/repo-size/ladroid/goku)
![License](https://img.shields.io/github/license/ladroid/goku)

[ [Englisch](README.md) | [Deutsch](docs/README.de.md) | [Español](docs/README.es.md) | [Français](docs/README.fr.md) | [日本語](docs/README.ja.md) ]

Goku ist eine 2D-Spieleentwicklungsanwendung für Rust (zukünftig mit Integration von 3D-Spielen). Komplett in Rust geschrieben.

Es ist verfügbar für **macOS**, **Windows** und **Linux**.

Basierend auf SDL2 (derzeit).

Goku ist fokussiert, leichtgewichtig und hat nur wenige Abhängigkeiten (hauptsächlich SDL2). Es bietet:

* Ein Fenster und eine Hauptschleife

* 2D-Grafik und Text

* Geräusche und Musik

* Tastatur-, Maus- und Gamepad-Eingabe

* GUI-Oberfläche für die Entwicklung

* Pixel-Charaktergenerator mit KI

<ins>Drittanbieter-Bibliotheken, die derzeit Goku verwenden:</ins>

* SDL2

* nalgebra

* imgui

* serde

* rfd

* glow

* webbrowser

## Verwendung

**WICHTIG!!!**

* Eine kurze Übersicht befindet sich hier -> [Goku Engine](https://gokuengine.com/)
* Die Dokumentation befindet sich hier -> [Gitbook](https://lados-organization.gitbook.io/goku/)
* Ein Entwicklungsblog, der unseren Fortschritt, Pläne und neue Funktionen abdeckt, befindet sich hier -> [News](https://gokuengine.com/news)
* Alle Beispiele befinden sich hier -> [Offizielle Beispiele](https://github.com/ladroid/goku/tree/main/examples)

## Anforderungen
### Linux
Installieren Sie diese über Ihr bevorzugtes Paketverwaltungstool oder über
http://www.libsdl.org/

**Ubuntu-Beispiel:**
> sudo apt-get install libsdl2-dev

**Fedora-Beispiel:**
> sudo dnf install SDL2-devel

**Arch-Beispiel:**
(Arch hat keine separaten regulären und Entwicklungs-Pakete, alles geht zusammen.)
> sudo pacman -S sdl2

Sie benötigen möglicherweise auch einen C-Compiler (`gcc`).

#### Statische Verlinkung in Linux

Sie können wählen, SDL2 statisch anstatt dynamisch mit dem Feature `static-link` zu verlinken.
Unter Linux müssen Sie zusätzlich eine der folgenden Aktionen ausführen:
* das Feature `bundled` verwenden
* das Feature `use-pkgconfig` verwenden, damit rustc weiß, wo es nach Ihren SDL2-Bibliotheken und deren Abhängigkeiten für die statische Verlinkung suchen soll. Dies ist erforderlich, da es keine integrierte Möglichkeit gibt, die benötigten Ressourcen zu finden, um SDL2 statisch von Ihrem System zu verlinken
* Entwicklungsbibliotheken mit [vcpkg][vcpkg] installieren. Anweisungen zur Erstellung einer statischen Binärdatei unter Linux und anderen Betriebssystemen mit vcpkg finden Sie [hier][cargo-vcpkg-usage]

### macOS
#### Homebrew
Auf macOS ist es eine gute Idee, diese über
[homebrew][homebrew] zu installieren.

```
brew install sdl2
```

In neueren Versionen von Homebrew werden die installierten Bibliotheken normalerweise in `$(brew --prefix)/lib` verlinkt.
Wenn Sie eine ältere Version verwenden, befindet sich der Symlink für SDL möglicherweise in `/usr/local/lib`.

Um das Verlinken von Bibliotheken, die von Homebrew installiert wurden, zu erleichtern, führen Sie die folgenden Schritte für Ihre jeweilige Shell aus.

Fügen Sie diese Zeile Ihrer `~/.zshenv` oder `~/.bash_profile` hinzu, je nachdem, ob Sie ZSH oder Bash verwenden.
```
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

### Windows (MSVC)

1. Laden Sie die MSVC-Entwicklungsbibliotheken von http://www.libsdl.org/ herunter (SDL2-devel-2.0.x-VC.zip).
2. Entpacken Sie SDL2-devel-2.0.x-VC.zip in einen Ordner Ihrer Wahl (Sie können ihn danach löschen).
3. Kopieren Sie alle lib-Dateien von
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    nach (für Rust 1.6 und höher)
    > C:\Program Files\Rust\\**lib**\rustlib\x86_64-pc-windows-msvc\lib

    oder nach (für Rust-Versionen 1.5 und niedriger)
    > C:\Program Files\Rust\\**bin**\rustlib\x86_64-pc-windows-msvc\lib

    oder in Ihren Bibliotheksordner Ihrer Wahl, und stellen Sie sicher, dass Sie eine Systemumgebungsvariable haben
    > LIB = C:\your\rust\library\folder

    Für Rustup-Benutzer wird dieser Ordner sich in
    > C:\Users\\{Your Username}\\.rustup\toolchains\\{current toolchain}\lib\rustlib\\{current toolchain}\lib

    befinden, wobei die aktuelle Toolchain wahrscheinlich `stable-x86_64-pc-windows-msvc` ist.

4. Kopieren Sie SDL2.dll von
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    in Ihr Cargo-Projekt, direkt neben Ihre Cargo.toml.

 5. Wenn Sie Ihr Spiel versenden, stellen Sie sicher, dass Sie SDL2.dll in dasselbe Verzeichnis kopieren, in dem sich Ihre kompilierte exe befindet, sonst wird das Spiel nicht starten.

#### Statische Verlinkung mit MSVC

Die von http://libsdl.org/ bereitgestellten MSVC-Entwicklungsbibliotheken enthalten keine statische Bibliothek. Das bedeutet, wenn Sie das Feature `static-link` mit der windows-msvc-Toolchain verwenden möchten, müssen Sie eine der folgenden Aktionen ausführen:

- Eine statische SDL2-Bibliothek selbst erstellen und in das `lib`-Verzeichnis Ihrer Toolchain kopieren; oder
- Auch das Feature `bundled` aktivieren, das für Sie eine statische Bibliothek erstellt; oder
- eine statische SDL2-Bibliothek von vcpkg verwenden, wie unten beschrieben.

## Merkmale

* Grafik:
    * ein Fenster und eine Hauptschleife

    * 2D-Grafiken und Text 
        - Button 
        - Schieberegler 
        - Kontrollkästchen
        - Textfeld
        - Schriftarten (nur ttf-Format)
        - Partikelsystem (Funken)
        - Parallax-Hintergrund

    * GUI-Schnittstelle für die Entwicklung
        * integrierter Texteditor
        * Debug-Konsole

    * Mehrere Bildformate: JPG und PNG

    * Beleuchtung:
        - Punktlicht
        - Spotlicht
        - Umgebungslichtfilter

* Audio
    - Abspielen
    - Schleife
    - Pause
    - Fortsetzen
    - Mehrere Audioformate: OGG, MP3, FLAC, MOD

* Eingabehandler:
    * Tastatur-, Maus- und Gamepad-E

ingabe

* Mathematische Typen:
    * Vektor2, Vektor3, Vektor4
    * Matrix33, Matrix34, Matrix43, Matrix44

* Physik:
    * Kollisionen
    * Starrkörper (derzeit haben wir keinen kinematischen Starrkörper)

* Szene:
    * Flexibles JSON-Dateiformat: Kann entweder eine ganze Szene oder einzelne Meshes beschreiben.

* Animation

* KI-System:
    * Verhaltensbaum

* Timer

* Dialogsystem

* Profiler

* Unterstützung für VSCode

* Unterstützt mehrere Sprachen:
    - Deutsch
    - Spanisch
    - Japanisch
    - Französisch

* Unterstützte Plattformen:
    - Windows / Mac / Linux
    - Web (WASM noch nicht vollständig integriert) (zusätzliche Referenzen [Emscripte](https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/) / [SDL-WASM](https://gitlab.com/ThibaultLemaire/rust-sdl-canvas-wasm) / [Web](https://github.com/koute/cargo-web) )
    - Android in der Zukunft

## Ausführung

1. ```git clone https://github.com/ladroid/goku.git```
2. Alles extrahieren
3. Befehl zum Ausführen: `cargo run`

**Wichtig!!!** Derzeit befindet sich die GUI noch in der Entwicklung, ich versuche, imgui und sdl2 zusammenzubringen, benötige aber einige Zeit, um sie vollständig kompatibel zu machen. Wenn also jemand es verwenden möchte, ist es möglich, die Komponente `Scene` hinzuzufügen und dort ein Skript zu schreiben. Wenn Sie wissen, wie man sie kombiniert, wäre das wirklich großartig!

## Erstellung für das Web

1. Werkzeuge drücken
2. Bauen drücken und Web auswählen

## Wie man den Viewport aktiviert

Um den Viewport zu aktivieren, gehen Sie zu Einstellungen -> Allgemein -> Canvas aktivieren, das gleiche gilt für den Gitteransichtsmodus

## TODO (priorisiert)

* ~~endlich eine separate .rs-Datei erstellen statt einer großen~~  

* ~~einen Viewport erstellen statt der aktuellen Lösung mit Canvas (wahrscheinlich ein separates Fenster in der App mit Kombination aus sdl2 und OpenGL)~~

* mit wgpu kombinieren ([Beispiel aus der sdl2-Bibliothek](https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/raw-window-handle-with-wgpu/main.rs) und [imgui-Renderer für wgpu-rs](https://github.com/Yatekii/imgui-wgpu-rs))

* Physik verbessern

* UI-System hinzufügen/verbessern (Bild für Schaltflächen hinzufügen)

* ~~Lichter und Schatten verbessern~~

* einfache Formen zeichnen (Kreis, Rechteck, Dreieck usw.)

* ~~Tabs~~

* ~~Profiler verbessern~~

* ~~Partikelsystem hinzufügen/verbessern~~

* ~~GUI der Engine sowie Texteditor verbessern (wahrscheinlich statt eingebautem Texteditor Integration mit VSCode oder anderem IDE)~~

* Blueprints hinzufügen (wahrscheinlich imgui-Node-Graph https://github.com/benmkw/imnodes-rs)

* Spiele für Mobilgeräte (iOS, Android) bauen

* Spiele für Konsolen (PS4-5), Xbox, Nintendo Switch) bauen

* Physikmaterial

* Integration mit C++ (wahrscheinlich etwas wie ein Bindgen)

## Beispiele

### 1. Tetris

Ein Beispiel für den Bau eines Tetris-Spiels finden Sie [hier](../examples/tetris_game_example.rs)

### 2. Roguelike-Prototyp (TODO)

Ein Beispiel für den Bau eines Roguelike-Prototyps finden Sie hier -> https://github.com/ladroid

### 3. Visuelle Effekte

1. Funken -> verwenden Sie die Funktion `spawn_particles_sparks` [von hier](src/two_d/particle_system.rs)
2. Feuer -> verwenden Sie die Funktion `spawn_particles_fires` [von hier](src/two_d/particle_system.rs)
3. Regen -> verwenden Sie die Funktion `spawn_particles_rain` [von hier](src/two_d/particle_system.rs)

### 4. Seitlich scrollendes Spiel

Ein Beispiel für den Bau eines seitlich scrollenden Prototyps finden Sie [hier](../examples/simple_parallax_example.rs)

### 5. Platformer

Beispiel für den Bau eines Side-Scrolling-Prototyps finden Sie [hier](../examples/simple_parallax_example.rs)

### 6. Einfache Zustände für Feinde festlegen (Verfolgung/Folgen)

Ein Beispiel für den Bau eines Platformer-Prototyps finden Sie [hier](../examples/enemy_behaviour.rs)

## Japanische Version

Die japanische Version finden Sie [hier](https://lados-organization.gitbook.io/goku/v/goku-game-engine_jp/)

## Französische Version

Die französische Version finden Sie [hier](https://lados-organization.gitbook.io/goku/v/goku-game-engine_fr/)

## Deutsche Version

Die deutsche Version finden Sie [hier](https://lados-organization.gitbook.io/goku/v/goku-game-engine_de/)

## Spanische Version

Die spanische Version finden Sie [hier](https://lados-organization.gitbook.io/goku/v/goku-game-engine_es/)

## Beitrag leisten

### Fehler melden
Verwenden Sie den Issue Tracker, um Fehlerberichte und Anfragen für Funktionen/Verbesserungen einzureichen. Stellen Sie vor der Einreichung eines neuen Issues sicher, dass es kein ähnliches offenes Issue gibt.

### Manuelle Tests
Jeder, der den Code manuell testet und Fehler meldet oder Vorschläge für Verbesserungen im Issue Tracker macht, ist sehr willkommen!

### Pull-Anfragen einreichen
Patches/Fixes werden in Form von Pull-Anfragen (PRs) akzeptiert. Stellen Sie sicher, dass das Problem, das die Pull-Anfrage behandelt, im Issue Tracker offen ist.

Die eingereichte Pull-Anfrage wird als Zustimmung zur Veröffentlichung unter der Apache 2.0-Lizenz betrachtet.

## Community

[Discord](https://discord.gg/9TAMqdRyED)

[GitHub-Diskussion](https://docs.github.com/en/discussions/quickstart)

## Lizenz
Goku ist unter der Apache-Lizenz Version 2.0 lizenziert. Siehe [LIZENZ](https://pages.github.com/) Datei.