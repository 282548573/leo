use crate::SymbolTable;
use indexmap::IndexMap;
use leo_span::Symbol;
use snarkvm_bytecode::{instructions::Instruction, register::Register, Program, Value};

pub struct Reg<P: Program> {
    pub register: Register<P>,
    pub symbol: Option<Symbol>,
}

impl<P: Program> PartialEq<Symbol> for Reg<P> {
    fn eq(&self, other: &Symbol) -> bool {
        matches!(self.symbol, Some(s) if s == *other)
    }
}

pub struct IrEmitter<'a, P: Program> {
    pub(crate) symbol_table: &'a mut SymbolTable<'a>,
    pub(crate) parent: Option<Symbol>,
    pub(crate) registers: Vec<Reg<P>>,
    pub(crate) constants: IndexMap<Symbol, Value<P>>,
    pub(crate) buffer: Vec<Instruction<P>>,
}

impl<'a, P: Program> IrEmitter<'a, P> {
    pub fn new(symbol_table: &'a mut SymbolTable<'a>) -> Self {
        Self {
            symbol_table,
            parent: None,
            registers: Vec::new(),
            constants: IndexMap::new(),
            buffer: Vec::new(),
        }
    }

    pub fn emit(&mut self, v: Instruction<P>) {
        self.buffer.push(v)
    }

    pub(crate) fn spawn_reg(&mut self, symbol: Option<Symbol>) -> Register<P> {
        let register = Register::Locator(self.registers.len() as u64);
        self.registers.push(Reg {
            register: register.clone(),
            symbol,
        });
        register
    }

    pub(crate) fn drop_reg(&mut self) -> Reg<P> {
        self.registers.pop().unwrap()
    }
}
