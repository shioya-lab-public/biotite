use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version)]
struct Args {
    input: PathBuf,

    #[clap(short, long)]
    output: Option<PathBuf>,

    #[clap(long)]
    irs: Vec<PathBuf>,

    #[clap(long)]
    arch: String,
}

fn main() {
    let args = Args::parse();
    let rv_src = fs::read_to_string(&args.input).expect("Unable to read the input file");
    let irs = args
        .irs
        .iter()
        .map(|path| fs::read(path).expect("Unable to read LLVM IR"))
        .collect();
    let ll_src = riscv2llvm::run(&rv_src, &irs, &args.arch);
    let output = args
        .output
        .unwrap_or_else(|| args.input.with_extension("ll"));
    fs::write(&output, &ll_src).expect("Unable to write the output file");
}
