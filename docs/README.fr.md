# goku

[ [Anglais](../README.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [日本語](README.ja.md) ]


goku est une application de développement de jeux en 2D pour Rust (avec intégration future de jeux en 3D). Écrit entièrement en Rust.

Il est disponible pour **macOS**, **Windows** et **Linux**.

Basé sur SDL2 (actuellement).

goku est axé sur la simplicité, léger et a peu de dépendances (principalement SDL2). Il fournit :

* une fenêtre et une boucle principale

* des graphiques en 2D et du texte

* des sons et de la musique

* une entrée clavier, souris et gamepad

* une interface graphique de développement (GUI)

<ins>Librairies tierces utilisées par goku actuellement :</ins>

* SDL2

* nalgebra

* imgui

* serde

* rfd

* walkdir

## Comment utiliser

La documentation se trouve ici -> [Gitbook](https://lados-organization.gitbook.io/goku/)

## Fonctionnalités

* Graphiques :
    * une fenêtre et une boucle principale

    * graphiques en 2D et texte 
        - Bouton 
        - Curseur 
        - Case à cocher
        - Boîte de texte
        - Polices (seulement le format ttf)
        - Système de particules (étincelles)
        - Fond parallaxe

    * Interface graphique de développement (GUI)
        * Éditeur de texte intégré
        * Console de débogage

    * Formats de fichier image multiples : JPG et PNG

* Audio
    - Lecture
    - Boucle
    - Pause
    - Reprise
    - Formats de fichier audio multiples : OGG, MP3, FLAC, MOD

* Gestionnaire d'entrées :
    * entrée clavier, souris et gamepad

* Types mathématiques :
    * Vector2, Vector3, Vector4
    * Matrice33, Matrice34, Matrice43, Matrice44

* Physique :
    * Collisions
    * Corps rigide (nous n'avons actuellement pas de corps rigide cinématique)

* Scène :
    * Format de fichier JSON flexible : peut décrire une scène entière ou des maillages individuels.

* Animation

* Système d'intelligence artificielle (IA) :
    * Arbre de comportement

* Minuteur

* Système de dialogue

* Prise en charge de plusieurs langues :
    - Allemand
    - Espagnol
    - Japonais
    - Français

* Plateforme de prise en charge :
    - Windows / Mac / Linux
    - Web (WASM non intégré complètement) (références additionnelles [Emscripten](https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/) / [SDL-WASM](https://gitlab.com/ThibaultLemaire/rust-sdl-canvas-wasm) / [Web](https://github.com/koute/cargo-web))
    - Android dans le futur

## Comment exécuter

Commande pour exécuter : `cargo run`

**Important !!!** Actuellement, l'interface graphique (GUI) est encore en développement. J'essaie de combiner imgui et sdl2, mais j'ai besoin de temps pour les rendre totalement compatibles. Ainsi, si quelqu'un souhaite l'utiliser, il est possible d'ajouter le composant `Scene` et d'y écrire un script. Si vous savez comment les combiner, ce serait vraiment génial !

## Comment construire pour le Web

1. Appuyez sur Outils
2. Appuyez sur Construire
3. Exécutez cette commande dans le répertoire où elle a été construite : `cargo web start wasm32-unknown-emscripten` ou `cargo web build --target wasm32-unknown-emscripten`

## À FAIRE

* Améliorer l'interface graphique du moteur ainsi que l'éditeur de texte

* Dessiner des formes simples (cercle, rectangle, triangle, etc.)

* Onglets

* Profilateur

* Vue (probablement besoin d'une combinaison de sdl2 et egui)

* Matériel physique

* Possibilité de créer des jeux en 3D (en considérant Vulkan)

* Améliorer les lumières et les ombres

* Ajouter/améliorer le système de particules

* Améliorer la physique (ajouter un corps rigide cinématique)

* Ajouter/améliorer le système d'interface utilisateur (UI)

* Voxel

* Ajouter des plans (probablement un graphe de nœuds imgui https://github.com/benmkw/imnodes-rs)

* Créer des jeux pour les appareils mobiles iOS, Android

* Créer des jeux pour les consoles (PS4-5), Xbox, Nintendo Switch

* Intégration avec C++ (probablement quelque chose comme bindgen)

## Exemples

### 1. Tetris

Un exemple de construction de jeu Tetris peut être trouvé ici -> https://github.com/ladroid

### 2. Prototype de jeu Roguelike (À FAIRE)

Un exemple de construction de prototype de jeu roguelike peut être trouvé ici -> https://github.com/ladroid

### 3. Effets visuels

1. Étincelles -> 
2. Feu -> 
3. Pluie -> 

### 4. Jeu de défilement latéral

Un exemple de construction de prototype de jeu de défilement latéral peut être trouvé ici -> https://github.com/ladroid

### 5. Plateforme

Un exemple de construction de prototype de jeu de plateforme peut être trouvé ici -> https://github.com/ladroid

### 6. Définir des états simples pour un ennemi (poursuite/suivi)

Un exemple de construction de prototype de jeu de plateforme peut être trouvé ici -> https://github.com/ladroid

## Version en japonais

La version japonaise peut être trouvée ici ->

## Version en français

La version française peut être trouvée ici ->

## Version en allemand

La version allemande peut être trouvée ici ->

## Version en espagnol

La version espagnole peut être trouvée ici ->

## Communauté

[Discord](https://discord.gg/RDW8f2mv)

[Discussion GitHub](https://docs.github.com/en/discussions/quickstart)

## Licence
goku est sous licence Apache version 2.0. Voir le fichier [LICENCE](https://pages.github.com/).
