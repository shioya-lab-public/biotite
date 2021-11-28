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

    #[structopt(name = "mabi", short, long)]
    abi: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    let rv_source = fs::read_to_string(&opt.input).expect("Unable to read the input file");
    let ll_source = riscv2llvm::run(&rv_source, &opt.abi);
    let path = opt.output.unwrap_or_else(|| opt.input.with_extension("ll"));
    fs::write(path, ll_source).expect("Unable to write the output file");
}
