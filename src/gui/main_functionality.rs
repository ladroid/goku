use std::path::Path;
use std::io::Write;
use crate::gui::state::State;

#[allow(dead_code)]
pub fn cut_selected_text(text: &str, range: std::ops::Range<usize>) -> (String, String) {
    let selected_text = text[range.clone()].to_string();
    let remaining_text = format!("{}{}", &text[..range.start], &text[range.end..]);
    (selected_text, remaining_text)
}

#[allow(dead_code)]
pub fn execute_code(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = &state.project_dir; // Assuming `project_dir` is stored in state

    // Build the cargo project
    let build_status = std::process::Command::new("cargo")
        .arg("build")
        .current_dir(project_dir)
        .status()?;

    if !build_status.success() {
        state.terminal.log_error("Failed to compile the code");
        return Err("Failed to compile the code".into());
    }

    // Run the built cargo project
    let output = std::process::Command::new("cargo")
        .arg("run")
        .current_dir(project_dir)
        .output()?;

    if !output.status.success() {
        state.terminal.log_error("Failed to execute the code");
        return Err("Failed to execute the code".into());
    }

    // If everything was successful, print the output
    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    state.terminal.log(format!("Output:\n{}", String::from_utf8_lossy(&output.stdout)));

    Ok(())
}

#[allow(dead_code)]
pub fn handle_two_d_module(temp_dir: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let two_d_src = Path::new("src/two_d.rs");
    let two_d_dest = temp_dir.join("src").join("two_d.rs");
    std::fs::copy(&two_d_src, &two_d_dest)?;

    let two_d_dir = temp_dir.join("src").join("two_d");
    std::fs::create_dir_all(&two_d_dir)?;

    let original_two_d_path = std::env::current_dir()?.join("src").join("two_d");
    for entry in std::fs::read_dir(original_two_d_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().ok_or("Failed to get file name")?.to_owned();
            std::fs::copy(&path, two_d_dir.join(filename))?;
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub fn handle_emscripten_module(temp_dir: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let two_d_src = Path::new("src/emscripten.rs");
    let two_d_dest = temp_dir.join("src").join("emscripten.rs");
    std::fs::copy(&two_d_src, &two_d_dest)?;

    let two_d_dir = temp_dir.join("src").join("emscripten");
    std::fs::create_dir_all(&two_d_dir)?;

    let original_two_d_path = std::env::current_dir()?.join("src").join("emscripten");
    for entry in std::fs::read_dir(original_two_d_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().ok_or("Failed to get file name")?.to_owned();
            std::fs::copy(&path, two_d_dir.join(filename))?;
        }
    }
    Ok(())
}

#[cfg(target_os = "windows")]
const EMSDK: &str = "emsdk.bat";
#[cfg(not(target_os = "windows"))]
const EMSDK: &str = "./emsdk";

#[cfg(target_os = "windows")]
const EMSDK_ENV: &str = "emsdk_env.bat";
#[cfg(not(target_os = "windows"))]
const EMSDK_ENV: &str = "source ./emsdk_env.sh";

#[allow(dead_code)]
pub fn execute_code_web(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = &state.project_dir; // Assuming `project_dir` is stored in state
    println!("{:?}", project_dir);

    // Assuming `handle_emscripten_module` is a function that sets up Emscripten
    handle_emscripten_setup(project_dir.to_str().unwrap())?;

    // Set the EMCC_CFLAGS environment variable for SDL2 support
    std::env::set_var("EMCC_CFLAGS", "-s USE_SDL=2");

    // Build the cargo project for WebAssembly
    let build_status = std::process::Command::new("cargo")
        .args(&["build", "--target=asmjs-unknown-emscripten"])
        .current_dir(project_dir)
        .status()?;

    if !build_status.success() {
        state.terminal.log_error("Failed to compile the code for WebAssembly");
        return Err("Failed to compile the code for WebAssembly".into());
    }

    Ok(())
}

fn handle_emscripten_setup(project_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Clone the Emscripten SDK
    std::process::Command::new("git")
        .args(&["clone", "https://github.com/emscripten-core/emsdk.git"])
        .current_dir(project_dir)
        .status()?;

    let emsdk_path = format!("{}/emsdk", project_dir);

    // Enter the EMSDK directory
    std::env::set_current_dir(&emsdk_path)?;

    // Fetch the latest version of the emsdk
    std::process::Command::new("git")
        .args(&["pull"])
        .status()?;

    // Download and install the latest SDK tools
    std::process::Command::new(EMSDK)
        .args(&["install", "3.1.49"])
        .status()?;

    // Make the "latest" SDK "active"
    std::process::Command::new(EMSDK)
        .args(&["activate", "latest"])
        .status()?;

    // Activate PATH and other environment variables in the current terminal
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", EMSDK_ENV])
            .status()?;
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(EMSDK_ENV)
            .status()?;
    }

    Ok(())
}

#[allow(dead_code)]
pub fn build_code(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = &state.project_dir; // Assuming `project_dir` is stored in state

    // Build the cargo project
    let build_status = std::process::Command::new("cargo")
        .arg("build")
        .current_dir(project_dir)
        .status()?;

    if !build_status.success() {
        state.terminal.log_error("Failed to compile the code");
        return Err("Failed to compile the code".into());
    }

    Ok(())
}

#[allow(dead_code)]
pub fn append_dependencies_to_cargo_toml(cargo_toml_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut cargo_toml = std::fs::read_to_string(cargo_toml_path)?;
    cargo_toml.push_str("\nnalgebra = \"0.32.2\"\nsdl2-sys = \"0.35.2\"\nserde = { version = \"1.0\", features = [\"derive\"] }\nserde_json = \"1.0\"\nserde_derive = \"1.0.163\"\nrand = \"0.8.5\"\n[dependencies.sdl2]\nversion = \"0.35\"\ndefault-features = false\nfeatures = [\"image\", \"ttf\", \"mixer\"]\n");
    std::fs::write(cargo_toml_path, cargo_toml)?;
    Ok(())
}

// fn create_web_toml(web_toml_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
//     let web_toml_content = r#"
// default-target = "wasm32-unknown-emscripten"

// [target.emscripten]
// link-args = [
//     "-s", "WASM=1",
//     "-s", "USE_SDL=2",
// ]
// "#;
//     std::fs::write(web_toml_path, web_toml_content)?;
//     Ok(())
// }

#[allow(dead_code)]
pub fn execute_command(command: &str, args: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let status = std::process::Command::new(command)
        .arg(args)
        .status()?;

    if !status.success() {
        return Err(format!("Failed to execute {:?}", args).into());
    }
    Ok(())
}

#[allow(dead_code)]
pub fn copy_directory(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !src.is_dir() {
        return Err("Source is not a directory".into());
    }

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            std::fs::create_dir_all(&dest_path)?;
            copy_directory(&path, &dest_path)?;
        } else {
            std::fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn save_project() -> Option<(String, String)> {
    let dialog = rfd::FileDialog::new()
        .add_filter("SuperCool Project", &["sc"])
        .set_directory(".")
        .set_title("Save Project")
        .save_file(); // Use save_file() instead of save()

    match dialog {
        Some(result) => {
            let path_str = result.into_os_string().into_string().unwrap();
            let sc_path_str = path_str.clone() + ".sc";
            Some((path_str, sc_path_str))
        },
        None => {
            eprintln!("Failed to show save dialog");
            None
        }
    }
}

#[allow(dead_code)]
pub fn save_project_to_path<P: AsRef<Path>>(path: P, state: &State) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    let json = serde_json::to_string_pretty(state)?;
    write!(file, "{}", json)
}

#[allow(dead_code)]
pub fn open_state() -> std::io::Result<Option<State>> {
    let dialog = rfd::FileDialog::new()
        .add_filter("SuperCool Project", &["sc"])
        .set_directory(".")
        .set_title("Open State")
        .pick_file();

    match dialog {
        Some(path) => {
            let file = std::fs::read_to_string(path)?;
            match serde_json::from_str::<State>(&file) {
                Ok(state) => Ok(Some(state)),
                Err(e) => {
                    eprintln!("Failed to parse state file: {}", e);
                    Ok(None)
                },
            }
        },
        None => {
            eprintln!("Failed to show open dialog");
            Ok(None)
        }
    }
}

#[allow(dead_code)]
pub fn is_vscode_installed() -> bool {
    if cfg!(target_os = "windows") {
        // Windows-specific logic
        std::process::Command::new("cmd")
            .args(["/C", "code --version"])
            .output()
            .is_ok()
    } else {
        // Unix-like OS logic
        std::process::Command::new("sh")
            .arg("-c")
            .arg("code --version")
            .output()
            .is_ok()
    }
}

#[allow(dead_code)]
pub fn user_wants_to_save(ui: &imgui::Ui, show_save_dialog: &mut bool) -> Option<bool> {
    let mut result = None;

    if *show_save_dialog {
        ui.open_popup("Save Scene?");
    }

    ui.modal_popup("Save Scene?", || {
        ui.text("Do you want to save the scene?");

        if ui.button("Yes") {
            result = Some(true);
            *show_save_dialog = false;
            ui.close_current_popup();
        }

        ui.same_line();

        if ui.button("No") {
            result = Some(false);
            *show_save_dialog = false;
            ui.close_current_popup();
        }
    });

    result
}