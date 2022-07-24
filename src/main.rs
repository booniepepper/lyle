use lyle::cli::{self, LyleMode};

fn main() {
    let mode = cli::parse();

    match mode {
        LyleMode::AssembleSingleSourceFile(source_path) => {
            println!("Is this your card? {:?}", source_path)
        }
    }
}
