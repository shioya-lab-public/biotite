use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version)]
struct Arguments {
    input: PathBuf,

    #[clap(short, long)]
    output: Option<PathBuf>,

    #[clap(long, name = "auto-split-functions")]
    auto_split_functions: bool,

    #[clap(long)]
    elf: Option<PathBuf>,
}

fn main() {
    let arguments = Arguments::parse();
    let rv_source = fs::read_to_string(&arguments.input).expect("Unable to read the input file");
    let elf = arguments.elf.map(|path| {
        fs::read_to_string(&path).expect("Unable to read the ELF information of the input file")
    });
    let ll_source = riscv2llvm::run(&rv_source, arguments.auto_split_functions, &elf);
    let ll_path = arguments
        .output
        .unwrap_or_else(|| arguments.input.with_extension("ll"));
    fs::write(&ll_path, &ll_source).expect("Unable to write the output file");
}
