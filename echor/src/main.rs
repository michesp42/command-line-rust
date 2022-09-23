use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Michael Espeña <michael.esp42@pm.me>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .value_parser(clap::value_parser!(std::ffi::OsString))
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.contains_id("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
