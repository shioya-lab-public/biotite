use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(version)]
struct Args {
    input: PathBuf,

    tdata: Option<PathBuf>,

    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Specify the target architecture for system call support.
    /// Omit it to avoid generating system-call-related functions.
    /// See `src/sys_call` for a list of supported architectures.
    #[arg(long)]
    sys_call: Option<String>,

    /// Specify the target architecture for direct memory access.
    /// Omit it to use slower but ISA-agnostic indirect memory access.
    /// See `src/mem` for a list of supported architectures.
    #[arg(long)]
    mem: Option<String>,

    /// At most one of the following four arguments should be set.
    /// Omit them will enable all optimization.
    /// See `src/opt` for a list of supported optimization passes.
    #[arg(long)]
    enable_all_opts: bool,

    #[arg(long)]
    disable_all_opts: bool,

    #[arg(long, num_args = 1..)]
    enable_opts: Vec<String>,

    #[arg(long, num_args = 1..)]
    disable_opts: Vec<String>,

    /// Specify directories containing source code.
    #[arg(long, num_args = 1..)]
    srcs: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let rv_src = fs::read_to_string(&args.input).expect("Unable to read the dump file");
    let tdata_src = args
        .tdata
        .map(|path| fs::read_to_string(path).expect("Unable to read the tdata file"));
    let ir_dir = args.output.unwrap_or(args.input).with_extension("ir");
    if !args.srcs.is_empty() {
        let _ = Command::new("rm").arg("-rf").arg(&ir_dir).status();
        fs::create_dir(&ir_dir).expect("Unable to create the IR directory");
    }
    let ll_prog = riscv2llvm::run(
        rv_src,
        tdata_src,
        args.sys_call,
        args.mem,
        args.enable_all_opts,
        args.disable_all_opts,
        args.enable_opts,
        args.disable_opts,
        args.srcs,
        ir_dir.clone(),
    );
    fs::write(ir_dir.with_extension("ll"), ll_prog.to_string())
        .expect("Unable to write the translated file");
    if let Some(mem_s) = ll_prog.mem_s {
        fs::write(ir_dir.with_extension("s"), mem_s).expect("Unable to write the translated file");
    }
    if let Some(mem_ld) = ll_prog.mem_ld {
        fs::write(ir_dir.with_extension("ld"), mem_ld)
            .expect("Unable to write the translated file");
    }
}
