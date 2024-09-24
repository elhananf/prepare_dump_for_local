use std::fmt;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum DumpPrepareError {
    FileRead,
    InputParse,
    FileWrite,
}

impl fmt::Display for DumpPrepareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DumpPrepareError::FileRead => {
                write!(f, "Cannot read from input file")
            }
            DumpPrepareError::InputParse => {
                write!(f, "Could not parse input file lines")
            }
            DumpPrepareError::FileWrite => {
                write!(f, "Cannot write output to file")
            }
        }
    }
}

impl std::error::Error for DumpPrepareError {}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>, DumpPrepareError> {
    let input_file = File::open(filename).map_err(|_| DumpPrepareError::FileRead)?;
    let buf = BufReader::new(input_file);
    buf.lines()
        .map(|line| line.map_err(|_| DumpPrepareError::InputParse))
        .collect()
}

pub fn prepare_dump_file(
    input_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), DumpPrepareError> {
    let input_lines = get_lines_from_file(input_path)?;
    let output_lines: Vec<String> = input_lines
        .iter()
        .map(|line| {
            if line.contains("OWNER TO") || line.contains("GRANT ALL") {
                format!("-- {}", line)
            } else {
                line.clone()
            }
        })
        .collect();
    let output = output_lines.join("\n");
    fs::write(output_path, output).map_err(|_| DumpPrepareError::FileWrite)?;
    Ok(())
}
