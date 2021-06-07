use riscv2llvm::run;
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
    let rv = fs::read_to_string(&opt.input).expect("Unable to read the input file.");
    let ll = run(&rv);
    let output = opt
        .output
        .clone()
        .unwrap_or_else(|| opt.input.with_extension("ll"));
    fs::write(&output, &ll).expect("Unable to write the output file.");
}
