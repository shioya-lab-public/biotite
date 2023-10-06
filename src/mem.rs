use crate::llvm_isa::Prog;

pub fn build(arch: &Option<String>, prog: &Prog) -> (Option<String>, Option<String>) {
    let (asm, ld) = match arch.as_deref() {
        Some("x86_64") => (include_str!("mem/x86_64.s"), include_str!("mem/x86_64.ld")),
        Some(arch) => panic!("Unknown architecture `{arch}`"),
        None => return (None, None),
    };
    let mmap_min_addr = 0x11000;
    let bytes: Vec<_> = prog
        .mem
        .iter()
        .skip(mmap_min_addr)
        .map(|b| b.to_string())
        .collect();
    let start = prog.mem.len() + 0x1000;
    let asm = asm
        .replace("{bytes}", &bytes.join(","))
        .replace("{len}", &bytes.len().to_string());
    let ld = ld
        .replace("{start}", &start.to_string())
        .replace("{mmap_min_addr}", &mmap_min_addr.to_string());
    (Some(asm), Some(ld))
}
