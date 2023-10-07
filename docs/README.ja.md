# goku

![GitHub Workflow Status](https://img.shields.io/github/commit-activity/t/ladroid/goku)
[![GitHub Repo stars](https://img.shields.io/github/stars/ladroid/goku)](https://github.com/ladroid/goku)
[![Documentation](https://docs.rs/imgui-wgpu/badge.svg)](https://lados-organization.gitbook.io/goku/)
![Repo Size](https://img.shields.io/github/repo-size/ladroid/goku)
![License](https://img.shields.io/github/license/ladroid/goku)

[ [English](../README.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [日本語](README.ja.md) ]


gokuはRust用の2Dゲーム開発アプリケーションであり（将来的には3Dゲームとの統合が計画されています）、完全にRustで書かれています。

**macOS**、**Windows**、および**Linux**で利用可能です。

現在はSDL2に基づいています。

gokuは集中的で軽量であり、依存関係が少ない（主にSDL2）です。以下の機能を提供します：

* ウィンドウとメインループ

* 2Dグラフィックスとテキスト

* サウンドと音楽

* キーボード、マウス、ゲームパッドの入力

* 開発のためのGUIインターフェイス

<ins>現在gokuが使用しているサードパーティのライブラリ：</ins>

* SDL2

* nalgebra

* imgui

* serde

* rfd

* glow

* webbrowser

## 使い方

**重要!!!**

ドキュメントはこちらにあります -> [Gitbook](https://lados-organization.gitbook.io/goku/)

## 要件
### Linux
お気に入りのパッケージ管理ツールを使用してインストールするか、または
http://www.libsdl.org/ を通じてインストールします。

**Ubuntuの例:**
> sudo apt-get install libsdl2-dev

**Fedoraの例:**
> sudo dnf install SDL2-devel

**Archの例:**
(Archは通常のパッケージと開発パッケージが別々になっていないので、すべて一緒になります。)
> sudo pacman -S sdl2

Cコンパイラ（`gcc`）も必要になるかもしれません。

#### Linuxでの静的リンク

`static-link`機能を使用してSDL2を動的にリンクする代わりに静的にリンクすることを選択できます。
Linuxでは、次のいずれかを追加で行う必要があります：
* `bundled`機能を使用する
* rustcがSDL2ライブラリとその依存関係を静的リンクするためのリソースを探す場所を知るために`use-pkgconfig`機能を使用する。これは、システムから静的にSDL2をリンクするために必要なリソースを見つける組み込みの方法がないため必要です
* [vcpkg][vcpkg]を使用して開発ライブラリをインストールする。vcpkgを使用してLinuxおよび他のオペレーティングシステムで静的バイナリを生成する方法に関する指示は[こちら][cargo-vcpkg-usage]にあります

### macOS
#### Homebrew
macOSでは、これらを
[homebrew][homebrew]経由でインストールすることをお勧めします。

```
brew install sdl2
```

Homebrewの最近のバージョンでは、インストールされたライブラリは通常`$(brew --prefix)/lib`にリンクされます。
古いバージョンを実行している場合、SDLのシンボリックリンクは`/usr/local/lib`に存在する可能性があります。

Homebrewによってインストールされたライブラリのリンクを簡単にするために、それぞれのシェルで以下の手順を行ってください。

`~/.zshenv`または`~/.bash_profile`にこの行を追加して、ZSHまたはBashを使用しているかどうかに応じています。
```
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

### Windows（MSVC）

1. http://www.libsdl.org/ からMSVC開発ライブラリをダウンロードします（SDL2-devel-2.0.x-VC.zip）。
2. SDL2-devel-2.0.x-VC.zipをお好きなフォルダに解凍します（後で削除することができます）。
3. 以下のlibファイルをすべてコピーします
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    Rust 1.6以降の場合は
    > C:\Program Files\Rust\\**lib**\rustlib\x86_64-pc-windows-msvc\lib

    Rustバージョン1.5およびそれ以下の場合は
    > C:\Program Files\Rust\\**bin**\rustlib\x86_64-pc-windows-msvc\lib

    またはお好きなライブラリフォルダにコピーし、以下のシステム環境変数が存在することを確認します
    > LIB = C:\your\rust\library\folder

    Rustupユーザーの場合、このフォルダは
    > C:\Users\\{Your Username}\\.rustup\toolchains\\{current toolchain}\lib\rustlib\\{current toolchain}\lib

  現在のツールチェーンはおそらく`stable-x86_64-pc-windows-msvc`です。

4. SDL2.dllを
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    からcargoプロジェクトにコピーし、Cargo.tomlのすぐ隣に置きます。

5. ゲームを出荷するときは、SDL2.dllをコンパイルされたexeと同じディレクトリにコピーして、ゲームが起動しないようにします。

#### MSVCと静的リンク

http://libsdl.org/ で提供されているMS

VC開発ライブラリには静的ライブラリが含まれていないため、windows-msvcツールチェーンで`static-link`機能を使用する場合、次のいずれかを行う必要があります。

- SDL2静的ライブラリを自分でビルドし、ツールチェーンの`lib`ディレクトリにコピーする；または
- `bundled`機能も有効にし、静的ライブラリをビルドする；または
- 下記のようにvcpkgから静的なSDL2ライブラリを使用する。

## 機能

* グラフィックス：
    * ウィンドウとメインループ

    * 2Dグラフィックスとテキスト
        - ボタン
        - スライダー
        - チェックボックス
        - テキストボックス
        - フォント（ttf形式のみ）
        - パーティクルシステム（スパーク）
        - パララックス背景

    * 開発用のGUIインターフェイス
        * 組み込みのテキストエディタ
        * デバッグコンソール

    * 複数の画像ファイル形式：JPGおよびPNG

    * ライティング：
        - ポイントライト
        - スポットライト
        - アンビエントライトフィルタ

* オーディオ
    - 再生
    - ループ
    - 一時停止
    - 再開
    - 複数のオーディオファイル形式：OGG、MP3、FLAC、MOD

* 入力ハンドラ：
    * キーボード、マウス、およびゲームパッドの入力

* 数学型：
    * Vector2 Vector3、Vector4
    * Matrix33、Matrix34、Matrix43、Matrix44

* 物理：
    * 衝突
    * 剛体（現在、キネマティック剛体は持っていません）

* シーン：
    * 柔軟なJSONファイル形式：全体のシーンまたは個々のメッシュを記述することができます。

* アニメーション

* AIシステム：
    * ビヘイビアツリー

* タイマー

* ダイアログシステム

* プロファイラー

* いくつかの言語をサポート：
    - ドイツ語
    - スペイン語
    - 日本語
    - フランス語

* サポートプラットフォーム：
    - Windows / Mac / Linux
    - Web（WASMは完全に統合されていません）（追加の参照 [Emscripte](https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/) / [SDL-WASM](https://gitlab.com/ThibaultLemaire/rust-sdl-canvas-wasm) / [Web](https://github.com/koute/cargo-web) )
    - 将来的にAndroid

## 実行方法

1. ```git clone https://github.com/ladroid/goku.git```
2. すべてを抽出
3. 実行コマンド：`cargo run`

**重要!!!** 現在、GUIはまだ開発中であり、imguiとsdl2を組み合わせることを試していますが、完全に互換性を持たせるには時間がかかります。したがって、それを使用することを希望する人は、コンポーネント`Scene`を追加し、そこにスクリプトを書くことが可能です。それらを組み合わせる方法を知っている場合は、本当に素晴らしいことになります！

## Web用にビルドする方法

1. ツールを押します
2. ビルドを押します
3. ビルドされたディレクトリでこのコマンドを実行します `cargo web start wasm32-unknown-emscripten` または `cargo web build --target wasm32-unknown-emscripten`

## 今後の課題（優先されています）

* 最終的には1つの大きなものの代わりに別の.rsファイルを作成する

* canvasを使った現在のソリューションの代わりにビューポートを作成する（おそらくアプリ内の別のウィンドウでsdl2とimguiを組み合わせる必要があります）

* 物理を改善する

* UIシステムを追加/改善する（ボタンに画像を追加することが可能になる）

* ライトとシャドウを改善する

* 単純な形状を描画する（円、長方形、三角形など）

* タブ

* プロファイラを改善する

* パーティクルシステムを追加/改善する

* エンジンのGUIおよびテキストエディタを改善する（おそらく組み込みのテキストエディタの代わりにVSCodeまたは他のideとの統合を作成する）

* ブループリントを追加する（おそらくimgui node graph https://github.com/benmkw/imnodes-rs）

* モバイルiOS、Android用のゲームをビルドする



* コンソール用のゲームをビルドする（PS4-5）、Xbox、Nintendo Switch

* 物理マテリアル

* C++との統合（おそらくbindgenのようなもの）

## 例

### 1. テトリス

テトリスゲームをビルドする例は[こちら](examples/tetris_game_example.txt)で見つけることができます

### 2. ローグライクプロトタイプ（TODO）

ローグライクプロトタイプをビルドする例はこちらで見つけることができます -> https://github.com/ladroid

### 3. ビジュアルエフェクト

1. スパーク -> 関数を簡単に使用します
2. 火 -> 関数を簡単に使用します
3. 雨 -> 関数を簡単に使用します

### 4. サイドスクロールゲーム

サイドスクロールプロトタイプをビルドする例は[こちら](examples/simple_parallax_example.txt)で見つけることができます

### 5. プラットフォーマー

プラットフォーマープロトタイプをビルドする例はこちらで見つけることができます -> https://github.com/ladroid

### 6. 敵のシンプルな状態を設定する（追跡/追従）

プラットフォーマープロトタイプをビルドする例は[こちら](examples/enemy_behaviour.txt)で見つけることができます

## Japanese ver.

日本語版は[こちら](https://lados-organization.gitbook.io/goku/v/goku-game-engine_jp/)で見つけることができます

## French ver.

フランス語版は[こちら](https://lados-organization.gitbook.io/goku/v/goku-game-engine_fr/)で見つけることができます

## German ver.

ドイツ語版は[こちら](https://lados-organization.gitbook.io/goku/v/goku-game-engine_de/)で見つけることができます

## Spanish ver.

スペイン語版は[こちら](https://lados-organization.gitbook.io/goku/v/goku-game-engine_es/)で見つけることができます

## 貢献する方法

### 問題の提出
問題トラッカーを使用してバグレポートと機能/拡張リクエストを提出します。新しい問題を提出する前に、同様のオープンな問題がないことを確認してください。

### 手動テスト
コードを手動でテストし、問題トラッカーでバグを報告したり、拡張の提案をしたりする人は大歓迎です！

### プルリクエストの提出
パッチ/修正はプルリクエスト（PR）の形式で受け付けられます。プルリクエストの問題が問題トラッカーでオープンであることを確認してください。

提出されたプルリクエストは、Apache 2.0ライセンスの下で公開することに同意したものと見なされます。

## コミュニティ

[Discord](https://discord.gg/9TAMqdRyED)

[GitHub Discussion](https://docs.github.com/en/discussions/quickstart)

## ライセンス
gokuはApacheライセンスバージョン2.0の下でライセンスされています。[LICENSE](https://pages.github.com/)ファイルを参照してください。
