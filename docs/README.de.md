# goku

[ [Englisch](../README.md) | [Deutsch](README.de.md) | [Spanisch](README.es.md) | [Französisch](README.fr.md) | [日本語](README.ja.md) ]


goku ist eine 2D-Spieleentwicklungsanwendung für Rust (in Zukunft auch für die Integration von 3D-Spielen). Geschrieben ausschließlich in Rust.

Es ist verfügbar für **macOS**, **Windows** und **Linux**.

Basierend auf SDL2 (derzeit).

goku ist fokussiert, leichtgewichtig und hat nur wenige Abhängigkeiten (hauptsächlich SDL2). Es bietet:

* ein Fenster und eine Hauptschleife

* 2D-Grafiken und Text

* Klänge und Musik

* Tastatur-, Maus- und Gamepad-Eingabe

* GUI-Schnittstelle für die Entwicklung

<ins>Von goku derzeit verwendete Drittbibliotheken:</ins>

* SDL2

* nalgebra

* imgui

* serde

* rfd

* walkdir

## Verwendung

Die Dokumentation befindet sich hier -> [Gitbook](https://lados-organization.gitbook.io/goku/)

## Funktionen

* Grafiken:
    * ein Fenster und eine Hauptschleife

    * 2D-Grafiken und Text
        - Button
        - Schieberegler
        - Kontrollkästchen
        - Textfeld
        - Schriftarten (nur im ttf-Format)
        - Partikelsystem (Funken)
        - Parallax-Hintergrund

    * GUI-Schnittstelle für die Entwicklung
        * integrierter Texteditor
        * Debug-Konsole

    * Unterstützte Bildformate: JPG und PNG

* Audio
    - Wiedergabe
    - Schleife
    - Pause
    - Fortsetzen
    - Unterstützte Audioformate: OGG, MP3, FLAC, MOD

* Eingabeverarbeitung:
    * Tastatur-, Maus- und Gamepad-Eingabe

* Mathematische Typen:
    * Vector2, Vector3, Vector4
    * Matrix33, Matrix34, Matrix43, Matrix44

* Physik:
    * Kollisionen
    * Starrkörper (derzeit kein kinematischer Starrkörper)

* Szene:
    * Flexibles JSON-Dateiformat: Kann entweder eine gesamte Szene oder einzelne Meshes beschreiben.

* Animation

* KI-System:
    * Verhaltensbaum

* Timer

* Dialogsystem

* Unterstützte Sprachen:
    - Deutsch
    - Spanisch
    - Japanisch
    - Französisch

* Unterstützte Plattformen:
    - Windows / Mac / Linux
    - Web (WASM noch nicht vollständig integriert) (zusätzliche Verweise [Emscripten](https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/) / [SDL-WASM](https://gitlab.com/ThibaultLemaire/rust-sdl-canvas-wasm) / [Web](https://github.com/koute/cargo-web))
    - Android in der Zukunft

## Ausführung

Befehl zur Ausführung: `cargo run`

**Wichtig!!!** Die GUI befindet sich derzeit noch in der Entwicklung. Ich versuche, imgui und sdl2 miteinander zu kombinieren, aber es wird noch einige Zeit dauern, bis es vollständig kompatibel ist. Wenn also jemand die Anwendung nutzen möchte, kann er das Komponente `Scene` hinzufügen und dort ein Skript schreiben. Wenn Sie wissen, wie sie kombiniert werden können, wäre das wirklich großartig!

## Kompilierung für das Web

1. Drücken Sie "Tools"
2. Drücken Sie "Build"
3. Führen Sie diesen Befehl im Verzeichnis aus, in dem er erstellt wurde: `cargo web start wasm32-unknown-emscripten` oder `cargo web build --target wasm32-unknown-emscripten`

## TODO

* Verbessern der GUI des Motors sowie des Texteditors

* Zeichnen einfacher Formen (Kreis, Rechteck, Dreieck, usw.)

* Registerkarten

* Profiler

* Ansichtsfenster (wahrscheinlich eine Kombination aus sdl2 und egui)

* Physikmaterial

* Möglichkeit zur Erstellung von 3D-Spielen (Betrachtung von Vulkan)

* Verbessern von Lichtern und Schatten

* Hinzufügen/Verbessern des Partikelsystems

* Verbessern der Physik (Hinzufügen eines kinematischen Starrkörpers)

* Hinzufügen/Verbessern des UI-Systems

* Voxel

* Hinzufügen von Bauplänen (wahrscheinlich mit imgui Node-Graph https://github.com/benmkw/imnodes-rs)

* Spieleentwicklung für mobile iOS, Android

* Spieleentwicklung für Konsolen (PS4-5), Xbox, Nintendo Switch

* Integration mit C++ (wahrscheinlich etwas wie Bindgen)

## Beispiele

### 1. Tetris

Ein Beispiel für den Aufbau eines Tetris-Spiels finden Sie hier -> https://github.com/ladroid

### 2. Roguelike-Prototyp (TODO)

Ein Beispiel für den Aufbau eines Roguelike-Prototyps finden Sie hier -> https://github.com/ladroid

### 3. Visuelle Effekte

1. Funken -> 
2. Feuer -> 
3. Regen -> 

### 4. Seitwärts scrollendes Spiel

Ein Beispiel für den Aufbau eines seitwärts scrollenden Prototyps finden Sie hier -> https://github.com/ladroid

### 5. Plattformer

Ein Beispiel für den Aufbau eines Plattformer-Prototyps finden Sie hier -> https://github.com/ladroid

### 6. Einfache Zustände für den Gegner festlegen (Verfolgung/Folgen)

Ein Beispiel für das Festlegen einfacher Zustände für einen Gegner (Verfolgung/Folgen) finden Sie hier -> https://github.com/ladroid

## Japanische Version

Die japanische Version finden Sie hier ->

## Französische Version

Die französische Version finden Sie hier ->

## Deutsche Version

Die deutsche Version finden Sie hier ->

## Spanische Version

Die spanische Version finden Sie hier ->

## Community

[Discord](https://discord.gg/RDW8f2mv)

[GitHub-Diskussion](https://docs.github.com/en/discussions/quickstart)

## Lizenz
goku steht unter der Apache-Lizenz Version 2.0. Siehe [LIZENZ](https://pages.github.com/).
