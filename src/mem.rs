use crate::llvm_isa::Prog;

pub fn run(arch: &str, prog: &Prog) -> (String, String) {
    let (asm, ld) = match arch {
        "x86_64" => (include_str!("mem/x86_64.s"), include_str!("mem/x86_64.ld")),
        arch => panic!("Unknown architecture `{arch}`"),
    };
    let mmap_min_addr = 0x11000;
    let bytes: Vec<_> = prog
        .image
        .iter()
        .skip(mmap_min_addr)
        .map(|b| b.to_string())
        .collect();
    let asm = asm
        .replace("{bytes}", &bytes.join(","))
        .replace("{size}", &bytes.len().to_string());
    let ld = ld
        .replace("{start}", &prog.image.len().to_string())
        .replace("{mmap_min_addr}", &mmap_min_addr.to_string());
    (asm, ld)
}
