use structopt::StructOpt;

mod args;

fn main() {
    let opt = args::Opt::from_args();
    let args::Opt { input, output } = &opt;

    println!("input: {}", input.to_str().unwrap());
    println!("output: {}", output.to_str().unwrap());
}
