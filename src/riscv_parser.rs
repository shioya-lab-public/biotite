use crate::riscv_isa::*;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::mem;

static SECTION_SIZE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\S+\s+(\S+)\s+([[:xdigit:]]+)\s+([[:xdigit:]]+)").unwrap());
static SYMBOL_SIZE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([[:xdigit:]]+)\s+\S+\s+(\S+\s+)?\S+\s+([[:xdigit:]]+)\s+(\.hidden\s+)?(\S+)?")
        .unwrap()
});
static SECTION: Lazy<Regex> = Lazy::new(|| Regex::new(r"Disassembly of section (\S+):").unwrap());
static SYMBOL: Lazy<Regex> = Lazy::new(|| Regex::new(r"([[:xdigit:]]+) <(\S+)>:").unwrap());
static BYTES: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[[:xdigit:]]+:\s+(([[:xdigit:]][[:xdigit:]] )+)").unwrap());
static TDATA: Lazy<Regex> = Lazy::new(|| Regex::new(r"([[:xdigit:]]+)\s+(.{35})").unwrap());

pub fn run(mut src: String, tdata: Option<String>) -> (Prog, HashMap<String, Addr>) {
    // Make sure the last block is properly processed
    src.push('\n');

    let mut lines = src.lines();
    let entry = parse_entry(&mut lines);
    let sections = parse_sections(&mut lines);
    let mut symbols = parse_symbols(&mut lines);
    let (mut data_blocks, mut code_blocks) = parse_disassembly(&mut lines);
    expand_data_blocks(&mut data_blocks, &sections, &symbols);
    split_load_gp(&mut code_blocks, &mut symbols);
    let tdata = tdata.map(|tdata| {
        let (tdata_addr, tdata_len, tdata_block) = parse_tdata(tdata);
        data_blocks.push(tdata_block);
        data_blocks.sort_unstable_by_key(|b| b.address);
        (tdata_addr, tdata_len)
    });
    let func_syms = symbols
        .clone()
        .into_iter()
        .filter_map(|((name, addr), (_, is_func))| if is_func { Some((name, addr)) } else { None })
        .collect();

    (
        Prog {
            entry,
            tdata,
            data_blocks,
            code_blocks,
            func_syms,
        },
        symbols
            .into_iter()
            .map(|((name, addr), (_, _))| (name, addr))
            .collect(),
    )
}

fn parse_entry<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Addr {
    Addr::new(
        lines
            .nth(3)
            .expect("Missing file headers")
            .strip_prefix("start address: 0x")
            .expect("Invalid file headers"),
    )
}

fn parse_sections<'a>(lines: &mut impl Iterator<Item = &'a str>) -> HashMap<(String, Addr), usize> {
    let mut lines = lines.skip(4);
    let mut sections = HashMap::new();
    while let Some(caps) = SECTION_SIZE.captures(
        lines
            .next()
            .expect("Section headers should end with an empty line"),
    ) {
        let section = (String::from(&caps[1]), Addr::new(&caps[3]));
        let size = usize::from_str_radix(&caps[2], 16).unwrap();
        sections.insert(section, size);
    }
    sections
}

fn parse_symbols<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
) -> HashMap<(String, Addr), (usize, bool)> {
    let mut lines = lines.skip(1);
    let mut symbols = HashMap::new();
    while let Some(caps) = SYMBOL_SIZE.captures(
        lines
            .next()
            .expect("The symbol table should end with an empty line"),
    ) {
        let addr = Addr::new(&caps[1]);
        let is_func = caps
            .get(2)
            .map(|m| m.as_str().trim() == "F")
            .unwrap_or_default();
        let size = usize::from_str_radix(&caps[3], 16).unwrap();
        let name = caps
            .get(5)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        symbols.insert((name, addr), (size, is_func));
    }
    symbols
}

fn parse_disassembly<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
) -> (Vec<DataBlock>, Vec<CodeBlock>) {
    let mut data_blocks = Vec::new();
    let mut code_blocks = Vec::new();
    let mut section = String::new();
    let mut symbol = String::new();
    let mut address = Addr(0);
    let mut insts = Vec::new();
    for line in lines {
        if let Some(caps) = SECTION.captures(line) {
            section = caps[1].to_string();
        } else if let Some(caps) = SYMBOL.captures(line) {
            address = Addr::new(&caps[1]);
            symbol = caps[2].to_string();
        } else if line.is_empty() {
            let block = (
                address,
                section.clone(),
                mem::take(&mut symbol),
                mem::take(&mut insts),
            );
            if block.3.is_empty() {
                continue;
            } else if block.1 == ".text" {
                code_blocks.push(block);
            } else if block.0 != Addr(0) {
                data_blocks.push(block);
            }
        } else {
            insts.push(line);
        }
    }
    (
        data_blocks.into_par_iter().map(parse_data_block).collect(),
        code_blocks.into_par_iter().map(parse_code_block).collect(),
    )
}

fn parse_data_block(
    (address, section, symbol, insts): (Addr, String, String, Vec<&str>),
) -> DataBlock {
    let mut bytes = Vec::new();
    if insts[0] != "..." {
        for inst in insts {
            let caps = BYTES.captures(inst).unwrap();
            for byte_str in caps[1].split(' ').filter(|s| !s.is_empty()) {
                let byte = u8::from_str_radix(byte_str, 16).unwrap();
                bytes.push(byte);
            }
        }
    }
    DataBlock {
        address,
        section,
        symbol,
        bytes,
    }
}

fn parse_code_block(
    (address, section, symbol, insts): (Addr, String, String, Vec<&str>),
) -> CodeBlock {
    CodeBlock {
        address,
        section,
        symbol,
        insts: insts.into_iter().map(Inst::new).collect(),
    }
}

/// Expand symbols whose disassembly is simply `...` to their correct length
fn expand_data_blocks(
    data_blocks: &mut Vec<DataBlock>,
    sections: &HashMap<(String, Addr), usize>,
    symbols: &HashMap<(String, Addr), (usize, bool)>,
) {
    for data_block in data_blocks {
        if data_block.bytes.is_empty() {
            let symbol = data_block.symbol.clone();
            let address = data_block.address;
            let size = match symbols.get(&(symbol, address)) {
                Some((size, _)) => *size,
                None => {
                    let section = data_block.section.clone();
                    match sections.get(&(section, address)) {
                        Some(usize) => *usize,
                        None => unreachable!(),
                    }
                }
            };
            data_block.bytes = vec![0; size];
        }
    }
}

/// Recover the `load_gp` function if it is merged into the `_start` function
fn split_load_gp(
    code_blocks: &mut Vec<CodeBlock>,
    symbols: &mut HashMap<(String, Addr), (usize, bool)>,
) {
    if let Some(start_pos) = code_blocks
        .iter()
        .position(|block| block.symbol == "_start")
    {
        let start = &mut code_blocks[start_pos];
        if let Inst::Jal { addr, .. } | Inst::PseudoJal { addr, .. } = start.insts[0] {
            if let Some(pos) = start.insts.iter().position(|inst| inst.address() == addr) {
                let load_gp = CodeBlock {
                    section: String::from(".text"),
                    symbol: String::from("load_gp"),
                    address: addr,
                    insts: start.insts.split_off(pos),
                };
                code_blocks.insert(start_pos + 1, load_gp);
                symbols.insert((String::from("load_gp"), addr), (16, true));
            }
        }
    }
}

fn parse_tdata(tdata: String) -> (Addr, usize, DataBlock) {
    let lines = tdata.lines().skip(3);
    let mut address = None;
    let mut bytes = Vec::new();
    for line in lines {
        let caps = TDATA.captures(line).unwrap();
        let addr = Addr(u64::from_str_radix(&caps[1], 16).unwrap());
        address = address.or(Some(addr));
        let byte_str: String = caps[2].chars().filter(|c| !c.is_whitespace()).collect();
        let mut i = 0;
        while i < byte_str.len() {
            let byte = u8::from_str_radix(&byte_str[i..i + 2], 16).unwrap();
            bytes.push(byte);
            i += 2;
        }
    }
    if let Some(address) = address {
        (
            address,
            bytes.len(),
            DataBlock {
                section: String::from(".tdata"),
                symbol: String::from(".tdata"),
                address,
                bytes,
            },
        )
    } else {
        panic!("The `tdata` section is empty");
    }
}
