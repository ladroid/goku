# goku

![GitHub Workflow Status](https://img.shields.io/github/commit-activity/t/ladroid/goku)
[![GitHub Repo stars](https://img.shields.io/github/stars/ladroid/goku)](https://github.com/ladroid/goku)
[![Documentation](https://docs.rs/imgui-wgpu/badge.svg)](https://lados-organization.gitbook.io/goku/)
![Repo Size](https://img.shields.io/github/repo-size/ladroid/goku)
![License](https://img.shields.io/github/license/ladroid/goku)

[ [English](../README.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [日本語](README.ja.md) ]


gokuはRust向けの2Dゲーム開発アプリケーションです（将来的には3Dゲームとの統合も予定しています）。純粋にRustで書かれています。

**macOS**、**Windows**、**Linux**で利用可能です。

現在のところSDL2をベースにしています。

gokuは、軽量かつ依存関係が少ない（主にSDL2）を特徴としています。以下を提供します：

* ウィンドウとメインループ

* 2Dグラフィックスとテキスト

* サウンドと音楽

* キーボード、マウス、ゲームパッド入力

* 開発のためのGUIインターフェース

<ins>現在gokuが使用しているサードパーティライブラリ：</ins>

* SDL2

* nalgebra

* imgui

* serde

* rfd

* walkdir

## 使い方

ドキュメントはこちら -> [Gitbook](https://lados-organization.gitbook.io/goku/)

## 特徴

* グラフィックス：
    * ウィンドウとメインループ

    * 2Dグラフィックスとテキスト 
        - ボタン 
        - スライダー 
        - チェックボックス
        - テキストボックス
        - フォント（ttf形式のみ）
        - パーティクルシステム（火花）
        - パララックス背景

    * 開発のためのGUIインターフェース
        * 組み込みのテキストエディタ
        * デバッグコンソール

    * 複数の画像ファイルフォーマット：JPGとPNG

* オーディオ
    - 再生
    - ループ
    - 一時停止
    - 再開
    - 複数のオーディオファイルフォーマット：OGG、MP3、FLAC、MOD

* 入力ハンドラ：
    * キーボード、マウス、ゲームパッド入力

* 数学の種類：
    * Vector2、Vector3、Vector4
    * Matrix33、Matrix34、Matrix43、Matrix44

* 物理学：
    * 衝突
    * 剛体（現在はキネマティック剛体はありません）

* シーン：
    * 柔軟なJSONファイル形式：シーン全体または個々のメッシュを記述できます。

* アニメーション

* AIシステム：
    * 行動ツリー

* タイマー

* ダイアログシステム

* 複数の言語のサポート：
    - ドイツ語
    - スペイン語
    - 日本語
    - フランス語

* サポートプラットフォーム：
    - Windows / Mac / Linux
    - Web（WASMは完全に統合されていません）（追加の参照 [Emscripten](https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/) / [SDL-WASM](https://gitlab.com/ThibaultLemaire/rust-sdl-canvas-wasm) / [Web](https://github.com/koute/cargo-web) ）
    - 将来的にはAndroid

## 実行方法

実行コマンド：`cargo run`

**重要！！！** 現在、GUIはまだ開発中です。imguiとsdl2を組み合わせていますが、完全に互換性を持たせるためには時間がかかります。したがって、誰かが使用したい場合、コンポーネント `Scene` を追加し、そこにスクリプトを書くことができます。それらを組み合わせる方法を知っている場合は、本当に素晴らしいことです！

## Web用にビルドする方法

1. ツールを押す
2. ビルドを押す
3. ビルドされたディレクトリで次のコマンドを実行する：`cargo web start wasm32-unknown-emscripten` または `cargo web build --target wasm32-unknown-emscripten`

## 未完了タスク

* エンジンのGUIとテキストエディタの改善

* 単純な図形の描画（円、四角形、三角形など）

* タブ

* プロファイラ

* ビューポート（おそらくsdl2とeguiの組み合わせが必要）

* 物理材料

* 3Dゲームの作成可能性（Vulkanを考慮）

* ライトとシャドウの改善

* パーティクルシステムの追加/改善

* 物理の改善（キネマティック剛体の追加）

* UIシステムの追加/改善

* ボクセル

* ブループリントの追加（おそらくimguiノードグラフ https://github.com/benmkw/imnodes-rs）

* iOS、Android向けのゲームの作成

* コンソール（PS4-5）、Xbox、Nintendo Switch向けのゲームの作成

* C++との統合（おそらくbindgenのようなもの）

## 例

### 1. テトリス

テトリスのゲームを作成する例はこちら -> https://github.com/ladroid

### 2. ローグライクのプロトタイプ（TODO）

ローグライクのプロトタイプの作成例はこちら -> https://github.com/ladroid

### 3. 視覚効果

1. 火花 -> 
2. 火 -> 
3. 雨 -> 

### 4. 横スクロールゲーム

横スクロールのプロトタイプの作成例はこちら -> https://github.com/ladroid

### 5. プラットフォーマー

プラットフォーマーのプロトタイプの作成例はこちら -> https://github.com/ladroid

### 6. 敵のシンプルな状態の設定（追跡/追従）

プラットフォーマーのプロトタイプの作成例はこちら -> https://github.com/ladroid

## 日本語版

日本語版はこちら ->

## フランス語版

フランス語版はこちら ->

## ドイツ語版

ドイツ語版はこちら ->

## スペイン語版

スペイン語版はこちら ->

## コミュニティ

[Discord](https://discord.gg/RDW8f2mv)

[GitHub Discussion](https://docs.github.com/en/discussions/quickstart)

## ライセンス
gokuはApacheライセンスバージョン2.0の下でライセンスされています。[LICENSE](https://pages.github.com/) ファイルを参照してください。
