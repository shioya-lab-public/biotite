use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
struct Args {
    input: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(long, default_value_t = 0)]
    module_size: usize,

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
    let out_dir = args
        .output
        .unwrap_or(args.input.with_extension("translated"));
    if !out_dir.exists() {
        fs::create_dir(&out_dir).expect("Unable to create the output directory");
    }
    let ll_prog = biotite::run(
        rv_src,
        args.arch,
        args.enable_all_opts,
        args.disable_all_opts,
        args.enable_opts,
        args.disable_opts,
        args.srcs,
        out_dir.join("ir"),
        args.module_size,
    );
    let (mk, main, mods) = ll_prog.to_modules();
    fs::write(out_dir.join("Makefile"), mk).expect("Unable to write output files");
    fs::write(out_dir.join("main.ll"), main).expect("Unable to write output files");
    for (i, md) in mods.into_iter().enumerate() {
        fs::write(out_dir.join(format!("{i}.ll")), md).expect("Unable to write output files");
    }
    if let Some((asm, ld)) = ll_prog.mem {
        fs::write(out_dir.join("image.s"), asm).expect("Unable to write output files");
        fs::write(out_dir.join("mapping.ld"), ld).expect("Unable to write output files");
    }
}
