use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
#[structopt(
    name = "prepare_dump_for_local",
    about = "Prepares a PostgreSQL DB dump file for local DB restoration"
)]
pub struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    /// Output file
    #[structopt(parse(from_os_str))]
    pub output: PathBuf,
}
