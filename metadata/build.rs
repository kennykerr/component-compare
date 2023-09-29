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
        .arg("test.winmd")
        .arg("src/metadata.idl");

    if !command.status().unwrap().success() {
        panic!("Failed to run midlrt");
    }
}
