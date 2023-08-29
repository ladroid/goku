# goku

[ [Inglés](../README.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Francés](README.fr.md) | [日本語](README.ja.md) ]


goku es una aplicación de desarrollo de juegos 2D para Rust (con integración futura de juegos 3D). Escrita completamente en Rust.

Está disponible para **macOS**, **Windows** y **Linux**.

Basada en SDL2 (actualmente).

goku es enfocada, ligera y tiene pocas dependencias (principalmente SDL2). Proporciona:

* una ventana y un bucle principal

* gráficos y texto en 2D

* sonidos y música

* entrada de teclado, ratón y gamepad

* interfaz gráfica de usuario (GUI) para el desarrollo

<ins>Bibliotecas de terceros utilizadas por goku actualmente:</ins>

* SDL2

* nalgebra

* imgui

* serde

* rfd

* walkdir

## Cómo usar

La documentación se encuentra aquí -> [Gitbook](https://lados-organization.gitbook.io/goku/)

## Características

* Gráficos:
    * una ventana y un bucle principal

    * gráficos y texto en 2D 
        - Botón 
        - Control deslizante 
        - Casilla de verificación
        - Cuadro de texto
        - Fuentes (solo formato ttf)
        - Sistema de partículas (chispas)
        - Fondo de paralaje

    * Interfaz gráfica de usuario (GUI) para el desarrollo
        * Editor de texto integrado
        * Consola de depuración

    * Múltiples formatos de archivo de imagen: JPG y PNG

* Audio
    - Reproducir
    - Bucle
    - Pausa
    - Reanudar
    - Múltiples formatos de archivo de audio: OGG, MP3, FLAC, MOD

* Manejador de entrada:
    * entrada de teclado, ratón y gamepad

* Tipos matemáticos:
    * Vector2, Vector3, Vector4
    * Matriz33, Matriz34, Matriz43, Matriz44

* Física:
    * Colisiones
    * Cuerpo rígido (actualmente no tenemos un cuerpo rígido cinemático)

* Escena:
    * Formato de archivo JSON flexible: Puede describir toda una escena o mallas individuales.

* Animación

* Sistema de inteligencia artificial (IA):
    * Árbol de comportamiento

* Temporizador

* Sistema de diálogo

* Soporte para varios idiomas:
    - Alemán
    - Español
    - Japonés
    - Francés

* Plataforma de soporte:
    - Windows / Mac / Linux
    - Web (WASM no integrado completamente) (referencias adicionales [Emscripten](https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/) / [SDL-WASM](https://gitlab.com/ThibaultLemaire/rust-sdl-canvas-wasm) / [Web](https://github.com/koute/cargo-web))
    - Android en el futuro

## Cómo ejecutar

Comando para ejecutar: `cargo run`

**¡Importante!** Actualmente la interfaz gráfica de usuario (GUI) aún está en desarrollo. Estoy intentando combinar imgui y sdl2, pero necesito tiempo para que sean completamente compatibles. Por lo tanto, si alguien desea utilizarlo, es posible agregar el componente `Scene` y escribir un script allí. Si sabes cómo combinarlos, ¡sería genial!

## Cómo compilar para la web

1. Presiona en Herramientas
2. Presiona en Compilar
3. Ejecuta este comando en el directorio donde se construyó: `cargo web start wasm32-unknown-emscripten` o `cargo web build --target wasm32-unknown-emscripten`

## POR HACER

* Mejorar la interfaz gráfica del motor, así como el editor de texto

* Dibujar formas simples (círculo, rectángulo, triángulo, etc.)

* Pestañas

* Perfilador

* Vista (probablemente se necesite la combinación de sdl2 y egui)

* Material de física

* Posibilidad de crear juegos 3D (considerando Vulkan)

* Mejorar luces y sombras

* Agregar/mejorar sistema de partículas

* Mejorar física (agregar cuerpo rígido cinemático)

* Agregar/mejorar sistema de interfaz de usuario (UI)

* Voxel

* Agregar planos (probablemente gráficos de nodos imgui https://github.com/benmkw/imnodes-rs)

* Creación de juegos para dispositivos móviles iOS, Android

* Creación de juegos para consolas (PS4-5), Xbox, Nintendo Switch

* Integración con C++ (probablemente algo como bindgen)

## Ejemplos

### 1. Tetris

Un ejemplo de cómo construir un juego de Tetris se puede encontrar aquí -> https://github.com/ladroid

### 2. Prototipo de Roguelike (POR HACER)

Un ejemplo de cómo construir un prototipo de roguelike se puede encontrar aquí -> https://github.com/ladroid

### 3. Efectos visuales

1. Chispas -> 
2. Fuego -> 
3. Lluvia -> 

### 4. Juego de desplazamiento lateral

Un ejemplo de cómo construir un prototipo de desplazamiento lateral se puede encontrar aquí -> https://github.com/ladroid

### 5. Plataformero

Un ejemplo de cómo construir un prototipo de plataforma se puede encontrar aquí -> https://github.com/ladroid

### 6. Establecer estados simples para un enemigo (perseguir/seguir)

Un ejemplo de cómo establecer estados simples para un enemigo (perseguir/seguir) se puede encontrar aquí -> https://github.com/ladroid

## Versión en japonés

La versión en japonés se puede encontrar aquí ->

## Versión en francés

La versión en francés se puede encontrar aquí ->

## Versión en alemán

La versión en alemán se puede encontrar aquí ->

## Versión en español

La versión en español se puede encontrar aquí ->

## Comunidad

[Discord](https://discord.gg/RDW8f2mv)

[Discusión en GitHub](https://docs.github.com/en/discussions/quickstart)

## Licencia
goku está bajo la licencia Apache versión 2.0. Consulta el archivo [LICENCIA](https://pages.github.com/).
