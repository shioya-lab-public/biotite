use crate::riscv_isa::*;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::mem;

static SECTION_SIZE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s+\S+\s+(\S+)\s+([[:xdigit:]]+)\s+([[:xdigit:]]+)").unwrap());
static SYMBOL_SIZE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([[:xdigit:]]+)\s+\S+\s+(\S+\s+)?\S+\s+([[:xdigit:]]+)\s+(\.hidden\s+)?(\S+)?")
        .unwrap()
});
static SECTION: Lazy<Regex> = Lazy::new(|| Regex::new(r"Disassembly of section (\S+):").unwrap());
static SYMBOL: Lazy<Regex> = Lazy::new(|| Regex::new(r"([[:xdigit:]]+) <(\S+)>:").unwrap());
static BYTES: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s+[[:xdigit:]]+:\s+(([[:xdigit:]][[:xdigit:]] )+)").unwrap());
static CONTENTS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+([[:xdigit:]]+)\s+(.{35})").unwrap());

pub fn run(src: String, tdata: Option<String>) -> Program {
    let mut src = src.lines();
    let entry = parse_entry(&mut src);
    let sections = parse_sections(&mut src);
    let symbols = parse_symbols(&mut src);
    let (mut data_blocks, code_blocks) = parse_assembly(&mut src);
    expand_data_blocks(&mut data_blocks, &sections, &symbols);
    let tdata_addr = match tdata {
        Some(tdata) => {
            let (tdata_addr, tdata_block) = parse_tdata(tdata);
            data_blocks.push(tdata_block);
            data_blocks.sort_unstable_by_key(|b| b.address);
            tdata_addr
        }
        None => Addr(0),
    };
    Program {
        entry,
        tdata: tdata_addr,
        data_blocks,
        code_blocks,
        symbols: symbols
            .into_iter()
            .map(|((name, addr), (_, is_func))| ((name, addr), is_func))
            .collect(),
    }
}

fn parse_entry<'a>(src: &mut impl Iterator<Item = &'a str>) -> Addr {
    let mut src = src.skip(3);
    Addr::new(
        src.next()
            .expect("Missing file header")
            .strip_prefix("start address: 0x")
            .expect("Invalid file header"),
    )
}

fn parse_sections<'a>(src: &mut impl Iterator<Item = &'a str>) -> HashMap<(String, Addr), usize> {
    let mut src = src.skip(4);
    let mut sections = HashMap::new();
    while let Some(caps) = SECTION_SIZE.captures(src.next().expect("Missing section header")) {
        let section = (String::from(&caps[1]), Addr::new(&caps[3]));
        let size = usize::from_str_radix(&caps[2], 16).unwrap();
        sections.insert(section, size);
    }
    sections
}

fn parse_symbols<'a>(
    src: &mut impl Iterator<Item = &'a str>,
) -> HashMap<(String, Addr), (usize, bool)> {
    let mut src = src.skip(1);
    let mut symbols = HashMap::new();
    while let Some(caps) = SYMBOL_SIZE.captures(src.next().expect("Missing symbol table")) {
        let name = caps.get(5).map(|m| m.as_str()).unwrap_or_default();
        let symbol = (String::from(name), Addr::new(&caps[1]));
        let size = usize::from_str_radix(&caps[3], 16).unwrap();
        let is_func = caps.get(2).map(|m| m.as_str()).unwrap_or_default().trim();
        symbols.insert(symbol, (size, is_func == "F"));
    }
    symbols
}

fn parse_assembly<'a>(src: &mut impl Iterator<Item = &'a str>) -> (Vec<DataBlock>, Vec<CodeBlock>) {
    let mut data_blocks = Vec::new();
    let mut code_blocks = Vec::new();
    let mut section = String::new();
    let mut symbol = String::new();
    let mut address = Addr(0);
    let mut lines = Vec::new();
    for line in src.filter(|l| !l.is_empty()) {
        if let Some(caps) = SYMBOL.captures(line) {
            let sym = caps[2].to_string();
            let addr = Addr::new(&caps[1]);
            let raw_block = (
                section.clone(),
                mem::replace(&mut symbol, sym),
                mem::replace(&mut address, addr),
                mem::take(&mut lines),
            );
            if raw_block.3.is_empty() {
                continue;
            } else if raw_block.0 == ".text" {
                code_blocks.push(raw_block);
            } else if raw_block.2 != Addr(0) {
                data_blocks.push(raw_block);
            }
        } else if let Some(caps) = SECTION.captures(line) {
            let sec = caps[1].to_string();
            let raw_block = (
                mem::replace(&mut section, sec),
                mem::take(&mut symbol),
                address,
                mem::take(&mut lines),
            );
            if raw_block.3.is_empty() {
                continue;
            } else if raw_block.0 == ".text" {
                code_blocks.push(raw_block);
            } else if raw_block.2 != Addr(0) {
                data_blocks.push(raw_block);
            }
        } else {
            lines.push(line);
        }
    }
    let raw_block = (section, symbol, address, lines);
    if raw_block.0 == ".text" {
        code_blocks.push(raw_block);
    } else if raw_block.2 != Addr(0) {
        data_blocks.push(raw_block);
    }
    (
        data_blocks.into_par_iter().map(parse_data_block).collect(),
        code_blocks.into_par_iter().map(parse_code_block).collect(),
    )
}

fn parse_data_block(
    (section, symbol, address, lines): (String, String, Addr, Vec<&str>),
) -> DataBlock {
    let mut bytes = Vec::new();
    if lines.len() > 1 || lines[0] != "..." {
        for line in lines {
            let caps = BYTES.captures(line).unwrap();
            for byte_str in caps[1].split(' ').filter(|s| !s.is_empty()) {
                let byte = u8::from_str_radix(byte_str, 16)
                    .unwrap_or_else(|_| panic!("Invalid byte `{byte_str}`"));
                bytes.push(byte);
            }
        }
    }
    DataBlock {
        section,
        symbol,
        address,
        bytes,
    }
}

fn parse_code_block(
    (section, symbol, address, lines): (String, String, Addr, Vec<&str>),
) -> CodeBlock {
    CodeBlock {
        section,
        symbol,
        address,
        insts: lines.into_iter().map(Inst::new).collect(),
    }
}

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

fn parse_tdata(tdata: String) -> (Addr, DataBlock) {
    let lines = tdata.lines().skip(3).filter(|l| !l.is_empty());
    let mut addr = None;
    let mut bytes = Vec::new();
    for line in lines {
        let caps = CONTENTS.captures(line).unwrap();
        if addr.is_none() {
            addr = Some(Addr(u64::from_str_radix(&caps[1], 16).unwrap()));
        }
        let byte_str: String = caps[2].chars().filter(|c| !c.is_whitespace()).collect();
        let mut i = 0;
        while i < byte_str.len() {
            let byte = u8::from_str_radix(&byte_str[i..i + 2], 16).unwrap();
            bytes.push(byte);
            i += 2;
        }
    }
    if let Some(addr) = addr {
        (
            addr,
            DataBlock {
                section: String::from(".tdata"),
                symbol: String::from(".tdata"),
                address: addr,
                bytes,
            },
        )
    } else {
        panic!("Empty `.tdata`");
    }
}
