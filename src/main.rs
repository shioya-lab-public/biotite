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

    /// Specify the target architecture for system call support.
    /// Omit it to avoid generating system-call-related functions.
    /// See `src/sys_call` for a list of supported architectures.
    #[arg(long)]
    arch: Option<String>,

    /// At most one of the following four arguments should be set.
    /// Omit them will enable all optimization.
    /// See `src/opt` for a list of supported optimization.
    #[arg(long)]
    enable_all_opts: bool,

    #[arg(long)]
    disable_all_opts: bool,

    #[arg(long, num_args = 1..)]
    enable_opts: Vec<String>,

    #[arg(long, num_args = 1..)]
    disable_opts: Vec<String>,

    /// Specify names of functions that will be substituted by LLVM IR.
    #[arg(long, num_args = 1..)]
    ir_funcs: Vec<String>,

    /// Specify paths to LLVM IR files for substitution.
    #[arg(long, num_args = 1..)]
    ir_files: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let rv_src = fs::read_to_string(&args.input).expect("Unable to read the dump file");
    let tdata_src = args
        .tdata
        .map(|path| fs::read_to_string(path).expect("Unable to read the tdata file"));
    let ir_files = args
        .ir_files
        .iter()
        .map(|path| fs::read_to_string(path).expect("Unable to read IR files"))
        .collect();
    let (ll_src, transed_ir_files) = riscv2llvm::run(
        rv_src,
        tdata_src,
        args.arch,
        args.enable_all_opts,
        args.disable_all_opts,
        args.enable_opts,
        args.disable_opts,
        args.ir_funcs,
        ir_files,
    );
    let output = args.output.unwrap_or(args.input).with_extension("ll");
    fs::write(output, ll_src).expect("Unable to write the translated file");
    args.ir_files
        .iter()
        .map(|path| path.with_extension("transed.ll"))
        .zip(transed_ir_files.iter())
        .for_each(|(path, transed_ir_file)| {
            fs::write(path, transed_ir_file).expect("Unable to write IR files")
        });
}
