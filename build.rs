extern crate embed_resource;
use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    if target.contains("android") {
        // Build for Android
        println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
        println!("cargo:rustc-link-lib=static=SDL2");
        println!("cargo:rustc-link-lib=static=SDL2main");
        println!("cargo:rustc-link-lib=dylib=GLESv1_CM");
        println!("cargo:rustc-link-lib=dylib=GLESv2");
        println!("cargo:rustc-link-lib=dylib=log");
    } else if target.contains("apple-ios") {
        // Build for iOS
        println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
        println!("cargo:rustc-link-lib=framework=OpenGLES");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=CoreMotion");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=GameController");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=framework=UIKit");
    } else if target.contains("windows") {
        // Windows specific configuration
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=dylib=SDL2");
        embed_resource::compile("config/app.rc", embed_resource::NONE);
    } else if target.contains("darwin") {
        // macOS specific configuration
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=framework=SDL2");
    }
}
