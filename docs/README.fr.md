# Goku Engine

<p align="center">
    <img src="../docs/image/Goku_logo.png" width="400" height="400" />
</p>

![GitHub Workflow Status](https://img.shields.io/github/commit-activity/t/ladroid/goku)
[![GitHub Repo stars](https://img.shields.io/github/stars/ladroid/goku)](https://github.com/ladroid/goku)
[![Documentation](https://docs.rs/imgui-wgpu/badge.svg)](https://lados-organization.gitbook.io/goku/)
![Repo Size](https://img.shields.io/github/repo-size/ladroid/goku)
![License](https://img.shields.io/github/license/ladroid/goku)

[ [Anglais](../README.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [日本語](README.ja.md) ]

Goku est une application de développement de jeux 2D pour Rust (avec une intégration future pour les jeux 3D). Entièrement écrit en Rust.

Il est disponible pour **macOS**, **Windows** et **Linux**.

Basé sur SDL2 (actuellement).

Goku est focalisé, léger et a peu de dépendances (principalement SDL2). Il propose :

* une fenêtre et une boucle principale

* des graphiques et du texte en 2D

* des sons et de la musique

* une entrée pour le clavier, la souris et le gamepad

* une interface graphique pour le développement

* Générateur de caractères en pixel utilisant l'IA

<ins>Les bibliothèques tierces actuellement utilisées par Goku :</ins>

* SDL2

* nalgebra

* imgui

* serde

* rfd

* glow

* webbrowser

## Comment utiliser

**IMPORTANT!!!**

* Un aperçu rapide est disponible ici -> [Goku Engine](https://gokuengine.com/)
* La documentation est située ici -> [Gitbook](https://lados-organization.gitbook.io/goku/)
* Un blog de développement couvrant nos progrès, plans et nouvelles fonctionnalités est situé ici -> [Actualités](https://gokuengine.com/news)
* Tous les exemples sont ici -> [Exemples officiels](https://github.com/ladroid/goku/tree/main/examples)

## Exigences
### Linux
Installez-les via votre outil de gestion de paquets préféré, ou via
http://www.libsdl.org/

**Exemple Ubuntu :**
> sudo apt-get install libsdl2-dev

**Exemple Fedora :**
> sudo dnf install SDL2-devel

**Exemple Arch :**
(Arch n'a pas de paquets réguliers et de développement séparés, tout est regroupé.)
> sudo pacman -S sdl2

Vous pourriez aussi avoir besoin d'un compilateur C (`gcc`).

#### Liaison statique sous Linux

Vous pouvez choisir de lier SDL2 statiquement plutôt que dynamiquement avec la fonction `static-link`.
Sous Linux, vous devrez en outre faire l'un des suivants :
* utiliser la fonction `bundled`
* utiliser la fonction `use-pkgconfig` pour que rustc sache où chercher vos bibliothèques SDL2 et leurs dépendances pour la liaison statique. Ceci est nécessaire car il n'y a pas de moyen intégré pour trouver les ressources nécessaires pour lier statiquement SDL2 depuis votre système
* installer des bibliothèques de développement avec [vcpkg][vcpkg]. Les instructions pour générer un binaire statique sous Linux et d'autres systèmes d'exploitation en utilisant vcpkg sont [ici][cargo-vcpkg-usage]

### macOS
#### Homebrew
Sur macOS, il est conseillé de les installer via
[homebrew][homebrew].

```
brew install sdl2
```

Dans les versions récentes de Homebrew, les bibliothèques installées sont généralement liées dans `$(brew --prefix)/lib`.
Si vous utilisez une version plus ancienne, le lien symbolique pour SDL pourrait résider dans `/usr/local/lib`.

Pour faciliter la liaison des bibliothèques installées par Homebrew, faites ce qui suit pour votre shell respectif.

Ajoutez cette ligne à votre `~/.zshenv` ou `~/.bash_profile` selon que vous utilisez ZSH ou Bash.
```
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

### Windows (MSVC)

1. Téléchargez les bibliothèques de développement MSVC depuis http://www.libsdl.org/ (SDL2-devel-2.0.x-VC.zip).
2. Décompressez SDL2-devel-2.0.x-VC.zip dans un dossier de votre choix (Vous pouvez le supprimer par la suite).
3. Copiez tous les fichiers lib de
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    à (pour Rust 1.6 et supérieur)
    > C:\Program Files\Rust\**lib**\rustlib\x86_64-pc-windows-msvc\lib

    ou à (pour les versions de Rust 1.5 et inférieures)
    > C:\Program Files\Rust\**bin**\rustlib\x86_64-pc-windows-msvc\lib

    ou dans votre dossier de bibliothèques de choix, et assurez-vous d'avoir une variable d'environnement système de
    > LIB = C:\your\rust\library\folder

    Pour les utilisateurs de Rustup, ce dossier sera dans
    > C:\Users\{Votre nom d'utilisateur}\.rustup\toolchains\{chaîne d'outils actuelle}\lib\rustlib\{chaîne d'outils actuelle}\lib

  Où la chaîne d'outils actuelle est probablement `stable-x86_64-pc-windows-msvc`.

4. Copiez SDL2.dll de
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    dans votre projet cargo, juste à côté de votre Cargo.toml.

5. Lorsque vous expédiez votre jeu, assurez-vous de copier SDL2.dll dans le même répertoire que votre exe compilé, sinon le jeu ne se lancera pas.

#### Liaison statique avec MSVC

Les bibliothèques de développement MSVC fournies par http://libsdl.org/ n'incluent pas de bibliothèque statique. Cela signifie que si vous voulez utiliser la fonction `static-link` avec la chaîne d'outils windows-msvc, vous devrez faire l'un des suivants :

- construire une bibliothèque statique SDL2 vous-même et la copier dans le répertoire `lib` de votre chaîne d'outils ; ou
- également activer la fonction `bundled`, qui construira une bibliothèque statique pour vous ; ou
- utiliser une bibliothèque SDL2 statique de vcpkg comme décrit ci-dessous.

### Configuration du générateur de caractères en pixel
Pour la nouvelle fonctionnalité de générateur de caractères en pixel, assurez-vous que Python 3 et PyTorch sont installés. Actuellement, l'IA est écrite en Python, mais il est prévu de la réécrire en Rust en utilisant [tch-rs](https://github.com/LaurentMazare/tch-rs).

#### Comment installer PyTorch
> pip3 install torch torchvision torchaudio

Pour plus d'informations, consultez [ici](https://pytorch.org/get-started/locally/)

#### Comment configurer PyO3
PyO3 utilise un script de construction (soutenu par le crate pyo3-build-config) pour déterminer la version de Python et définir les bons arguments de l'éditeur de liens. Par défaut, il tentera d'utiliser ce qui suit dans l'ordre :

* Tout environnement virtuel Python actif.
* L'exécutable python (s'il s'agit d'un interpréteur Python 3).
* L'exécutable python3.

Vous pouvez remplacer l'interpréteur Python en définissant la variable d'environnement `PYO3_PYTHON`, par exemple `PYO3_PYTHON=python3.6`, `PYO3_PYTHON=/usr/bin/python3.9`, ou même un interpréteur PyPy `PYO3_PYTHON=pypy3`.

Parfois, PyO3 peut donner une erreur avec la liaison des bibliothèques Python ; dans ce cas, vous pouvez utiliser la variable d'environnement suivante :

Pour Windows :
> $env:LIB += ";<emplacement_de_la_bibliothèque_python>"

Pour UNIX :
> export LIB=$LIB:/emplacement_de_la_bibliothèque_python

## Caractéristiques

* Graphismes :
    * une fenêtre et une boucle principale

    * graphismes et texte en 2D
        - Bouton
        - Curseur
        - Case à cocher
        - Zone de texte
        - Polices (uniquement au format ttf)
        - Système de particules (étincelles)
        - Fond parallaxe

    * interface graphique pour le développement
        - éditeur de texte intégré
        - Console de débogage

    * Plusieurs formats de fichiers image : JPG et PNG

    * Éclairage :
        - Lumière ponctuelle
        - Lumière spot
        - Filtre de lumière ambiante

* Audio
    - Jouer
    - Boucle
    - Pause
    - Reprendre
    - Plusieurs formats de fichiers audio : OGG, MP3, FLAC, MOD

* Gestionnaire d'entrée :
    * entrée clavier, souris, et gamepad

* Types mathématiques :
    * Vector2, Vector3, Vector4
    * Matrix33, Matrix34, Matrix43, Matrix44

* Physique :
    * Collisions
    * Corps rigides (actuellement nous n'avons pas de corps rigide cinématique)

* Scène :
    * Format de fichier JSON flexible : Peut décrire soit une scène entière, soit des maillages individuels.

* Animation

* Système d'IA :
    * Arbre de comportement

* Minuteur

* Système de dialogue

* Profileur

* Prise en charge de VSCode

* Prise en charge de plusieurs langues :
    - Allemand
    - Espagnol
    - Japonais
    - Français

* Plateformes supportées :
    - Windows / Mac / Linux
    - Web (WASM non entièrement intégré) (références supplémentaires [Emscripte](https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/) / [SDL-WASM](https://gitlab.com/ThibaultLemaire/rust-sdl-canvas-wasm) / [Web](https://github.com/koute/cargo-web) )
    - Android dans le futur

## Comment exécuter

1. ```git clone https://github.com/ladroid/goku.git```
2. Extraire tout
3. Commande pour exécuter : `cargo run`

> **Important !!!**
> Actuellement, l'interface utilisateur graphique est toujours en développement, j'essaie de combiner imgui et sdl2 ensemble mais j'ai besoin de temps pour le rendre entièrement compatible. Ainsi, si quelqu'un veut l'utiliser, il est possible d'ajouter le composant `Scene` et d'écrire un script là. Si vous savez comment les combiner, ce serait vraiment génial !

## Comment construire pour le Web

1. Appuyez sur Outils
2. Appuyez sur Construire et choisissez Web

## Comment activer le Viewport

Pour activer le Viewport, allez dans Préférences -> Général -> Activer la toile de la même manière que pour le mode Vue de grille.

## À faire (est priorisé)

* ~~créer enfin un fichier .rs séparé au lieu d'un grand~~  

* ~~créer une fenêtre de visualisation au lieu de la solution actuelle avec la toile (probablement besoin d'une fenêtre séparée dans l'application avec combinaison de sdl2 et OpenGL)~~

* combiner avec wgpu ([Exemple de la bibliothèque sdl2](https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/raw-window-handle-with-wgpu/main.rs) et [rendu imgui pour wgpu-rs](https://github.com/Yatekii/imgui-wgpu-rs))

* améliorer la physique

* ajouter/améliorer le système d'interface utilisateur (rendre possible l'ajout d'images pour les boutons)

* ~~améliorer les lumières et les ombres~~

* dessiner des formes simples (cercle, rectangle, triangle, etc.)

* ~~onglets~~

* ~~améliorer le profileur~~

* ~~ajouter/améliorer le système de particules~~

* ~~améliorer l'interface utilisateur du moteur ainsi que l'éditeur de texte (probablement au lieu de l'éditeur de texte intégré faire une intégration avec VSCode ou un autre ide)~~

* ajouter des blueprints (probablement imgui node graph https://github.com/benmkw/imnodes-rs)

* créer des jeux pour mobiles iOS, Android

* créer des jeux pour consoles (PS4-5), Xbox, Nintendo Switch

* matériau physique

* intégration avec C++ (probablement quelque chose comme bindgen)

## Exemples

### 1. Tetris

L'exemple de construction d'un jeu Tetris peut être trouvé [ici](examples/tetris_game_example.rs)

### 2. Prototype de Roguelike (Prototype)

L'exemple de construction d'un prototype de roguelike peut être trouvé [ici](examples/roguelike/README_game.md)

### 3. Effets visuels

1. Étincelles -> utilisez une fonction `spawn_particles_sparks` [d'ici](src/two_d/particle_system.rs)
2. Feu -> utilisez une fonction `spawn_particles_fires` [d'ici](src/two_d/particle_system.rs)
3. Pluie -> utilisez une fonction `spawn_particles_rain` [d'ici](src/two_d/particle_system.rs)

### 4. Jeu à défilement latéral

L'exemple de construction d'un prototype de jeu à défilement latéral peut être trouvé [ici](examples/simple_parallax_example.rs)

### 5. Plateforme

L'exemple de construction d'un prototype de jeu de plateforme peut être trouvé [ici](examples/simple_platformer.rs)

### 6. Définir des états simples pour l'ennemi (poursuite/suivi)

L'exemple de construction d'un prototype de jeu de plateforme peut être trouvé [ici](examples/enemy_behaviour.rs)

## Version japonaise

La version japonaise peut être trouvée [ici](https://lados-organization.gitbook.io/goku/v/goku-game-engine_jp/)

## Version française

La version française peut être trouvée [ici](https://lados-organization.gitbook.io/goku/v/goku-game-engine_fr/)

## Version allemande

La version allemande peut être trouvée [ici](https://lados-organization.gitbook.io/goku/v/goku-game-engine_de/)

## Version espagnole

La version espagnole peut être trouvée [ici](https://lados-organization.gitbook.io/goku/v/goku-game-engine_es/)

## Comment contribuer

### Soumettre des problèmes
Utilisez le suivi des problèmes pour soumettre des rapports de bogues et des demandes de fonctionnalités/améliorations. Avant de soumettre un nouveau problème, assurez-vous qu'il n'y a pas de problème ouvert similaire.

### Tests manuels
Toute personne testant manuellement le code et signalant des bogues ou des suggestions d'améliorations dans le suivi des problèmes est la bienvenue !

### Soumission de Pull Requests
Les correctifs sont acceptés sous forme de pull requests (PRs). Assurez-vous que le problème que la pull request adresse est ouvert dans le suivi des problèmes.

La pull request soumise est considérée comme ayant accepté de publier sous licence Apache 2.0.

## Communauté

[Discord](https://discord.gg/9TAMqdRyED)

[Discussion sur GitHub](https://docs.github.com/en/discussions/quickstart)

## Licence
Goku est licencié sous la licence Apache version 2.0. Voir le fichier [LICENCE](https://pages.github.com/).