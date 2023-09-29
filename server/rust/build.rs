use windows_bindgen::*;

fn main() {
    match bindgen(["--etc", "src/bindings.txt"]) {
        Ok(ok) => println!("{}", ok),
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}
