use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version)]
struct Args {
    input: PathBuf,

    #[clap(long)]
    tdata: Option<PathBuf>,

    #[clap(long)]
    arch: String,

    #[clap(long)]
    irs: Vec<PathBuf>,

    #[clap(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let rv_src = fs::read_to_string(&args.input).expect("Unable to read the input file");
    let tdata_src = args
        .tdata
        .map(|path| fs::read_to_string(&path).expect("Unable to read the input `.tdata` file"));
    let irs = args
        .irs
        .iter()
        .map(|path| fs::read(path).expect("Unable to read LLVM IR"))
        .collect();
    let ll_src = riscv2llvm::run(&args.arch, &rv_src, &tdata_src, &irs);
    let output = args
        .output
        .unwrap_or_else(|| args.input.with_extension("ll"));
    fs::write(&output, &ll_src).expect("Unable to write the output file");
}
