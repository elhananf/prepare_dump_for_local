use structopt::StructOpt;

mod args;
mod prepare;

pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let opt = args::Opt::from_args();
    let args::Opt { input, output } = &opt;

    prepare::prepare_dump_file(input, output)?;

    Ok(())
}
