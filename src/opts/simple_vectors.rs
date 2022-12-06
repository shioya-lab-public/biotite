use crate::llvm_isa as ll;
use crate::riscv_isa as rv;
use std::collections::HashSet;

const THRESHOLD: usize = 2;

pub fn simple_vectors(mut prog: ll::Program) -> ll::Program {
    for func in &mut prog.funcs {
        if func.dynamic {
            continue;
        }

        let targets = find_jump_targets(func);
        let mut stat = (String::new(), 0);
        let mut spans = Vec::new();
        for (i, block) in func.inst_blocks.iter().enumerate() {
            let id = get_inst_type_id(&block.rv_inst);
            if targets.contains(&block.rv_inst.address()) || id != stat.0 || i - stat.1 == 4 {
                if i - stat.1 >= THRESHOLD {
                    spans.push((stat.0, stat.1, i));
                }
                stat = (id, i);
            }
        }
        let len = func.inst_blocks.len();
        if len - stat.1 >= THRESHOLD {
            spans.push((stat.0, stat.1, len));
        }

        for (id, start, end) in spans {
            match id.as_str() {
                "Addi" => {
                    let mut rds = HashSet::new();
                    let mut overlap = false;
                    for block in &func.inst_blocks[start..end] {
                        let ll::Inst::Load { ptr:rs, .. } = block.insts.first().unwrap().clone()else {unreachable!()};
                        if rds.contains(&rs) {
                            overlap = true;
                        }
                        let ll::Inst::Store { ptr:rd, .. } = block.insts.last().unwrap().clone() else {unreachable!()};
                        rds.insert(rd);
                    }
                    if overlap {
                        continue;
                    }

                    let mut ops = Vec::new();
                    let mut stores = Vec::new();
                    for block in &mut func.inst_blocks[start..end] {
                        stores.push(block.insts.pop().unwrap());
                        ops.push(block.insts.pop().unwrap());
                    }
                    let mut va = Vec::new();
                    let mut vb = Vec::new();
                    let mut vav = ll::Value::Undef;
                    let mut vbv = ll::Value::Undef;
                    for (i, op) in ops.into_iter().enumerate() {
                        let ll::Inst::Add { rslt, ty, op1, op2 } = op else {unreachable!()};
                        let inst = ll::Inst::Insertelement {
                            rslt,
                            ty: ll::Type::Vector(end - start, Box::new(ty.clone())),
                            val: vav,
                            elt: op1,
                            idx: i,
                        };
                        va.push(inst);
                        vav = rslt;
                        let ll::Value::Temp(addr, _) = rslt else {unreachable!()};
                        let rslt = ll::Value::Temp(addr, 2);
                        let inst = ll::Inst::Insertelement {
                            rslt,
                            ty: ll::Type::Vector(end - start, Box::new(ty.clone())),
                            val: vbv,
                            elt: op2,
                            idx: i,
                        };
                        vb.push(inst);
                        vbv = rslt;
                    }
                    let ll::Inst::Store{val: ll::Value::Temp(addr, _),ty,..} = stores.last().unwrap() else {unreachable!()};
                    let vop_rslt = ll::Value::Temp(*addr, 3);
                    let vop = ll::Inst::Add {
                        rslt: vop_rslt,
                        ty: ll::Type::Vector(end - start, Box::new(ty.clone())),
                        op1: vav,
                        op2: vbv,
                    };
                    let mut vc = Vec::new();
                    let mut vd = Vec::new();
                    for (i, op) in stores.into_iter().enumerate() {
                        let ll::Inst::Store { ty, val, ptr } = op else {unreachable!()};
                        let ll::Value::Temp(addr, _) = val else {unreachable!()};
                        let rslt = ll::Value::Temp(addr, 4);
                        let inst = ll::Inst::Extractelement {
                            rslt,
                            ty: ll::Type::Vector(end - start, Box::new(ty.clone())),
                            val: vop_rslt,
                            idx: i,
                        };
                        vc.push(inst);
                        let inst = ll::Inst::Store { ty, val: rslt, ptr };
                        vd.push(inst);
                    }
                    func.inst_blocks[end - 1].insts.extend(va);
                    func.inst_blocks[end - 1].insts.extend(vb);
                    func.inst_blocks[end - 1].insts.push(vop);
                    func.inst_blocks[end - 1].insts.extend(vc);
                    func.inst_blocks[end - 1].insts.extend(vd);
                }
                "Addiw" => {
                    let mut rds = HashSet::new();
                    let mut overlap = false;
                    for block in &func.inst_blocks[start..end] {
                        let ll::Inst::Load { ptr:rs, .. } = block.insts.first().unwrap().clone()else {unreachable!()};
                        if rds.contains(&rs) {
                            overlap = true;
                        }
                        let ll::Inst::Store { ptr:rd, .. } = block.insts.last().unwrap().clone() else {unreachable!()};
                        rds.insert(rd);
                    }
                    if overlap {
                        continue;
                    }

                    let mut ops = Vec::new();
                    let mut truncs = Vec::new();
                    let mut sexts = Vec::new();
                    let mut stores = Vec::new();
                    for block in &mut func.inst_blocks[start..end] {
                        stores.push(block.insts.pop().unwrap());
                        sexts.push(block.insts.pop().unwrap());
                        truncs.push(block.insts.pop().unwrap());
                        ops.push(block.insts.pop().unwrap());
                    }
                    let mut va = Vec::new();
                    let mut vb = Vec::new();
                    let mut vav = ll::Value::Undef;
                    let mut vbv = ll::Value::Undef;
                    for (i, op) in ops.into_iter().enumerate() {
                        let ll::Inst::Add { rslt, ty, op1, op2 } = op else {unreachable!()};
                        let inst = ll::Inst::Insertelement {
                            rslt,
                            ty: ll::Type::Vector(end - start, Box::new(ty.clone())),
                            val: vav,
                            elt: op1,
                            idx: i,
                        };
                        va.push(inst);
                        vav = rslt;
                        let ll::Value::Temp(addr, _) = rslt else {unreachable!()};
                        let rslt = ll::Value::Temp(addr, 2);
                        let inst = ll::Inst::Insertelement {
                            rslt,
                            ty: ll::Type::Vector(end - start, Box::new(ty.clone())),
                            val: vbv,
                            elt: op2,
                            idx: i,
                        };
                        vb.push(inst);
                        vbv = rslt;
                    }
                    let ll::Inst::Store{val: ll::Value::Temp(addr, _),ty,..} = stores.last().unwrap() else {unreachable!()};
                    let vop_rslt = ll::Value::Temp(*addr, 3);
                    let vop = ll::Inst::Add {
                        rslt: vop_rslt,
                        ty: ll::Type::Vector(end - start, Box::new(ty.clone())),
                        op1: vav,
                        op2: vbv,
                    };
                    let mut vc = Vec::new();
                    let mut vd = Vec::new();
                    let mut ve = Vec::new();
                    let mut vf = Vec::new();
                    for (i, (trunc, (sext, store))) in truncs
                        .into_iter()
                        .zip(sexts.into_iter().zip(stores.into_iter()))
                        .enumerate()
                    {
                        let ll::Inst::Store { ty, val, ptr } = store else {unreachable!()};
                        let ll::Value::Temp(addr, _) = val else {unreachable!()};
                        let rslt = ll::Value::Temp(addr, 4);
                        let inst = ll::Inst::Extractelement {
                            rslt,
                            ty: ll::Type::Vector(end - start, Box::new(ty.clone())),
                            val: vop_rslt,
                            idx: i,
                        };
                        vc.push(inst);

                        let ll::Inst::Trunc { ty1, ty2, .. } = trunc else {unreachable!()};
                        let new_rslt = ll::Value::Temp(addr, 5);
                        let inst = ll::Inst::Trunc {
                            rslt: new_rslt,
                            ty1,
                            val: rslt,
                            ty2,
                        };
                        let rslt = new_rslt;
                        vd.push(inst);

                        let ll::Inst::Sext { ty1, ty2, .. } = sext else {unreachable!()};
                        let new_rslt = ll::Value::Temp(addr, 6);
                        let inst = ll::Inst::Sext {
                            rslt: new_rslt,
                            ty1,
                            val: rslt,
                            ty2,
                        };
                        let rslt = new_rslt;
                        ve.push(inst);

                        let inst = ll::Inst::Store { ty, val: rslt, ptr };
                        vf.push(inst);
                    }
                    func.inst_blocks[end - 1].insts.extend(va);
                    func.inst_blocks[end - 1].insts.extend(vb);
                    func.inst_blocks[end - 1].insts.push(vop);
                    func.inst_blocks[end - 1].insts.extend(vc);
                    func.inst_blocks[end - 1].insts.extend(vd);
                    func.inst_blocks[end - 1].insts.extend(ve);
                    func.inst_blocks[end - 1].insts.extend(vf);
                }
                _ => (),
            }
        }
    }
    prog
}

fn find_jump_targets(func: &ll::Func) -> HashSet<rv::Addr> {
    let mut blocks = HashSet::new();
    for block in &func.inst_blocks {
        match &block.rv_inst {
            rv::Inst::Beq { addr, .. }
            | rv::Inst::Bne { addr, .. }
            | rv::Inst::Blt { addr, .. }
            | rv::Inst::Bge { addr, .. }
            | rv::Inst::Bltu { addr, .. }
            | rv::Inst::Bgeu { addr, .. }
            | rv::Inst::Beqz { addr, .. }
            | rv::Inst::Bnez { addr, .. }
            | rv::Inst::Blez { addr, .. }
            | rv::Inst::Bgez { addr, .. }
            | rv::Inst::Bltz { addr, .. }
            | rv::Inst::Bgtz { addr, .. }
            | rv::Inst::J { addr, .. } => blocks.insert(*addr),
            _ => continue,
        };
    }
    blocks
}

fn get_inst_type_id(inst: &rv::Inst) -> String {
    format!("{inst:?}")
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .to_string()
}
