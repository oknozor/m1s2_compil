use std::collections::HashMap;
use std::fmt::Display;

use crate::ast::expression::*;
use crate::ast::expression::Expression::*;
use crate::ast::statement::*;
use crate::ast::statement::Statement::*;
use crate::ast::statement::Statement;
use crate::file_writer::CReserved::*;
use crate::file_writer::INCLUDES;
use crate::to_c::to_c;

static STD_LIB: &[&'static str] = &["add", "mull", "div", "eq", "print"];

pub struct PrettyPrinter<'printer> {
    pub out: &'printer mut String,
}

#[allow(unused)]
impl<'pr> PrettyPrinter<'pr> {
    fn with_loc() -> bool {
        true
    }

    pub fn visit_program_root(&mut self, e: Vec<Box<Statement>>) {
        self.out.push_str(INCLUDES);
        let vars_gen = self.visit_global_vars(e);
        let func_gen = self.visit_function_declarations(vars_gen);
        let main_gen = self.visit_calls(func_gen);
        self.out.push_str("\n}");
    }

    fn visit_calls(&mut self, e: Vec<Box<Statement>>) -> Vec<Box<Statement>> {
        self.out.push_str("\nvoid main() {\n");
        let mut new_root = e.clone();
        e.iter().enumerate().for_each(|(i, statement)| {
            self.visit_statement(statement);
        });
        new_root
    }

    fn visit_global_vars(&mut self, root: Vec<Box<Statement>>) -> Vec<Box<Statement>> {
        let mut new_root = root.clone();
        root.iter().enumerate().for_each(|(i, statement)| {
            if let box VariableDeclaration(var) = statement {
                self.visit_variable_declaration(var);
                new_root.remove(i);
            }
        });
        new_root
    }

    fn visit_function_declarations(&mut self, root: Vec<Box<Statement>>) -> Vec<Box<Statement>> {
        let mut new_root = root.clone();
        root.iter().enumerate().for_each(|(i, statement)| {
            if let box FunctionDeclaration(function) = statement {
                new_root.remove(i);
                self.append(Databox.as_str());
                self.visit_function_declaration(function);
            };
        });
        new_root
    }

    // statement
    fn visit_statement(&mut self, s: &Statement) {
        match s {
            BlockStatement(b) => self.visit_block_statement(b),
            ExpressionStatement(e) => self.visit_expression_statement(e),
            WhileStatement(v) => self.visit_while_statment(v),
            IfStatement(i) => self.visit_if_statement(i),
            SwitchStatement(s) => self.visit_switch_statement(s),
            ForStatement(f) => self.visit_for_statement(f),
            BreakStatement(b) => self.visit_break_statement(b),
            ContinueStatement(c) => self.visit_continue_statement(c),
            ReturnStatement(r) => self.visit_return_statement(r),
            _ => (),
        }
        self.new_line();
    }


    fn visit_while_statment(&mut self, w: &WhileStmt) {
        self.append(While.as_str());
        self.parentesis_left();
        self.visit_expression(&w.test);
        self.parentesis_right();
        self.bracket_left();
        self.visit_statement(&w.body);
        self.bracket_right();
    }

    fn visit_block_statement(&mut self, s: &BlockStmt) {
        s.body
         .iter()
         .for_each(|statement| self.visit_statement(statement))
    }
    fn visit_variable_declarator(&mut self, v: &Variable) {
        self.append(Databox.as_str());
        self.out.push_str(&v.id.name);
        self.visit_identifier(&v.id);
        if let Some(box init) = &v.init {
            self.append_equal();
            self.append_ref(init);
        };
        self.semi_col();
    }

    fn append_ref(&mut self, init: &Expression) {
        match init {
            StringLiteral(ref s) => {
                let out = s.value.clone();
                let out = format!("{{.data.str = \"{}\", .type=STR }}", out);
                self.out.push_str(&out);
            }
            NumericLiteral(n) => {
                let out = format!("{{.data.num = {}, .type=NUM }}", n.value);
                self.out.push_str(out.as_str());
            }
            _ => ()
        }
    }

    fn visit_while_statement(&mut self, w: &WhileStmt) {
        self.append(While.as_str());
        self.parentesis_left();
        self.visit_expression(&w.test);
        self.parentesis_right();
        self.bracket_left();
        self.bracket_right();
        self.visit_statement(&w.body);
    }

    fn visit_variable_declaration(&mut self, v: &VariableDec) {
        v.declarations.iter().for_each(|declaration| {
            if let box VariableDeclarator(declarator) = declaration {
                self.visit_variable_declarator(&declarator);
                self.new_line();
            }
        });
    }

    fn visit_if_statement(&mut self, i: &IfStmt) {
        self.out.push_str("if(");
        self.visit_expression(&i.test);
        self.visit_statement(&i.consequent);
    }

    fn visit_switch_statement(&mut self, s: &SwitchStmt) {}
    fn visit_option_expression(&mut self, exp: &Option<Box<Expression>>) {
        if let Some(expression) = exp {
            self.visit_expression(&expression);
        }
    }

    fn visit_for_statement(&mut self, f: &ForStmt) {
        self.visit_option_expression(&f.init);
        self.visit_option_expression(&f.test);
        self.visit_option_expression(&f.update);
        self.visit_statement(&f.body);
    }

    fn visit_break_statement(&mut self, f: &BreakStmt) {
        self.append(Break.as_str());
    }

    fn visit_continue_statement(&mut self, c: &ContinueStmt) {
        self.append(Continue.as_str());
    }

    fn visit_return_statement(&mut self, r: &ReturnStmt) {
        self.visit_option_expression(&r.argument);
    }

    fn visit_function_declaration(&mut self, f: &FunctionDec) {
        let identifier = self.visit_identifier(&f.id);

        self.out.push_str(&identifier);
        self.parentesis_left();
        f.params.iter().for_each(|param| {
            self.append(Databox.as_str());
            self.out.push_str(param.name.as_str());
        });
        self.parentesis_right();
        self.bracket_left();
        self.new_line();
        self.visit_block_statement(&f.body);
        self.bracket_right();
    }
    fn visit_identifier(&mut self, id: &Id) -> String {
        id.name.clone()
    }

    fn visit_expression(&mut self, exp: &Expression) {
        match exp {
            NumericLiteral(ref n) => {
                let out = format!("{}", n.value);
                self.out.push_str(&out);
            }
            StringLiteral(ref s) => self.out.push_str(&format!("\"{}\"", &s.value)),
            UpdateExpression(ref u) => self.visit_update_expression(u),
            BinaryExpression(ref b) => self.visit_binary_expression(b),
            UnaryExpression(ref u) => self.visit_unary_expression(u),
            MemberExpression(ref m) => self.visit_member_expression(m),
            CallExpression(ref c) => self.visit_call_expression(c),
            AssignmentExpression(ref e) => self.visit_assign(e),
            LogicalExpression(ref l) => self.visit_logical_expression(l),
            _ => (),
        };
    }

    fn visit_expression_statement(&mut self, s: &ExpressionStmt) {
        self.visit_expression(&s.expression);
        self.semi_col();
    }

    fn visit_binary_expression(&mut self, b: &BinaryExp) {
        let mut identifier_left =
            if let box Identifier(left) = &b.left {
                Some(left)
            } else {
                None
            };
        let mut identifier_right =
            if let box Identifier(right) = &b.right {
                Some(right)
            } else {
                None
            };
        let mut as_parenthesis =
            if let Some(extra) =
            &b.extra {
                true
            } else {
                false
            };
        let as_identifier = if identifier_left.is_some() || identifier_right.is_some() {
            true
        } else {
            false
        };

        let operator = &b.operator;
        let mut temp_expression = String::new();
        if as_parenthesis { self.parentesis_left(); }

        if as_identifier {
            self.out.push_str(to_c(operator));
            self.parentesis_left();

            if let Some(left) = identifier_left {
                self.out.push_str(&left.name);
            } else {
                self.visit_expression(&b.left);
            };

            self.coma();

            if let Some(right) = identifier_right {
                self.out.push_str(&right.name);
            } else {
                self.visit_expression(&b.right);
            };

            self.parentesis_right();
        } else {
            self.visit_expression(&b.left);
            self.out.push_str(operator);
            self.visit_expression(&b.right);
        }
        if as_parenthesis { self.parentesis_right(); }
    }


    fn visit_assign(&mut self, a: &AssignmentExp) {
        if let box Identifier(id) = &a.left {
            self.out.push_str(&id.name);
        }
        self.out.push_str(a.operator.as_str());
        self.visit_expression(&a.right);
    }

    fn visit_unary_expression(&mut self, u: &UnaryExp) {
        self.out.push_str(u.operator.as_str());
        self.visit_expression(&u.argument);
    }

    fn visit_update_expression(&mut self, u: &UpdateExp) {
        let expression = &u.argument;
        self.visit_expression(&u.argument);
        let mut arg = "";
        match expression {
            box Identifier(exp) => arg = &exp.name,
            _ => unimplemented!()
        };
    }

    fn visit_member_expression(&mut self, m: &MemberExp) {
        self.visit_expression(&m.object);
        self.visit_expression(&m.property);
    }

    fn visit_logical_expression(&mut self, l: &LogicalExp) {
        self.visit_expression(&l.left);
        self.out.push_str(l.operator.as_str());
        self.visit_expression(&l.right);
    }

    fn visit_call_expression(&mut self, e: &CallExp) {
        let mut standard_lib_call = false;
        if let box Identifier(id) = &e.callee {
            if STD_LIB.contains(&id.name.as_str()) {
                standard_lib_call = true;
            }
            self.out.push_str(&id.name);
        }

        self.parentesis_left();
        let last = e.arguments.len() - 1;

        e.arguments.iter().enumerate().for_each(|(i, expression)| {
            if !standard_lib_call {
                self.out.push_str("new");
                self.parentesis_left();
            }
            if let box Identifier(id) = expression {
                self.out.push_str(&id.name);
            }
            self.visit_expression(expression);
            if i != last {
                self.out.push_str(",");
            };
            if !standard_lib_call {
                self.parentesis_right();
            }
        });
        self.parentesis_right();
    }

    #[allow(unused)]
    fn append(&mut self, word: &str) {
        self.out.push_str(word)
    }
    pub fn print(&self) {
        println!("{}", self.out)
    }

    fn new_line(&mut self) {
        self.out.push_str("\n");
    }
    fn semi_col(&mut self) {
        self.out.push_str(";");
    }
    fn append_col(&mut self) {
        self.out.push_str(":");
    }
    fn backspace(&mut self) {
        self.out.push_str(" ");
    }
    fn parentesis_left(&mut self) {
        self.out.push_str("(");
    }
    fn parentesis_right(&mut self) {
        self.out.push_str(")");
    }
    fn bracket_left(&mut self) {
        self.out.push_str("{");
    }
    fn bracket_right(&mut self) {
        self.out.push_str("}");
    }
    fn coma(&mut self) {
        self.out.push_str(",");
    }
    fn append_equal(&mut self) {
        self.out.push_str("=");
    }
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
