use crate::riscv_isa::*;
use std::collections::HashMap;

pub struct Interpreter {
    bytes: Vec<u8>,
    insts: HashMap<Address, Instruction>,
    regs: HashMap<Register, i64>,
    fregs: HashMap<FPRegister, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            bytes: Vec::new(),
            insts: HashMap::new(),
            regs: HashMap::new(),
            fregs: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: Program) {
        self.bytes = program.data_blocks[0].bytes.clone();
        self.regs.insert(Register::Zero, 0);
        self.regs.insert(Register::Ra, 0);
        self.regs.insert(Register::Sp, 10240);
        self.regs.insert(Register::Gp, 0);
        self.regs.insert(Register::Tp, 0);
        self.regs.insert(Register::T0, 0);
        self.regs.insert(Register::T1, 0);
        self.regs.insert(Register::T2, 0);
        self.regs.insert(Register::S0, 0);
        self.regs.insert(Register::S1, 0);
        self.regs.insert(Register::A0, 1);
        self.regs.insert(Register::A1, 0);
        self.regs.insert(Register::A2, 0);
        self.regs.insert(Register::A3, 0);
        self.regs.insert(Register::A4, 0);
        self.regs.insert(Register::A5, 0);
        self.regs.insert(Register::A6, 0);
        self.regs.insert(Register::A7, 0);
        self.regs.insert(Register::S2, 0);
        self.regs.insert(Register::S3, 0);
        self.regs.insert(Register::S4, 0);
        self.regs.insert(Register::S5, 0);
        self.regs.insert(Register::S6, 0);
        self.regs.insert(Register::S7, 0);
        self.regs.insert(Register::S8, 0);
        self.regs.insert(Register::S9, 0);
        self.regs.insert(Register::S10, 0);
        self.regs.insert(Register::S11, 0);
        self.regs.insert(Register::T3, 0);
        self.regs.insert(Register::T4, 0);
        self.regs.insert(Register::T5, 0);
        self.regs.insert(Register::T6, 0);
        self.fregs.insert(FPRegister::Ft0, 0.0);
        self.fregs.insert(FPRegister::Ft1, 0.0);
        self.fregs.insert(FPRegister::Ft2, 0.0);
        self.fregs.insert(FPRegister::Ft3, 0.0);
        self.fregs.insert(FPRegister::Ft4, 0.0);
        self.fregs.insert(FPRegister::Ft5, 0.0);
        self.fregs.insert(FPRegister::Ft6, 0.0);
        self.fregs.insert(FPRegister::Ft7, 0.0);
        self.fregs.insert(FPRegister::Fs0, 0.0);
        self.fregs.insert(FPRegister::Fs1, 0.0);
        self.fregs.insert(FPRegister::Fa0, 0.0);
        self.fregs.insert(FPRegister::Fa1, 0.0);
        self.fregs.insert(FPRegister::Fa2, 0.0);
        self.fregs.insert(FPRegister::Fa3, 0.0);
        self.fregs.insert(FPRegister::Fa4, 0.0);
        self.fregs.insert(FPRegister::Fa5, 0.0);
        self.fregs.insert(FPRegister::Fa6, 0.0);
        self.fregs.insert(FPRegister::Fa7, 0.0);
        self.fregs.insert(FPRegister::Fs2, 0.0);
        self.fregs.insert(FPRegister::Fs3, 0.0);
        self.fregs.insert(FPRegister::Fs4, 0.0);
        self.fregs.insert(FPRegister::Fs5, 0.0);
        self.fregs.insert(FPRegister::Fs6, 0.0);
        self.fregs.insert(FPRegister::Fs7, 0.0);
        self.fregs.insert(FPRegister::Fs8, 0.0);
        self.fregs.insert(FPRegister::Fs9, 0.0);
        self.fregs.insert(FPRegister::Fs10, 0.0);
        self.fregs.insert(FPRegister::Fs11, 0.0);
        self.fregs.insert(FPRegister::Ft8, 0.0);
        self.fregs.insert(FPRegister::Ft9, 0.0);
        self.fregs.insert(FPRegister::Ft10, 0.0);
        self.fregs.insert(FPRegister::Ft11, 0.0);

        let mut addr = Address(0);
        for block in program.code_blocks {
            for inst in block.instructions {
                self.insts.insert(inst.address(), inst);
            }
            if let "_start" = block.symbol.as_str() {
                addr = block.address;
            }
        }
        loop {
            addr = self.interpret(addr);
        }
    }

    fn interpret(&mut self, Address(addr): Address) -> Address {
        use Instruction::*;
        // println!("#{addr}#");
        let inst = &self.insts[&Address(addr)];
        let mut next = (inst.raw().len() / 2) as u64 + addr;

        if addr == 0x1022c {
            println!("?{:?}?", self.regs[&Register::A0]);
            panic!()
        }

        match inst {
            // RV32I
            Lui {
                rd,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = (imm << 44) >> 32,
            Auipc {
                rd,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = (imm << 12) + addr as i64,
            Jal {
                rd,
                addr: Address(addr),
                ..
            } => {
                *self.regs.get_mut(rd).unwrap() = next as i64;
                next = *addr;
            }
            Jalr {
                rd,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                *self.regs.get_mut(rd).unwrap() = next as i64;
                next = (self.regs[rs1] + imm) as u64;
            }
            ImplicitJalr {
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                *self.regs.get_mut(&Register::Ra).unwrap() = next as i64;
                next = (self.regs[rs1] + imm) as u64;
            }
            Beq {
                rs1,
                rs2,
                addr: Address(addr),
                ..
            } => {
                if self.regs[rs1] == self.regs[rs2] {
                    next = *addr
                }
            }
            Bne {
                rs1,
                rs2,
                addr: Address(addr),
                ..
            } => {
                if self.regs[rs1] == self.regs[rs2] {
                    next = *addr
                }
            }
            Blt {
                rs1,
                rs2,
                addr: Address(addr),
                ..
            } => {
                if self.regs[rs1] < self.regs[rs2] {
                    next = *addr
                }
            }
            Bge {
                rs1,
                rs2,
                addr: Address(addr),
                ..
            } => {
                if self.regs[rs1] >= self.regs[rs2] {
                    next = *addr
                }
            }
            Bltu {
                rs1,
                rs2,
                addr: Address(addr),
                ..
            } => {
                if (self.regs[rs1] as u64) < (self.regs[rs2] as u64) {
                    next = *addr
                }
            }
            Bgeu {
                rs1,
                rs2,
                addr: Address(addr),
                ..
            } => {
                if (self.regs[rs1] as u64) >= (self.regs[rs2] as u64) {
                    next = *addr
                }
            }
            Lb {
                rd,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let addr = (self.regs[rs1] + imm) as usize;
                *self.regs.get_mut(rd).unwrap() = self.bytes[addr] as i64;
            }
            Lh {
                rd,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let addr = (self.regs[rs1] + imm) as usize;
                *self.regs.get_mut(rd).unwrap() =
                    i16::from_le_bytes(self.bytes[addr..addr + 2].try_into().unwrap()) as i64;
            }
            Lw {
                rd,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let addr = (self.regs[rs1] + imm) as usize;
                *self.regs.get_mut(rd).unwrap() =
                    i32::from_le_bytes(self.bytes[addr..addr + 4].try_into().unwrap()) as i64;
            }
            Lbu {
                rd,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let addr = (self.regs[rs1] + imm) as usize;
                *self.regs.get_mut(rd).unwrap() = self.bytes[addr] as u64 as i64;
            }
            Lhu {
                rd,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let addr = (self.regs[rs1] + imm) as usize;
                *self.regs.get_mut(rd).unwrap() =
                    u16::from_le_bytes(self.bytes[addr..addr + 2].try_into().unwrap()) as i64;
            }
            Sb {
                rs2,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let byte = self.regs[rs2].to_le_bytes()[0];
                let addr = (self.regs[rs1] + imm) as usize;
                self.bytes[addr] = byte;
            }
            Sh {
                rs2,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let mut bytes = &mut self.regs[rs2].to_le_bytes()[0..2];
                let addr = (self.regs[rs1] + imm) as usize;
                self.bytes[addr..addr + 2].swap_with_slice(&mut bytes);
            }
            Sw {
                rs2,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let mut bytes = &mut self.regs[rs2].to_le_bytes()[0..4];
                let addr = (self.regs[rs1] + imm) as usize;
                self.bytes[addr..addr + 4].swap_with_slice(&mut bytes);
            }
            Addi {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] + imm,
            Slti {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => {
                if self.regs[rs1] < *imm {
                    *self.regs.get_mut(rd).unwrap() = 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }
            Sltiu {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => {
                if (self.regs[rs1] as u64) < (*imm as u64) {
                    *self.regs.get_mut(rd).unwrap() = 1
                } else {
                    *self.regs.get_mut(rd).unwrap()= 0
                }
            }
            Xori {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] ^ imm,
            Ori {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] | imm,
            Andi {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] & imm,
            Slli {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] << imm,
            Srli {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = (u64::from_le_bytes(self.regs[rs1].to_le_bytes()) >> imm) as i64,
            Srai {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] >> imm,
            Add { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] + self.regs[rs2],
            Sub { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] - self.regs[rs2],
            Sll { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] << self.regs[rs2],
            Slt { rd, rs1, rs2, .. } => {
                if self.regs[rs1] < self.regs[rs2] {
                    *self.regs.get_mut(rd).unwrap() = 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }
            Sltu { rd, rs1, rs2, .. } => {
                if (self.regs[rs1] as u64) < (self.regs[rs2] as u64) {
                    *self.regs.get_mut(rd).unwrap()= 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }
            Xor { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] ^ self.regs[rs2],
            Srl { rd, rs1, rs2, .. } => {
                *self.regs.get_mut(rd).unwrap() =
                    (u64::from_le_bytes(self.regs[rs1].to_le_bytes()) >> self.regs[rs2]) as i64
            }
            Sra { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] >> self.regs[rs2],
            Or { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] | self.regs[rs2],
            And { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] & self.regs[rs2],
            Fence { .. } => {}
            Ecall {  .. } => unsafe {
                *self.regs.get_mut(&Register::A0).unwrap() = libc::syscall(
                    self.regs[&Register::A7],
                    self.regs[&Register::A0],
                    self.regs[&Register::A1],
                    self.regs[&Register::A2],
                    self.regs[&Register::A3],
                    self.regs[&Register::A4],
                    self.regs[&Register::A5],
                );
            },
            Ebreak { .. } => panic!("`ebreak` is not implemented"),

            // RV64I
            Lwu {
                rd,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let addr = (self.regs[rs1] + imm) as usize;
                *self.regs.get_mut(rd).unwrap() =
                    u32::from_le_bytes(self.bytes[addr..addr + 4].try_into().unwrap()) as i64;
            }
            Ld {
                rd,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let addr = (self.regs[rs1] + imm) as usize;
                *self.regs.get_mut(rd).unwrap() = i64::from_le_bytes(self.bytes[addr..addr + 8].try_into().unwrap());
            }
            Sd {
                rs2,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let mut bytes = &mut self.regs[rs2].to_le_bytes()[..];
                let addr = (self.regs[rs1] + imm) as usize;
                self.bytes[addr..addr + 8].swap_with_slice(&mut bytes);
            }
            Addiw {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] + imm,
            Slliw {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] << imm,
            Srliw {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap()= (u64::from_le_bytes(self.regs[rs1].to_le_bytes()) >> imm) as i64,
            Sraiw {
                rd,
                rs1,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] >> imm,
            Addw { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] + self.regs[rs2],
            Subw { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] - self.regs[rs2],
            Sllw { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] << self.regs[rs2],
            Srlw { rd, rs1, rs2, .. } => {
                *self.regs.get_mut(rd).unwrap() =
                    (u64::from_le_bytes(self.regs[rs1].to_le_bytes()) >> self.regs[rs2]) as i64
            }
            Sraw { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] >> self.regs[rs2],

            // Pseudoinstructions
            Nop { .. } => {}
            Li {
                rd,
                imm: Immediate(imm),
                ..
            } => *self.regs.get_mut(rd).unwrap() = *imm,
            Mv { rd, rs1, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1],
            Not { rd, rs1, .. } => *self.regs.get_mut(rd).unwrap() = !self.regs[rs1],
            Neg { rd, rs1, .. } => *self.regs.get_mut(rd).unwrap() = -self.regs[rs1],
            Negw { rd, rs1, .. } => *self.regs.get_mut(rd).unwrap() = !self.regs[rs1],
            SextW { rd, rs1, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1],
            Seqz { rd, rs1, .. } => {
                if self.regs[rs1] == 0 {
                    *self.regs.get_mut(rd).unwrap() = 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }
            Snez { rd, rs1, .. } => {
                if self.regs[rs1] != 0 {
                    *self.regs.get_mut(rd).unwrap()= 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }
            Sltz { rd, rs1, .. } => {
                if self.regs[rs1] < 0 {
                    *self.regs.get_mut(rd).unwrap() = 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }
            Sgtz { rd, rs1, .. } => {
                if self.regs[rs1] > 0 {
                    *self.regs.get_mut(rd).unwrap() = 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }

            Beqz {
                rs1,
                addr: Address(addr),
                ..
            } => {
                if self.regs[rs1] == 0 {
                    next = *addr
                }
            }
            Bnez {
                rs1,
                addr: Address(addr),
                ..
            } => {
                if self.regs[rs1] != 0 {
                    next = *addr
                }
            }
            Blez {
                rs1,
                addr: Address(addr),
                ..
            } => {
                if self.regs[rs1] <= 0 {
                    next = *addr
                }
            }
            Bgez {
                rs1,
                addr: Address(addr),
                ..
            } => {
                if self.regs[rs1] >= 0 {
                    next = *addr
                }
            }
            Bltz {
                rs1,
                addr: Address(addr),
                ..
            } => {
                if self.regs[rs1] < 0 {
                    next = *addr
                }
            }
            Bgtz {
                rs1,
                addr: Address(addr),
                ..
            } => {
                if self.regs[rs1] > 0 {
                    next = *addr
                }
            }

            J {
                addr: Address(addr),
                ..
            } => next = *addr,
            Jr { rs1, .. } => next = self.regs[rs1] as u64,
            OffsetJr {
                imm: Immediate(imm),
                rs1,
                ..
            } => next = (self.regs[rs1] + imm) as u64,
            PseudoJalr { rs1, .. } => {
                *self.regs.get_mut(&Register::Ra).unwrap() = next as i64;
                next = self.regs[rs1] as u64;
            }
            Ret { .. } => next = self.regs[&Register::Ra] as u64,

            PseudoFence { .. } => {}

            // Misc
            Unimp { .. } => {} // panic!("Encounter `unimp` at `{}`", address),
            Unknown { .. } => {} // panic!("Encounter `unknown` at `{}`", address),

            // Ad Hoc
            Mul { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] * self.regs[rs2],
            Mulw { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] * self.regs[rs2],
            Divw { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] / self.regs[rs2],
            Divu { rd, rs1, rs2, .. } => {
                *self.regs.get_mut(rd).unwrap() = ((self.regs[rs1] as u64) / (self.regs[rs2] as u64)) as i64
            }
            Divuw { rd, rs1, rs2, .. } => {
                *self.regs.get_mut(rd).unwrap() = ((self.regs[rs1] as u64) / (self.regs[rs2] as u64)) as i64
            }
            Remu { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] % self.regs[rs2],
            Remw { rd, rs1, rs2, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] % self.regs[rs2],
            Mulhu { rd, rs1, rs2, .. } => {
                *self.regs.get_mut(rd).unwrap() =
                    (((self.regs[rs1] as u128) * (self.regs[rs2] as u128)) >> 64) as u64 as i64
            }

            Fld {
                frd,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let addr = (self.regs[rs1] + imm) as usize;
                *self.fregs.get_mut(frd).unwrap() = 
                    f64::from_le_bytes(self.bytes[addr..addr + 8].try_into().unwrap());
            }
            Fsd {
                frs2,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let mut bytes = &mut self.fregs[frs2].to_le_bytes()[..];
                let addr = (self.regs[rs1] + imm) as usize;
                self.bytes[addr..addr + 8].swap_with_slice(&mut bytes);
            }
            Flw {
                frd,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let addr = (self.regs[rs1] + imm) as usize;
                *self.fregs.get_mut(frd).unwrap() = 
                    f32::from_le_bytes(self.bytes[addr..addr + 4].try_into().unwrap()) as f64;
            }
            Fsw {
                frs2,
                imm: Immediate(imm),
                rs1,
                ..
            } => {
                let mut bytes = (self.fregs[frs2] as f32).to_le_bytes();
                let addr = (self.regs[rs1] + imm) as usize;
                self.bytes[addr..addr + 4].swap_with_slice(&mut bytes);
            }
            FmvXD { rd, frs1, .. } => {
                *self.regs.get_mut(rd).unwrap() = i64::from_le_bytes(self.fregs[frs1].to_le_bytes())
            }
            FmvDX { frd, rs1, .. } => {
                *self.fregs.get_mut(frd).unwrap() =  f64::from_le_bytes(self.regs[rs1].to_le_bytes())
            }
            FmvXW { rd, frs1, .. } => {
                *self.regs.get_mut(rd).unwrap() = i64::from_le_bytes(self.fregs[frs1].to_le_bytes())
            }
            FmvWX { frd, rs1, .. } => {
                *self.fregs.get_mut(frd).unwrap() =  f64::from_le_bytes(self.regs[rs1].to_le_bytes())
            }
            FmulD {
                frd, frs1, frs2, ..
            } => *self.fregs.get_mut(frd).unwrap() =  self.fregs[frs1] * self.fregs[frs2],
            FsubD {
                frd, frs1, frs2, ..
            } => *self.fregs.get_mut(frd).unwrap() =  self.fregs[frs1] - self.fregs[frs2],
            FaddD {
                frd, frs1, frs2, ..
            } => *self.fregs.get_mut(frd).unwrap() =  self.fregs[frs1] + self.fregs[frs2],
            FnegD {
                frd,
                frs1,
                ..
            } => *self.fregs.get_mut(frd).unwrap() =  -self.fregs[frs1],
            FmaddD {
                frd,
                frs1,
                frs2,
                frs3,
                ..
            } => *self.fregs.get_mut(frd).unwrap() =  self.fregs[frs1] * self.fregs[frs2] + self.fregs[frs3],
            FmsubD {
                frd,
                frs1,
                frs2,
                frs3,
                ..
            } => *self.fregs.get_mut(frd).unwrap() =  self.fregs[frs1] * self.fregs[frs2] - self.fregs[frs3],
            FnmsubD {
                frd,
                frs1,
                frs2,
                frs3,
                ..
            } => *self.fregs.get_mut(frd).unwrap() =  -self.fregs[frs1] * self.fregs[frs2] + self.fregs[frs3],
            FmulS {
                frd, frs1, frs2, ..
            } => *self.fregs.get_mut(frd).unwrap() =  self.fregs[frs1] * self.fregs[frs2],
            FdivD {
                frd, frs1, frs2, ..
            } => *self.fregs.get_mut(frd).unwrap() =  self.fregs[frs1] / self.fregs[frs2],
            FdivS {
                frd, frs1, frs2, ..
            } => *self.fregs.get_mut(frd).unwrap() =  self.fregs[frs1] / self.fregs[frs2],

            FcvtSL { frd, rs1, .. } => *self.fregs.get_mut(frd).unwrap() =  self.regs[rs1] as f64,
            FcvtDWu { frd, rs1, .. } => *self.fregs.get_mut(frd).unwrap() =  self.regs[rs1] as f64,
            FcvtDW { frd, rs1, .. } => *self.fregs.get_mut(frd).unwrap() =  self.regs[rs1] as f64,
            FcvtWD { rd, frs1, .. } => *self.regs.get_mut(rd).unwrap() = self.fregs[frs1] as i64,
            FcvtSW { frd, rs1, .. } => *self.fregs.get_mut(frd).unwrap() =  self.regs[rs1] as f64,
            FcvtDS { frd, frs1, .. } => *self.fregs.get_mut(frd).unwrap() =  self.fregs[frs1],
            FcvtSD { frd, frs1, .. } => *self.fregs.get_mut(frd).unwrap() =  self.fregs[frs1],

            FmvD { frd, frs1, .. } => *self.fregs.get_mut(frd).unwrap() =  self.fregs[frs1],
            FeqD { rd, frs1, frs2, .. } => {
                if self.fregs[frs1] == self.fregs[frs2] {
                    *self.regs.get_mut(rd).unwrap() = 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }
            FltD { rd, frs1, frs2, .. } => {
                if self.fregs[frs1] < self.fregs[frs2] {
                    *self.regs.get_mut(rd).unwrap() = 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }
            FleD { rd, frs1, frs2, .. } => {
                if self.fregs[frs1] <= self.fregs[frs2] {
                    *self.regs.get_mut(rd).unwrap() = 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }
            FeqS { rd, frs1, frs2, .. } => {
                if self.fregs[frs1] == self.fregs[frs2] {
                    *self.regs.get_mut(rd).unwrap() = 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }
            FltS { rd, frs1, frs2, .. } => {
                if self.fregs[frs1] < self.fregs[frs2] {
                    *self.regs.get_mut(rd).unwrap() = 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }
            FleS { rd, frs1, frs2, .. } => {
                if self.fregs[frs1] <= self.fregs[frs2] {
                    *self.regs.get_mut(rd).unwrap() = 1
                } else {
                    *self.regs.get_mut(rd).unwrap() = 0
                }
            }

            Frrm { .. } => {}
            Csrs { .. } => {}
            Csrrw { .. } => {}
            Csrr { .. } => {}
            Csrsi { .. } => {}
            Csrrsi { .. } => {}
            Csrrci { .. } => {}
            Csrrs { .. } => {}

            ZextB { rd, rs1, .. } => *self.regs.get_mut(rd).unwrap() = self.regs[rs1] & 255,

            inst => todo!("{:?}", inst),
        }

        *self.regs.get_mut(&Register::Zero).unwrap() = 0;
        Address(next)
    }
}
