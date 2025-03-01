use crate::llvm_isa::Prog;

const MMAP_MIN_ADDR: usize = 0x11000;

pub fn run(arch: &str, prog: &Prog) -> (String, String) {
    let bytes = prog
        .image
        .iter()
        .skip(MMAP_MIN_ADDR)
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(",");
    match arch {
        "x86_64" => (
            format!(
                include_str!("templates/image.s"),
                bytes = bytes,
                size = bytes.len()
            ),
            include_str!("templates/x86_64.ld")
                .replace("{start}", &prog.image.len().to_string())
                .replace("{mmap_min_addr}", &MMAP_MIN_ADDR.to_string()),
        ),
        arch => panic!("Unknown architecture `{arch}`"),
    }
}
