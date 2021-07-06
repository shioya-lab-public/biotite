use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

pub type LlvmProgram = ();

pub struct LlvmFunction {
    name: String,
    return_type: LlvmType,
    parameters: Vec<(LlvmType, LlvmValue)>,
    body: Vec<LlvmInstruction>,
}

impl Display for LlvmFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut param_str = self
            .parameters
            .iter()
            .map(|(ty, var)| format!("{} {}, ", ty, var))
            .reduce(|params, param| params + &param)
            .map(|mut params| {
                params.truncate(params.len() - 2);
                params
            })
            .unwrap();
        let mut body_str = self
            .body
            .iter()
            .map(|inst| format!("    {}\n", inst))
            .reduce(|insts, inst| insts + &inst)
            .unwrap();
        write!(
            f,
            "define {} @{}({}) {{\n{}}}\n",
            self.return_type, self.name, param_str, body_str
        )
    }
}

pub enum LlvmInstruction {
    Ret(LlvmType, LlvmValue),
}

impl Display for LlvmInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LlvmInstruction::Ret(ty, val) => write!(f, "ret {} {}", ty, val),
        }
    }
}

pub enum LlvmType {
    I64,
}

impl Display for LlvmType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LlvmType::I64 => write!(f, "i64"),
        }
    }
}

pub enum LlvmValue {
    Integer(i64),
    Variable(SSA),
}

impl Display for LlvmValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LlvmValue::Integer(val) => write!(f, "{}", val),
            LlvmValue::Variable(var) => write!(f, "{}", var),
        }
    }
}

pub struct SSA {
    name: String,
    version: usize,
}

impl SSA {
    pub fn new(name: &str, version: &usize) -> Self {
        SSA {
            name: name.to_string(),
            version: *version,
        }
    }
}

impl Display for SSA {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "%{}_{}", self.name, self.version)
    }
}

pub struct SSABuilder {
    variables: HashMap<String, usize>,
}

impl SSABuilder {
    pub fn new() -> Self {
        SSABuilder {
            variables: HashMap::new(),
        }
    }

    pub fn build(&mut self, name: &str) -> SSA {
        let ver = self.variables.entry(name.to_string()).or_default();
        let var = SSA::new(name, ver);
        *ver += 1;
        var
    }

    pub fn clear(&mut self) {
        self.variables.clear();
    }
}
