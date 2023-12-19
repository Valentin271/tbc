use std::collections::HashMap;

use tiny_elf::asm::{Operand, Program};

/// The symbol table
///
/// Note there is no parent table because we don't need it in TinyBASIC, everything is global.
#[derive(Default, Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
    /// Current address on the stack
    ///
    /// This is basically the compiler's `rsp`
    current_address: u32,
}

impl SymbolTable {
    /// Creates a new symbol in the table
    pub fn insert(&mut self, name: &str, ty: Type) {
        // This basically implements shadowing
        // Meaning you can declare a variable with the same name
        if self.get(name).is_some() {
            return;
        }

        self.symbols
            .insert(name.into(), Symbol::new(ty, self.current_address));
        self.current_address += 8;
    }

    /// Gets a symbol by name
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// The size to allocate on the stack for every symbol to fit
    ///
    /// Note that string are note on the stack but in the data section.
    pub fn size(&self) -> u32 {
        self.symbols
            .values()
            .filter(|s| s.ty != Type::String)
            .fold(0, |acc, s| acc + s.ty.size())
    }

    /// Puts the variable with the given name in the [`Rbx`](tiny_elf::asm::Register::Rbx) register
    ///
    /// This is not usually part of a symbol table. However, this is convenient since stack access 
    /// is so complicated because pointer arithmetic is not implemented.
    pub fn access(&self, name: &str, program: Program) -> Program {
        use tiny_elf::asm::{Mnemonic::*, Register::*};
        let addr = self.get(name).unwrap().end_addr() as i32;
        program
            .add(Mov(R14, Rsp.into()))
            .add(Mov(Rsp, R15.into()))
            .add(Sub(Rsp, addr.into()))
            .add(Pop(Rbx))
            .add(Mov(Rsp, R14.into()))
    }

    /// [`Mov`](tiny_elf::asm::Mnemonic::Mov) the given operand into the variable with the given
    /// name
    ///
    /// This is not usually part of a symbol table. However, this is convenient since stack access 
    /// is so complicated because pointer arithmetic is not implemented.
    pub fn write(&self, name: &str, value: Operand, program: Program) -> Program {
        use tiny_elf::asm::{Mnemonic::*, Register::*};
        let addr = self.get(name).unwrap().st_addr() as i32;
        program
            .add(Mov(R14, Rsp.into()))
            .add(Mov(Rsp, R15.into()))
            .add(Sub(Rsp, addr.into()))
            .add(Push(value))
            .add(Mov(Rsp, R14.into()))
    }
}

#[derive(Default, Debug)]
pub struct Symbol {
    ty: Type,
    /// Its address on the stack
    address: u32,
}

impl Symbol {
    fn new(ty: Type, address: u32) -> Self {
        Self { ty, address }
    }

    /// Gets the start address of this symbol
    ///
    /// Useful for writing to it.
    pub fn st_addr(&self) -> u32 {
        self.address
    }

    /// Gets the end address of this symbol
    ///
    /// Useful for reading it.
    pub fn end_addr(&self) -> u32 {
        self.address + self.ty.size()
    }
}

#[derive(Default, Debug, PartialEq)]
pub enum Type {
    #[default]
    Int,
    /// Strings are currently not used in this implementation because they cannot be in a variable
    String,
}

impl Type {
    /// Returns the size in memory of this data type
    pub fn size(&self) -> u32 {
        8
    }
}
