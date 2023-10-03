use windows_bindgen::*;

fn main() {
    println!("cargo:rerun-if-changed=src/metadata.idl");
    let metadata_dir = format!("{}/System32/WinMetadata", env!("windir"));

    let mut command = std::process::Command::new("midlrt.exe");
    command
        .arg("/winrt")
        .arg("/nomidl")
        .arg("/h")
        .arg("nul")
        .arg("/metadata_dir")
        .arg(&metadata_dir)
        .arg("/reference")
        .arg(format!("{metadata_dir}/Windows.Foundation.winmd"))
        .arg("/winmd")
        .arg("server.winmd")
        .arg("src/metadata.idl");

    if !command.status().unwrap().success() {
        panic!("Failed to run midlrt");
    }

    match bindgen(["--etc", "src/bindings.txt"]) {
        Ok(ok) => println!("{}", ok),
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}
