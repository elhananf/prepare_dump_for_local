use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let input_file = File::open(filename).unwrap();
    let buf = BufReader::new(input_file);
    buf.lines().map(|line| line.unwrap()).collect()
}

pub fn prepare_dump_file(input_path: &PathBuf, output_path: &PathBuf) {
    let input_lines = get_lines_from_file(input_path);
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
    fs::write(output_path, output).unwrap();
}
