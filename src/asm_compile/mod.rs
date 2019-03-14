use crate::asm_compile::asm_writer::ASMWriter;

pub mod asm_writer;
pub mod asm_visitor;

pub const ASM_INIT: &'static str = ".text\n.global main\n.type main, @function\nmain:\n";

pub const RAX: &'static str ="rax";
pub const RBX: &'static str ="rbx";
pub const ASM_ADD: &'static str = "addq";
pub const ASM_SUB: &'static str = "subq";
pub const ASM_MUL: &'static str = "imul";
pub const ASM_DIV: &'static str = "idiv";
pub const ASM_MOVE: &'static str = "movq";
pub const ASM_PUSH: &'static str = "pushq";
pub const ASM_POP: &'static str = "popq";
pub const ASM_RET: &'static str = "ret";


pub enum Register {
    RAX,
    RBX
}

impl ToString for Register {
    fn to_string(&self) -> String {
        match self {
            Register::RAX => RAX.to_string(),
            Register::RBX => RBX.to_string()
        }
    }
}
