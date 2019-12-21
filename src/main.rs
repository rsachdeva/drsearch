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

fn find_matches<W: Write>(cli: &Cli, writer: &mut W) -> Result<(), Error> {
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

fn find_matches_trait_object_style(cli: &Cli, writer: &mut dyn Write) -> Result<(), Error> {
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
        find_matches_trait_object_style(&cli, &mut std::io::stdout())
            .expect("Could not complete Search due to: ");
    } else {
        println!("calling genric style (default)");
        find_matches(&cli, &mut std::io::stdout()).expect("Could not complete Search due to: ");
    }
    println!(
        "--------------------------------------------------------------------------------------"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_matches() {
        let mut file = NamedTempFile::new().expect("new file could not be created");
        writeln!(file, "lorem ipsum\ndolor sit amet\nlorem ipsum\n")
            .expect("could not write to file");

        let mut result = Vec::new();
        let cli = Cli {
            pattern: "lorem".to_string(),
            path: PathBuf::from(file.path()),
            style: Option::from(Command::TraitStyle),
        };
        find_matches(&cli, &mut result).expect("could not process find matches");
        assert_eq!(result, b"lorem ipsum\nlorem ipsum\n");
    }

    #[test]
    fn test_matches_trait_object_style() {
        let mut file = NamedTempFile::new().expect("new file could not be created");
        writeln!(file, "lorem ipsum\ndolor sit amet\nlorem ipsum\n")
            .expect("could not write to file");

        let mut result = Vec::new();
        let cli = Cli {
            pattern: "lorem".to_string(),
            path: PathBuf::from(file.path()),
            style: Option::from(Command::TraitStyle),
        };
        find_matches_trait_object_style(&cli, &mut result).expect("could not process find matches");
        assert_eq!(result, b"lorem ipsum\nlorem ipsum\n");
    }
}
