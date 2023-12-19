use tiny_elf::asm::{Mnemonic, Program};

pub trait Optimize {
    fn optimize(self) -> Self;
}

/// Optimizes the assembly instructions of a [`Program`]
impl Optimize for Program {
    fn optimize(mut self) -> Self {
        use tiny_elf::asm::{Immediate::*, Mnemonic::*, Operand::*};
        let mut instructions = Vec::new();

        let mut last: Option<Mnemonic> = None;

        for inst in self.instructions {
            let new_inst = match (&last, inst) {
                // inc/dev is faster then add/sub 1
                (_, Add(r, Imm(Imm8(1) | Imm16(1) | Imm32(1)))) => Inc(r),
                (_, Sub(r, Imm(Imm8(1) | Imm16(1) | Imm32(1)))) => Dec(r),
                // add/sub 0 is useless
                (_, Add(_, Imm(Imm8(0) | Imm16(0) | Imm32(0)))) => continue,
                (_, Sub(_, Imm(Imm8(0) | Imm16(0) | Imm32(0)))) => continue,
                // xor is faster than mov 0
                (_, Mov(r, Imm(Imm8(0) | Imm16(0) | Imm32(0)))) => Xor(r, r.into()),
                // push then pop essentially means mov
                (Some(Push(o)), Pop(r)) => {
                    instructions.pop();
                    Mov(r, o.clone())
                }
                (_, i) => i,
            };

            last = Some(new_inst.clone());
            instructions.push(new_inst)
        }

        self.instructions = instructions;
        self
    }
}
