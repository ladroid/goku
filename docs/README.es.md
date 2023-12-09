# goku

![GitHub Workflow Status](https://img.shields.io/github/commit-activity/t/ladroid/goku)
[![GitHub Repo stars](https://img.shields.io/github/stars/ladroid/goku)](https://github.com/ladroid/goku)
[![Documentation](https://docs.rs/imgui-wgpu/badge.svg)](https://lados-organization.gitbook.io/goku/)
![Repo Size](https://img.shields.io/github/repo-size/ladroid/goku)
![License](https://img.shields.io/github/license/ladroid/goku)

[ [Inglés](../README.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Francés](README.fr.md) | [日本語](README.ja.md) ]


goku es una aplicación de desarrollo de juegos 2D para Rust (con integración futura con juegos 3D). Escrito puramente en Rust.

Está disponible para **macOS**, **Windows** y **Linux**.

Basado en SDL2 (actualmente).

goku es enfocado, ligero y tiene pocas dependencias (principalmente SDL2). Proporciona:

* una ventana y un bucle principal

* gráficos y texto en 2D

* sonidos y música

* entrada de teclado, ratón y gamepad

* interfaz de usuario gráfica para desarrollo

<ins>librerías de terceros utilizadas actualmente por goku:</ins>

* SDL2

* nalgebra

* imgui

* serde

* rfd

* glow

* webbrowser

## Cómo usar

**¡IMPORTANTE!**

La documentación se encuentra aquí -> [Gitbook](https://lados-organization.gitbook.io/goku/)

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
En macOS, es una buena idea instalar estos a través de
[homebrew][homebrew].

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
    > C:\Program Files\Rust\\**lib**\rustlib\x86_64-pc-windows-msvc\lib

    o a (para las versiones de Rust 1.5 y anteriores)
    > C:\Program Files\Rust\\**bin**\rustlib\x86_64-pc-windows-msvc\lib

    o a tu carpeta de bibliotecas de elección, y asegúrate de tener una variable de entorno del sistema de
    > LIB = C:\your\rust\library\folder

    Para los usuarios de Rustup, esta carpeta estará en
    > C:\Users\\{Your Username}\\.rustup\toolchains\\{current toolchain}\lib\rustlib\\{current toolchain}\lib

  Donde current toolchain es probablemente `stable-x86_64-pc-windows-msvc`.

4. Copia SDL2.dll de
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    a tu proyecto de cargo, justo al lado de tu Cargo.toml.

5. Cuando vayas a enviar tu juego, asegúrate de copiar SDL2.dll al mismo directorio que tu exe compilado, de lo contrario, el juego no se iniciará.

#### Vinculación estática con MSVC

Las bibliotecas de desarrollo MSVC proporcionadas por http://libsdl.org/ no incluyen una biblioteca estática. Esto significa que si deseas usar la característica `static-link` con la cadena de herramientas windows-msvc, tendrás que hacer una de las siguientes:

- construir una biblioteca estática SDL2 por ti mismo y copiarla al directorio `lib` de tu cadena de herramientas; o
- también habilitar la característica `bundled`, que construirá una biblioteca estática para ti; o
- usar una biblioteca SDL2 estática de vcpkg como se describe a continuación.

## Características

* Gráficos:
    * una ventana y un bucle principal

    * gráficos y texto en 2D
        - Botón
        - Deslizador
        - Casilla de verificación
        - Caja de texto
        - Fuentes (solo formato ttf)
        - Sistema de partículas (chispas)
        - Fondo parallax

    * interfaz de usuario gráfica para desarrollo
        * editor de texto incorporado
        * Consola de depuración

    * Varios formatos de archivo de imagen: JPG y PNG

    * Iluminación:
        - Luz puntual
        - Luz de punto
        - Filtro de luz ambiental

* Audio
    - Reproducir
    - Loop
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
   

 * Formato de archivo JSON flexible: podría describir una escena completa o mallas individuales.

* Animación

* Sistema de IA:
    * Árbol de comportamiento

* Temporizador

* Sistema de diálogo

* Perfilador

* Soporta varios idiomas:
    - Alemán
    - Español
    - Japonés
    - Francés

* Plataforma de soporte:
    - Windows / Mac / Linux
    - Web (WASM no integrado completamente) (referencias adicionales [Emscripte](https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/) / [SDL-WASM](https://gitlab.com/ThibaultLemaire/rust-sdl-canvas-wasm) / [Web](https://github.com/koute/cargo-web) )
    - Android en el futuro

## Cómo ejecutar

1. ```git clone https://github.com/ladroid/goku.git```
2. extraer todo
3. comando para ejecutar: `cargo run`

**¡Importante!** Actualmente la GUI aún está en desarrollo, estoy intentando combinar imgui y sdl2 juntos pero necesito algo de tiempo para hacerlo completamente compatible. Por lo tanto, si alguien quiere usarlo es posible agregar el componente `Scene` y escribir un script allí. ¡Si sabes cómo combinarlos sería realmente genial!

## Cómo construir para Web

1. Presiona Herramientas
2. Presiona Construir
3. ejecuta este comando en el directorio donde se construyó `cargo web start wasm32-unknown-emscripten` o `cargo web build --target wasm32-unknown-emscripten`

## TODO (está priorizado)

* finalmente hacer un archivo .rs separado en lugar de uno grande

* hacer una ventana de visualización en lugar de la solución actual con canvas (probablemente necesite una ventana separada dentro de la aplicación con combinación de sdl2 e imgui)

* mejorar la física

* agregar/mejorar el sistema de UI (hacer posible agregar imagen para botones)

* mejorar luces y sombras

* dibujar formas simples (círculo, rectángulo, triángulo, etc.)

* pestañas

* mejorar el perfilador

* agregar/mejorar el sistema de partículas

* mejorar la GUI del motor así como el editor de texto (probablemente en lugar del editor de texto incorporado hacer una integración con VSCode u otro ide)

* agregar blueprints (probablemente imgui node graph https://github.com/benmkw/imnodes-rs)

* construir juegos para móviles iOS, Android

* construir juegos para consolas (PS4-5), Xbox, Nintendo Switch

* material físico

* integración con C++ (probablemente algo como un bindgen)

## Ejemplos

### 1. Tetris

El ejemplo de construcción de un juego de tetris se puede encontrar [aquí](../examples/tetris_game_example.rs)

### 2. Prototipo de Roguelike (TODO)

El ejemplo de construcción de un prototipo de Roguelike se puede encontrar aquí -> https://github.com/ladroid

### 3. Efectos visuales

1. Chispas -> simplemente use una función
2. Fuego -> simplemente use una función
3. Lluvia -> simplemente use una función

### 4. Juego de desplazamiento lateral

El ejemplo de construcción de un prototipo de desplazamiento lateral se puede encontrar [aquí](../examples/simple_parallax_example.rs)

### 5. Plataformas

El ejemplo de construcción de un prototipo de juego de plataformas se puede encontrar aquí -> https://github.com/ladroid

### 6. Establecer estados simples para enemigos (persecución/seguimiento)

El ejemplo de construcción de un prototipo de juego de plataformas se puede encontrar [aquí](../examples/enemy_behaviour.rs)

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
goku está licenciado bajo la versión de la licencia Apache 2.0. Ver archivo [LICENCIA](https://pages.github.com/).
