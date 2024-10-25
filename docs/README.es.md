# Goku Engine

<p align="center">
    <img src="../docs/image/Goku_logo.png" width="400" height="400" />
</p>

![GitHub Workflow Status](https://img.shields.io/github/commit-activity/t/ladroid/goku)
[![GitHub Repo stars](https://img.shields.io/github/stars/ladroid/goku)](https://github.com/ladroid/goku)
[![Documentation](https://docs.rs/imgui-wgpu/badge.svg)](https://lados-organization.gitbook.io/goku/)
![Repo Size](https://img.shields.io/github/repo-size/ladroid/goku)
![License](https://img.shields.io/github/license/ladroid/goku)

[ [Inglés](../README.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Francés](README.fr.md) | [日本語](README.ja.md) ]

Goku es una aplicación de desarrollo de juegos 2D para Rust (con integración futura con juegos 3D). Escrito puramente en Rust.

Está disponible para **macOS**, **Windows** y **Linux**.

Basado en SDL2 (actualmente).

Goku es enfocado, ligero y tiene pocas dependencias (principalmente SDL2). Proporciona:

* una ventana y un bucle principal
* gráficos y texto en 2D
* sonidos y música
* entrada de teclado, ratón y gamepad
* interfaz de usuario gráfica para desarrollo
* Generador de personajes en pixel usando IA

<ins>librerías de terceros utilizadas actualmente por Goku:</ins>

* SDL2
* nalgebra
* imgui
* serde
* rfd
* glow
* webbrowser

## Cómo usar

**¡IMPORTANTE!**

* Una visión general rápida está aquí -> [Goku Engine](https://gokuengine.com/)
* La documentación se encuentra aquí -> [Gitbook](https://lados-organization.gitbook.io/goku/)
* Un blog de desarrollo que cubre nuestro progreso, planes y nuevas características se encuentra aquí -> [Noticias](https://gokuengine.com/news)
* Todos los ejemplos están aquí -> [Ejemplos Oficiales](https://github.com/ladroid/goku/tree/main/examples)

## Requisitos
### Linux
Instala estos a través de tu herramienta de gestión de paquetes favorita, o a través de
http://www.libsdl.org/

**Ejemplo de Ubuntu:**
> sudo apt-get install libsdl2-dev

**Ejemplo de Fedora:**
> sudo dnf install SDL2-devel

**Ejemplo de Arch:**
(Arch no tiene paquetes regulares y de desarrollo separados, todo va junto.)
> sudo pacman -S sdl2

También podrías necesitar un compilador de C (`gcc`).

#### Vinculación estática en Linux

Puedes optar por vincular SDL2 estáticamente en lugar de dinámicamente con la característica `static-link`.
En Linux, deberás hacer adicionalmente una de las siguientes:
* usar la característica `bundled`
* usar la característica `use-pkgconfig` para que rustc sepa dónde buscar tus bibliotecas SDL2 y sus dependencias para la vinculación estática. Esto es necesario porque no hay una forma incorporada de encontrar los recursos necesarios para vincular estáticamente SDL2 desde tu sistema
* instalar bibliotecas de desarrollo con [vcpkg][vcpkg]. Las instrucciones para generar un binario estático en Linux y otros sistemas operativos usando vcpkg están [aquí][cargo-vcpkg-usage]

### macOS
#### Homebrew
En macOS, es una buena idea instalar estos a través de [homebrew][homebrew].

```
brew install sdl2
```

En las versiones recientes de Homebrew, las bibliotecas instaladas suelen estar vinculadas en `$(brew --prefix)/lib`.
Si estás ejecutando una versión anterior, el enlace simbólico para SDL podría residir en `/usr/local/lib`.

Para facilitar la vinculación de las bibliotecas instaladas por Homebrew, haz lo siguiente para tu shell respectiva.

Agrega esta línea a tu `~/.zshenv` o `~/.bash_profile` dependiendo de si usas ZSH o Bash.
```
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

### Windows (MSVC)

1. Descarga las bibliotecas de desarrollo MSVC desde http://www.libsdl.org/ (SDL2-devel-2.0.x-VC.zip).
2. Descomprime SDL2-devel-2.0.x-VC.zip en una carpeta de tu elección (Puedes eliminarla después).
3. Copia todos los archivos lib de
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    a (para Rust 1.6 en adelante)
    > C:\Program Files\Rust\**lib**\rustlib\x86_64-pc-windows-msvc\lib

    o a (para las versiones de Rust 1.5 y anteriores)
    > C:\Program Files\Rust\**bin**\rustlib\x86_64-pc-windows-msvc\lib

    o a tu carpeta de bibliotecas de elección, y asegúrate de tener una variable de entorno del sistema de
    > LIB = C:\tu\carpeta\de\biblioteca\de\rust

    Para los usuarios de Rustup, esta carpeta estará en
    > C:\Users\{Tu Nombre de Usuario}\.rustup\toolchains\{cadena de herramientas actual}\lib\rustlib\{cadena de herramientas actual}\lib

  Donde la cadena de herramientas actual es probablemente `stable-x86_64-pc-windows-msvc`.

4. Copia SDL2.dll de
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    a tu proyecto de cargo, justo al lado de tu Cargo.toml.

5. Cuando vayas a enviar tu juego, asegúrate de copiar SDL2.dll al mismo directorio que tu exe compilado, de lo contrario, el juego no se iniciará.

#### Vinculación estática con MSVC

Las bibliotecas de desarrollo MSVC proporcionadas por http://libsdl.org/ no incluyen una biblioteca estática. Esto significa que si deseas usar la característica `static-link` con la cadena de herramientas windows-msvc, tendrás que hacer una de las siguientes:

- construir una biblioteca estática SDL2 por ti mismo y copiarla al directorio `lib` de tu cadena de herramientas; o
- también habilitar la característica `bundled`, que construirá una biblioteca estática para ti; o
- usar una biblioteca SDL2 estática de vcpkg como se describe a continuación.

### Configuración del Generador de Personajes en Píxel
Para la nueva característica de generador de personajes en píxel, asegúrate de que Python 3 y PyTorch estén instalados. Actualmente la IA está escrita en Python, sin embargo, hay un plan para reescribirla en Rust utilizando [tch-rs](https://github.com/LaurentMazare/tch-rs).

#### Cómo instalar PyTorch
> pip3 install torch torchvision torchaudio

Para más información consulta [aquí](https://pytorch.org/get-started/locally/)

#### Cómo configurar PyO3
PyO3 usa un script de compilación (respaldado por la biblioteca pyo3-build-config) para determinar la versión de Python y establecer los argumentos del enlazador correctos. Por defecto intentará usar lo siguiente en orden:

* Cualquier entorno virtual activo de Python.
* El ejecutable python (si es un intérprete de Python 3).
* El ejecutable python3.

Puedes anular el intérprete de Python estableciendo la variable de entorno `PYO3_PYTHON`, por ejemplo, `PYO3_PYTHON=python3.6`, `PYO3_PYTHON=/usr/bin/python3.9`, o incluso un intérprete PyPy `PYO3_PYTHON=pypy3`.

A veces PyO3 puede dar un error al enlazar bibliotecas de Python; para este caso se puede usar una variable de entorno:

Para Windows:
> $env:LIB += ";<ubicación_de_la_biblioteca_python>"

Para UNIX:
> export LIB=$LIB:/ubicación_de_la_biblioteca_python

## Características

* Gráficos:
    * una ventana y un bucle principal

    * gráficos y texto en 2D
        - Botón
        - Deslizador
        - Casilla de verificación
        - Cuadro de texto
        - Fuentes (solo formato ttf)
        - Sistema de partículas (chispas)
        - Fondo parallax

    * interfaz de usuario gráfica para desarrollo
        * editor de texto incorporado
        * Consola de depuración

    * Varios formatos de archivo de imagen: JPG y PNG

    * Iluminación:
        - Luz puntual
        - Luz de foco
        - Filtro de luz ambiental

* Audio
    - Reproducir
    - Bucle
    - Pausa
    - Reanudar
    - Varios formatos de archivo de audio: OGG, MP3, FLAC, MOD

* Controlador de entrada:
    * entrada de teclado, ratón y gamepad

* Tipos matemáticos:
    * Vector2, Vector3, Vector4
    * Matrix33, Matrix34, Matrix43, Matrix44

* Física:
    * Colisiones
    * Cuerpo rígido (actualmente no tenemos un cuerpo rígido cinemático)

* Escena:
    * Formato de archivo JSON flexible: Podría describir una escena completa o mallas individuales.

* Animación

* Sistema de IA:
    * Árbol de comportamiento

* Temporizador

* Sistema de diálogo

* Perfilador

* Soporte para VSCode

* Soporta varios idiomas:
    - Alemán
    - Español
    - Japonés
    - Francés

* Plataforma compatible:
    - Windows / Mac / Linux
    - Web (WASM no integrado completamente) (referencias adicionales [Emscripte](https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/) / [SDL-WASM](https://gitlab.com/ThibaultLemaire/rust-sdl-canvas-wasm) / [Web](https://github.com/koute/cargo-web) )
    - Android en el futuro

## Cómo ejecutar

1. ```git clone https://github.com/ladroid/goku.git```
2. extraer todo
3. comando para ejecutar: `cargo run`

> **¡Importante!**
> Actualmente la GUI aún está en desarrollo, estoy intentando combinar imgui y sdl2 juntos pero necesito algo de tiempo para hacerlo completamente compatible. Por lo tanto, si alguien quiere usarlo, es posible agregar el componente `Scene` y escribir un script allí. ¡Si sabes cómo combinarlos sería realmente genial!

## Cómo construir para Web

1. Presiona Herramientas
2. Presiona Construir y elige Web

## Cómo habilitar Viewport

Para habilitar Viewport ve a Preferencias -> General -> Habilitar lienzo, lo mismo con el modo de vista de cuadrícula

## TODO (priorizado)

* ~~finalmente hacer un archivo .rs separado en lugar de uno grande~~

* ~~hacer una ventana de visualización en lugar de la solución actual con lienzo (probablemente necesite una ventana separada dentro de la aplicación con combinación de sdl2 y OpenGL)~~

* combinar con wgpu ([Ejemplo de la biblioteca sdl2](https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/raw-window-handle-with-wgpu/main.rs) y [renderizador imgui para wgpu-rs](https://github.com/Yatekii/imgui-wgpu-rs))

* mejorar la física

* agregar/mejorar el sistema de UI (hacer posible agregar imagen para botones)

* ~~mejorar luces y sombras~~

* dibujar formas simples (círculo, rectángulo, triángulo, etc.)

* ~~pestañas~~

* ~~mejorar el perfilador~~

* ~~agregar/mejorar el sistema de partículas~~

* ~~mejorar la GUI del motor así como el editor de texto (probablemente en lugar del editor de texto incorporado hacer una integración con VSCode u otro IDE)~~

* agregar planos (probablemente imgui node graph https://github.com/benmkw/imnodes-rs)

* construir juegos para móviles iOS, Android

* construir juegos para consolas (PS4-5), Xbox, Nintendo Switch

* material físico

* integración con C++ (probablemente algo como un bindgen)

## Ejemplos

### 1. Tetris

El ejemplo de construcción de un juego de tetris se puede encontrar [aquí](examples/tetris_game_example.rs)

### 2. Prototipo Roguelike (Prototipo)

El ejemplo de construcción de un prototipo roguelike se puede encontrar [aquí](examples/roguelike/README_game.md)

### 3. Efectos visuales

1. Chispas -> usa una función `spawn_particles_sparks` [de aquí](src/two_d/particle_system.rs)
2. Fuego -> usa una función `spawn_particles_fires` [de aquí](src/two_d/particle_system.rs)
3. Lluvia -> usa una función `spawn_particles_rain` [de aquí](src/two_d/particle_system.rs)

### 4. Juego de desplazamiento lateral

El ejemplo de construcción de un prototipo de desplazamiento lateral se puede encontrar [aquí](examples/simple_parallax_example.rs)

### 5. Plataformas

El ejemplo de construcción de un prototipo de plataformas se puede encontrar [aquí](examples/simple_platformer.rs)

### 6. Establecer estados simples para enemigos (persecución/seguimiento)

El ejemplo de construcción de un prototipo de plataformas se puede encontrar [aquí](examples/enemy_behaviour.rs)

## Versión japonesa

La versión japonesa se puede encontrar [aquí](https://lados-organization.gitbook.io/goku/v/goku-game-engine_jp/)

## Versión francesa

La versión francesa se puede encontrar [aquí](https://lados-organization.gitbook.io/goku/v/goku-game-engine_fr/)

## Versión alemana

La versión alemana se puede encontrar [aquí](https://lados-organization.gitbook.io/goku/v/goku-game-engine_de/)

## Versión española

La versión española se puede encontrar [aquí](https://lados-organization.gitbook.io/goku/v/goku-game-engine_es/)

## Cómo contribuir

### Presentación de problemas
Usa el Issue Tracker para enviar informes de errores y solicitudes de características/mejoras. Antes de enviar un nuevo problema, asegúrate de que no haya un problema abierto similar.

### Pruebas manuales
¡Cualquiera que pruebe manualmente el código e informe errores o sugerencias de mejoras en el Issue Tracker es muy bienvenido!

### Presentación de Pull Requests
Los parches/correcciones se aceptan en forma de pull requests (PRs). Asegúrate de que el problema que aborda la pull request esté abierto en el Issue Tracker.

La pull request presentada se considera que ha aceptado publicar bajo la licencia Apache 2.0.

## Comunidad

[Discord](https://discord.gg/9TAMqdRyED)

[Discusión de GitHub](https://docs.github.com/en/discussions/quickstart)

## Licencia
Goku está licenciado bajo la versión de la licencia Apache 2.0. Ver archivo [LICENCIA](https://pages.github.com/).