use crate::visitor::Visitor;
use crate::ast::statement::Statement::*;
use crate::ast::statement::*;
use crate::ast::expression::Expression::*;
use crate::ast::expression::*;
use crate::asm_compile::asm_writer::ASMWriter;
use crate::writer::*;
use crate::asm_compile::Register;
use crate::asm_compile::*;

impl<'pr> Visitor for ASMWriter<'pr> {
    fn visit_block_statement(&mut self, s: &BlockStmt) {
        self.append("block");
    }

    fn visit_variable_declarator(&mut self, v: &Variable) {
        self.append("variable_dco");
    }

    fn visit_while_statement(&mut self, w: &WhileStmt) {
        self.append("variable_dco");
    }

    fn visit_variable_declaration(&mut self, v: &VariableDec) {
        self.append("variable_de");
    }

    fn visit_if_statement(&mut self, i: &IfStmt) {
        self.append("if");
    }

    fn visit_switch_statement(&mut self, s: &SwitchStmt) {
        self.append("switch");
    }

    fn visit_case(&mut self, case: &CaseStmt) {
        self.append("case");
    }

    fn visit_for_statement(&mut self, f: &ForStmt) {
        self.append("for");
    }

    fn visit_break_statement(&mut self, f: &BreakStmt) {
        self.append("break");
    }

    fn visit_return_statement(&mut self, r: &ReturnStmt) {
        self.append("ret");
    }

    fn visit_continue_statement(&mut self, c: &ContinueStmt) {
        self.append("continue");
    }

    fn visit_function_declaration(&mut self, f: &FunctionDec) {
        self.append("func_dec");
    }

    fn visit_option_expression(&mut self, exp: &Option<Box<Expression>>) {
        self.append("opt_exp");
    }

    fn visit_expression(&mut self, exp: &Expression) {
        match exp {
            NumericLiteral(ref n) => {
                self.change_register();
                self.append(&format!("\t{}\t${}, %{}", ASM_MOVE, n.to_string(), self.reg.to_string()))
            }
            StringLiteral(ref s) => {
                self.change_register();
                self.append(&format!("\t{}\t{}, ${}", ASM_MOVE, s.to_string(), self.reg.to_string()))
            }
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
    }

    fn visit_binary_expression(&mut self, b: &BinaryExp) {
        self.visit_expression(&b.right);
        self.append(NEW_LINE);
        self.visit_expression(&b.left);
        self.append(NEW_LINE);
        self.write_asm_op(&b.operator);
        self.append(NEW_LINE);
        self.append("#bin exp end\n")
    }

    fn visit_assign(&mut self, a: &AssignmentExp) {
        self.append("assign_exp");
        self.append(NEW_LINE);
    }

    fn visit_unary_expression(&mut self, u: &UnaryExp) {
        self.append("unary_exp");
    }

    fn visit_update_expression(&mut self, u: &UpdateExp) {
        self.append("update_exp");
    }

    fn visit_member_expression(&mut self, m: &MemberExp) {
        self.append("member_exp");
    }

    fn visit_logical_expression(&mut self, l: &LogicalExp) {
        self.append("log exp");
    }

    fn visit_call_expression(&mut self, e: &CallExp) {
        self.append("call_exp");
    }

    fn visit_object_expression(&mut self, o: &ObjectExp, id: String) {
        self.append("obj exp");
    }

    fn visit_property_expression(&mut self, id: &str, p: &Property) {
        self.append("prop exp");
    }
}