use windows_bindgen::*;

fn main() {
    match bindgen(["--etc", "src/bindings.txt"]) {
        Ok(ok) => println!("{}", ok),
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }

    let mut to = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    to.pop();
    to.pop();
    to.push("target");
    to.push(std::env::var("PROFILE").unwrap());

    let mut from = to.clone();
    from.push("deps");
    from.push("test.dll");

    to.push("test.dll");

    std::fs::copy(from, to).unwrap();
}
