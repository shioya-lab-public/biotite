use clap::Parser;
use regex::Regex;
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

    #[arg(long)]
    disable_all_opts: bool,

    #[arg(long, num_args = 1..)]
    enable_opts: Vec<String>,

    #[arg(long, num_args = 1..)]
    disable_opts: Vec<String>,

    #[arg(long, num_args = 1..)]
    src_funcs: Vec<String>,

    #[arg(long)]
    no_opaque_pointers: bool,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let rv_src = fs::read_to_string(&args.input).expect("Unable to read the input file");
    let tdata_src = args
        .tdata
        .map(|path| fs::read_to_string(&path).expect("Unable to read the tdata file"));
    let mut ll_src = riscv2llvm::run(
        rv_src,
        tdata_src,
        args.arch,
        args.enable_all_opts,
        args.disable_all_opts,
        args.enable_opts,
        args.disable_opts,
        args.src_funcs,
    );
    if !args.no_opaque_pointers {
        let ptr = Regex::new(r"i8\*\*|i8\*|i16\*|i32\*|i64\*|double\*").unwrap();
        ll_src = ptr.replace_all(&ll_src, "ptr").to_string();
    }
    let output = args.output.unwrap_or(args.input).with_extension("ll");
    fs::write(&output, &ll_src).expect("Unable to write the output file");
}
