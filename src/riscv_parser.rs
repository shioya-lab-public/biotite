use crate::riscv_isa::*;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::mem;
use std::sync::LazyLock;

static SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\S+)\s+([[:xdigit:]]+) ([[:xdigit:]]+)").unwrap());
static SYMBOL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([[:xdigit:]]+)\s+\S+\s+(\S+\s+)?\S+\s+([[:xdigit:]]+)\s+(\.hidden\s+)?(\S+)?")
        .unwrap()
});
static INST_SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Disassembly of section (\S+):").unwrap());
static INST_SYMBOL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([[:xdigit:]]+) <(\S+)>:").unwrap());
static BYTE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r":\s+(([[:xdigit:]][[:xdigit:]] )+)").unwrap());

pub fn run(mut src: String) -> (Prog, HashMap<String, Vec<Addr>>) {
    // Make sure the last block is properly processed.
    src.push('\n');

    let mut lines = src.lines();
    let entry = parse_entry(&mut lines);
    let sections = parse_sections(&mut lines);
    let symbols = parse_symbols(&mut lines);
    let (mut data_blocks, code_blocks) = parse_disassembly(&mut lines);
    fill_data_blocks(&mut data_blocks, &sections, &symbols);
    let tdata = data_blocks
        .iter()
        .find(|block| block.section == ".tdata")
        .map(|block| {
            (
                block.address,
                sections[&(String::from(".tdata"), block.address)],
            )
        });
    let func_syms = symbols
        .clone()
        .into_iter()
        .filter_map(|((_, addr), (_, is_func))| if is_func { Some(addr) } else { None })
        .collect();
    let mut syms = HashMap::new();
    for ((name, addr), (_, _)) in symbols {
        syms.entry(name).or_insert_with(Vec::new).push(addr);
    }
    (
        Prog {
            entry,
            data_blocks,
            code_blocks,
            tdata,
            func_syms,
        },
        syms,
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
    while let Some(caps) =
        SECTION.captures(lines.next().expect("EOF while parsing section headers"))
    {
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
    while let Some(caps) =
        SYMBOL.captures(lines.next().expect("EOF while parsing the symbol table"))
    {
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
        if let Some(caps) = INST_SECTION.captures(line) {
            section = caps[1].to_string();
        } else if let Some(caps) = INST_SYMBOL.captures(line) {
            address = Addr::new(&caps[1]);
            symbol = caps[2].to_string();
        } else if line.is_empty() {
            let block = (
                mem::replace(&mut address, Addr(0)),
                section.clone(),
                mem::take(&mut symbol),
                mem::take(&mut insts),
            );
            if block.2.is_empty() {
                // This is caused by the empty line after the section name line
                continue;
            } else if block.1 == ".text" {
                code_blocks.push(block);
            } else if block.0 != Addr(0) {
                // Keep only meaningful data blocks.
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
            let caps = BYTE.captures(inst).unwrap();
            bytes.extend(
                caps[1]
                    .split_whitespace()
                    .map(|s| u8::from_str_radix(s, 16).unwrap()),
            );
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

// Fill symbols whose disassembly is simply `...` with 0s.
fn fill_data_blocks(
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
                        None => panic!("Unknown symbol at {address}"),
                    }
                }
            };
            data_block.bytes = vec![0; size];
        }
    }
}
