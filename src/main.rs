use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
struct Args {
    input: PathBuf,

    tdata: Option<PathBuf>,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(long)]
    arch: Option<String>,

    #[arg(long)]
    enable_all_opts: bool,

    #[arg(long, num_args = 1..)]
    enable_opts: Vec<String>,

    #[arg(long, num_args = 1..)]
    disable_opts: Vec<String>,

    #[arg(long)]
    disable_all_opts: bool,

    #[arg(long, num_args = 1..)]
    src_funcs: Vec<String>,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let rv_src = fs::read_to_string(&args.input).expect("Unable to read the input file");
    let tdata_src = args
        .tdata
        .map(|path| fs::read_to_string(&path).expect("Unable to read the tdata file"));
    let ll_src = riscv2llvm::run(
        rv_src,
        tdata_src,
        args.arch,
        args.enable_all_opts,
        args.enable_opts,
        args.disable_opts,
        args.disable_all_opts,
        args.src_funcs,
    );
    let output = args.output.unwrap_or(args.input).with_extension("ll");
    fs::write(&output, &ll_src).expect("Unable to write the output file");
}
