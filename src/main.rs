/// The current version of the CLI. (As defined in Cargo.toml)
pub const LYLE_VERSION: &str = std::env!("CARGO_PKG_VERSION");

pub const HELP_MSG: &str = "Usage: lyle FILE";

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.contains(&"--version".into()) {
        println!("lyle {}", LYLE_VERSION);
        std::process::exit(0);
    } else if args.contains(&"--help".into()) {
        println!("{}", HELP_MSG);
        std::process::exit(0);
    }

    // TODO: -o/--output=FILE and others

    // TODO: Allow compiling multiple files (including headers)

    if args.len() > 1 {
        eprintln!("{}", HELP_MSG);
        eprintln!(
            "Error: Expected 1 argument of FILE but got {} arguments.",
            args.len()
        );
        std::process::exit(1);
    }

    println!("ok bye")
}
