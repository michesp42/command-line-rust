use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Michael Espeña <michael.esp42@pm.me>")
        .about("Rust echo")
        .get_matches();
}
