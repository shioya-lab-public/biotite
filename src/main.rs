use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(version)]
struct Args {
    input: PathBuf,

    tdata: PathBuf,

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
    srcs: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let rv_src = fs::read_to_string(&args.input).expect("Unable to read the dump file");
    let tdata_src = fs::read_to_string(&args.tdata).expect("Unable to read the tdata file");
    let ir_dir = args.output.unwrap_or(args.input).with_extension("ir");
    if !args.srcs.is_empty() {
        Command::new("rm")
            .arg("-rf")
            .arg(&ir_dir)
            .status()
            .expect("Unable to remove the old src IR directory");
        fs::create_dir(&ir_dir).expect("Unable to create the src IR directory");
    }
    let ll_prog = riscv2llvm::run(
        rv_src,
        tdata_src,
        args.arch,
        args.enable_all_opts,
        args.disable_all_opts,
        args.enable_opts,
        args.disable_opts,
        args.srcs,
        ir_dir.clone(),
    );
    fs::write(ir_dir.with_extension("ll"), ll_prog.to_string())
        .expect("Unable to write the translated IR file");
    if let Some((asm, ld)) = ll_prog.mem {
        fs::write(ir_dir.with_extension("s"), asm).expect("Unable to write the memory asssembly");
        fs::write(ir_dir.with_extension("ld"), ld).expect("Unable to write the linker script");
    }
}
