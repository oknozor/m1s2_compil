use crate::ast::expression::*;
use crate::ast::expression::Expression::*;
use crate::ast::statement::*;
use crate::ast::statement::Statement::*;
use crate::c_compile::*;
use crate::c_compile::c_write_utils::*;
use crate::c_compile::c_writer::CWriter;
use crate::visitor::Visitor;

impl<'pr> Visitor for  CWriter<'pr> {

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
            SwitchCase(case) => self.visit_case(case),
            _ => (),
        };
    }


    fn visit_while_statment(&mut self, w: &WhileStmt) {
        self.append(WHILE);
        self.append(PARENTHESIS_LEFT);
        self.visit_expression(&w.test);
        self.append(PARENTHESIS_RIGHT);
        self.append(BRACKET_LEFT);
        self.visit_statement(&w.body);
        self.append(BRACKET_RIGHT);
    }

    fn visit_block_statement(&mut self, s: &BlockStmt) {
        s.body.iter().for_each(|statement| self.visit_statement(statement))
    }

    fn visit_variable_declarator(&mut self, v: &Variable) {
        self.append(DATABOX);
        self.append(&v.to_string());
        if let Some(box init) = &v.init {
            self.append(EQ);
            let var_as_databox_str = &self.get_ref_as_str(init, v.id.name.clone());
            self.append(var_as_databox_str);
        };
        self.append(SEMI_COL);
    }

    fn visit_while_statement(&mut self, w: &WhileStmt) {
        self.append(WHILE);
        self.append(PARENTHESIS_LEFT);
        self.visit_expression(&w.test);
        self.append(PARENTHESIS_RIGHT);
        self.append(BRACKET_RIGHT);
        self.visit_statement(&w.body);
        self.append(BRACKET_RIGHT);
    }

    fn visit_variable_declaration(&mut self, v: &VariableDec) {
        v.declarations.iter().for_each(|declaration| {
            if let box VariableDeclarator(declarator) = declaration {
                self.visit_variable_declarator(&declarator);
                self.append(NEW_LINE);
            }
        });
    }

    fn visit_if_statement(&mut self, i: &IfStmt) {
        self.append(IF);
        self.append(PARENTHESIS_LEFT);
        self.visit_expression(&i.test);
        self.append(PARENTHESIS_RIGHT);
        self.append(BRACKET_LEFT);
        self.visit_statement(&i.consequent);
        self.append(BRACKET_RIGHT);
        if let Some(alternate) = &i.alternate {
            self.append(ELSE);
            self.append(BRACKET_LEFT);
            self.visit_statement(alternate);
            self.append(BRACKET_RIGHT);
        }
    }

    fn visit_switch_statement(&mut self, s: &SwitchStmt) {
        self.append(SWITCH);
        self.append(BRACKET_LEFT);
        self.visit_expression(&s.discriminant);
        &s.cases.iter().for_each(|case| {
            self.visit_case(case);
        });
        self.append(BRACKET_RIGHT);
    }

    fn visit_case(&mut self, case: &CaseStmt) {
        self.append(CASE);
        self.append(COL);
        self.append(" ");
        if let Some(test) = &case.test {
            self.visit_expression(test);
        }
        self.append(NEW_LINE);
        &case.consequent.iter().for_each(|consequent| {
            self.visit_statement(consequent);
        });
        self.append(BREAK);
        self.append(SEMI_COL);
    }

    fn visit_for_statement(&mut self, f: &ForStmt) {
        self.visit_option_expression(&f.init);
        self.append(SEMI_COL);
        self.append(FOR);
        self.append(PARENTHESIS_LEFT);

        self.append(SEMI_COL);
        self.visit_option_expression(&f.test);
        self.append(SEMI_COL);
        self.visit_option_expression(&f.update);
        self.append(PARENTHESIS_RIGHT);
        self.append(BRACKET_LEFT);
        self.append(NEW_LINE);
        self.visit_statement(&f.body);
        self.append(BRACKET_RIGHT);
    }

    fn visit_break_statement(&mut self, f: &BreakStmt) {
        self.append(BREAK);
        self.append(SEMI_COL);
    }

    fn visit_return_statement(&mut self, r: &ReturnStmt) {
        self.append(RETURN);

        match &r.argument {
            Some(box NumericLiteral(_)) |
            Some(box StringLiteral(_)) => {
                self.append(NEW);
                self.append(PARENTHESIS_LEFT);
                self.visit_option_expression(&r.argument);
                self.append(PARENTHESIS_RIGHT);
            }
            Some(box Identifier(id)) => self.append(&id.name),
            _ => self.visit_option_expression(&r.argument),
        };
        self.append(SEMI_COL);
    }

    fn visit_continue_statement(&mut self, c: &ContinueStmt) {
        self.append(CONTINUE);
        self.append(SEMI_COL);
    }

    fn visit_function_declaration(&mut self, f: &FunctionDec) {
        self.append(&f.id.to_string());
        self.append(PARENTHESIS_LEFT);
        f.params.iter().for_each(|param| {
            self.append(DATABOX);
            self.append(param.name.as_str());
        });

        self.append(PARENTHESIS_RIGHT);
        self.append(BRACKET_LEFT);
        self.append(NEW_LINE);
        self.visit_block_statement(&f.body);
        self.append(BRACKET_RIGHT);
    }

    fn visit_option_expression(&mut self, exp: &Option<Box<Expression>>) {
        if let Some(expression) = exp {
            self.visit_expression(&expression);
        }
    }

    fn visit_expression(&mut self, exp: &Expression) {
        match exp {
            NumericLiteral(ref n) => self.append(&n.to_string()),
            StringLiteral(ref s) => self.append(&s.to_string()),
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
        self.append(SEMI_COL);
    }

    fn visit_binary_expression(&mut self, b: &BinaryExp) {
        let mut option_left = &b.left.try_as_identifier();
        let mut option_right = &b.right.try_as_identifier();
        let mut temp_expression = String::new();
        if b.has_parenthesis() { self.append(PARENTHESIS_LEFT); }

        if b.has_idendifier(option_left, option_right) {
            self.append(bin_op_to_c(&b.operator));
            self.append(PARENTHESIS_LEFT);

            self.append_option_identifier_or_visit_expression(option_left, &b.left);
            self.append(COMA);
            self.append_option_identifier_or_visit_expression(option_right, &b.right);

            self.append(PARENTHESIS_RIGHT);
        } else {
            self.visit_expression(&b.left);
            self.append(&b.operator);
            self.visit_expression(&b.right);
        }
        if b.has_parenthesis() { self.append(PARENTHESIS_RIGHT); }
    }


    /// if the assignment operator is equal,  use the new macro from std
    /// else generate a binary expression from the assignment and visit it
    fn visit_assign(&mut self, a: &AssignmentExp) {
        if let box Identifier(id) = &a.left {
            let identifier = &id.to_string();
            self.append(&id.name);
            self.append(EQ);
            if &a.operator != EQ {
                let assign_bin_op = assign_to_c(id.clone(),
                                                &a.operator,
                                                a.right.clone(),
                                                a.loc.clone());
                self.visit_binary_expression(&assign_bin_op);
            } else {
                self.append(NEW);
                self.append(PARENTHESIS_LEFT);
                self.visit_expression(&a.right);
                self.append(PARENTHESIS_RIGHT);
            }
        }
    }

    fn visit_unary_expression(&mut self, u: &UnaryExp) {
        self.append(u.operator.as_str());
        self.visit_expression(&u.argument);
    }

    fn visit_update_expression(&mut self, u: &UpdateExp) {
        let arg = &u.argument.try_as_identifier();
        if let Some(idendifier) = *arg {
            let update_c_string = update_to_c(&u.operator, &idendifier.to_string());
            self.append(update_c_string
                .expect("unable to parse update expression").as_str());
        }
    }

    fn visit_member_expression(&mut self, m: &MemberExp) {
        self.visit_expression(&m.object);
        self.visit_expression(&m.property);
    }

    fn visit_logical_expression(&mut self, l: &LogicalExp) {
        self.visit_expression(&l.left);
        self.append(l.operator.as_str());
        self.visit_expression(&l.right);
    }

    fn visit_call_expression(&mut self, e: &CallExp) {
        let mut standard_lib_call = false;
        if let box Identifier(id) = &e.callee {
            if STD_LIB.contains(&id.name.as_str()) {
                standard_lib_call = true;
            }
            self.append(&id.to_string());
        }

        self.append(PARENTHESIS_LEFT);
        let last = e.arguments.len() - 1;

        e.arguments.iter().enumerate().for_each(|(i, expression)| {
            if !standard_lib_call {
                self.append(NEW);
                self.append(PARENTHESIS_LEFT);
            }

            if let box Identifier(id) = expression {
                self.append(&id.to_string());
            }
            self.visit_expression(expression);

            if i != last { self.append(COMA); };
            if !standard_lib_call { self.append(PARENTHESIS_RIGHT); }
        });
        self.append(PARENTHESIS_RIGHT);
    }

    fn visit_object_expression(&mut self, e: &ObjectExp, id: String) {
        self.append(NEW_DICT);
        self.append(SEMI_COL);
        self.append(NEW_LINE);

        e.properties.iter().for_each(|prop| {
            self.visit_property_expression( &id, prop);
        })
    }

    fn visit_property_expression(&mut self, id: &str, p: &Property) {
        let prop_id = p.key.try_as_string_from_lit().or(p.key.try_as_string_from_identifier());
        let prop_id = prop_id.expect("unable to parse object property id");
        let value = self.get_ref_as_str(&p.value, id.to_string());
/*
        self.append(&format!("dictionary_add({}.dict, \"{}\" , new)", id, prop_id, &value.clone()));
*/
    }
}