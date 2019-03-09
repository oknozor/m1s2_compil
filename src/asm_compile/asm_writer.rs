use crate::ast::statement::Statement;
use crate::asm_compile::ASM_INIT;
use crate::visitor::Visitor;
use crate::asm_compile::RAX;
use crate::asm_compile::RBX;
use crate::asm_compile::Register;
use crate::asm_compile::*;
use crate::writer::TAB;
use crate::writer::NEW_LINE;

pub struct ASMWriter<'printer> {
    pub out: &'printer mut String,
    pub reg: Register,
}

impl<'printer> ASMWriter<'printer> {
    pub fn visit_program_root(&mut self, root_nodes: Vec<Box<Statement>>) {
        self.change_register();
        self.append(ASM_INIT);
        /*
                let vars_gen = self.visit_global_vars(root_nodes);
        */
        /*
                let func_gen = self.visit_function_declarations(vars_gen);
        */
        let main_gen = self.visit_calls(root_nodes);
    }

    fn visit_calls(&mut self, nodes: Vec<Box<Statement>>) -> Vec<Box<Statement>> {
        let mut new_root = nodes.clone();
        nodes.iter().enumerate().for_each(|(i, statement)| {
            self.visit_statement(statement);
        });
        new_root
    }

    pub fn append(&mut self, word: &str) {
        self.out.push_str(word)
    }

    pub fn change_register(&mut self) {
        if let Register::RAX = self.reg {
            self.reg = Register::RBX;
        } else {
            self.reg = Register::RAX;
        }
    }

    pub fn write_asm_op(&mut self, str_op: &str) {
        self.append(TAB);
        match str_op {
            "+" => {
                self.append(ASM_ADD);
                self.append(&format!("{}%{}, %{}", TAB, RAX, RBX));
            }
            "-" => {
                self.append(ASM_SUB);
                self.append(&format!("{}%{}, %{}", TAB, RBX, RAX));
            }
            "/" => {
                self.append(ASM_DIV);
                self.append("cqto");
                self.append(&format!("{} %{}", TAB, RBX));
                self.append(NEW_LINE)
            }
            "*" => {
                self.append(ASM_MUL);
                self.append(&format!("{} %{}", TAB, RBX));
                self.append(NEW_LINE)
            }
            _ => println!("{} not found", str_op)
        }
    }
}
