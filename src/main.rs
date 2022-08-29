use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version)]
struct Args {
    input: PathBuf,

    #[clap(short, long)]
    output: Option<PathBuf>,

    #[clap(short, long, default_value=8)]
    jobs: usize,

    #[clap(long)]
    entry: Option<String>,

    #[clap(long)]
    irs: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let rv_src = fs::read_to_string(&args.input).expect("Unable to read the input file");
    let irs = args
        .irs
        .iter()
        .map(|path| fs::read(path).expect("Unable to read LLVM IR"))
        .collect();
    let ll_src = riscv2llvm::run(&rv_src, &irs, args.jobs);
    let output = arguments
        .output
        .unwrap_or_else(|| args.input.with_extension("ll"));
    fs::write(&output, &ll_src).expect("Unable to write the output file");
}
