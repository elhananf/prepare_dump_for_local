use structopt::StructOpt;

mod args;
mod prepare;

fn main() {
    let opt = args::Opt::from_args();
    let args::Opt { input, output } = &opt;

    prepare::prepare_dump_file(input, output);
}
