use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "riscv2llvm")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let input = opt.input;
    let output = opt.output;
    let rv_source = fs::read_to_string(&input).expect("Unable to read the input file");
    let ll_source = riscv2llvm::run(&rv_source);
    let path = output.unwrap_or_else(|| input.with_extension("ll"));
    fs::write(&path, &ll_source).expect("Unable to write the output file");
}
