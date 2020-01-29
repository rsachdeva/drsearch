use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Copy, Clone, Debug)]
enum Command {
    TraitStyle,
    GenericStyle,
}

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,
    path: PathBuf,
    #[structopt(subcommand)]
    // makes this command line positional argument optional
    style: Option<Command>,
}

fn find_matches_generic_compile_time_style<W: Write>(
    cli: &Cli,
    writer: &mut W,
) -> Result<(), Error> {
    let f = File::open(&cli.path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let s = line.unwrap();
        if s.contains(cli.pattern.as_str()) {
            writeln!(writer, "{}", s)?;
        }
    }
    Ok(())
}

fn find_matches_trait_object_run_time_style(
    cli: &Cli,
    writer: &mut dyn Write,
) -> Result<(), Error> {
    let f = File::open(&cli.path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let s = line.unwrap();
        if s.contains(cli.pattern.as_str()) {
            writeln!(writer, "{}", s)?
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::from_args();
    println!(
        "--------------------------------------------------------------------------------------"
    );
    let style = &cli.style.unwrap_or(Command::GenericStyle);
    println!("Style is `{:?}`", style);
    if let Command::TraitStyle = style {
        println!("calling trait style");
        find_matches_trait_object_run_time_style(&cli, &mut std::io::stdout())
            .expect("Could not complete Search due to: ");
    } else {
        println!("calling generic style (default)");
        find_matches_generic_compile_time_style(&cli, &mut std::io::stdout())
            .expect("Could not complete Search due to: ");
    }
    println!(
        "--------------------------------------------------------------------------------------"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    const COULD_NOT_PROCESS: &str = "could not process find matches";
    const EXPECTED_RESULT: &[u8; 24] = b"lorem ipsum\nlorem ipsum\n";

    fn tmp_named_file() -> NamedTempFile {
        NamedTempFile::new().expect("new file could not be created")
    }

    fn setup_options(command_style: Command, file: &mut NamedTempFile) -> (Vec<u8>, Cli) {
        writeln!(file, "lorem ipsum\ndolor sit amet\nlorem ipsum\n")
            .expect("could not write to file");
        let result = Vec::new();
        let cli = Cli {
            pattern: str::to_string("lorem"),
            path: PathBuf::from(file.path()),
            style: Option::from(command_style),
        };
        (result, cli)
    }

    #[test]
    fn test_matches_generic_compile_time_style() {
        let mut file = tmp_named_file();
        let (mut result, cli) = setup_options(Command::GenericStyle, &mut file);

        find_matches_generic_compile_time_style(&cli, &mut result).expect(COULD_NOT_PROCESS);
        file.close().expect("file could not be closed");
        assert_eq!(result, EXPECTED_RESULT);
    }

    #[test]
    fn test_matches_trait_object_run_time_style() {
        let mut file = tmp_named_file();
        let (mut result, cli) = setup_options(Command::TraitStyle, &mut file);

        find_matches_trait_object_run_time_style(&cli, &mut result).expect(COULD_NOT_PROCESS);
        file.close().expect("new file could not be closed");
        assert_eq!(result, EXPECTED_RESULT);
    }
}
