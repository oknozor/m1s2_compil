use crate::token::token::Token;
use crate::ast::statement::Statement;
use crate::ast::statement::BlockStatement;
use std::rc::Rc;
use crate::ast::expression::Expression;
use crate::ast::expression::TokenStream;
use crate::ast::expression::ToToken;
use crate::scope::scope::ScopeType::*;
use crate::ast::statement::WhileStatement;
use crate::ast::statement::ForStatement;
use crate::ast::statement::IfStatement;
use crate::ast::statement::ReturnStatement;
use crate::ast::statement::BreakStatement;
use crate::ast::statement::ContinueStatement;
use crate::ast::statement::SwitchCase;
use crate::ast::statement::SwitchStatement;
use crate::token::control_flow::ControlFlow::Continue;
use crate::token::control_flow::ControlFlow::Break;

// TODO :  add Label for continue and break
#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub name: Option<Rc<String>>,
    pub parents: Vec<Rc<String>>,
    pub childs: Vec<Scope>,
    pub token_stream: TokenStream,
    pub scope_type: ScopeType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ScopeType {
    Function,
    Var,
    While,
    Switch,
    Return,
    Case,
    If,
    Else,
    For,
    Root,
    Block,
}

impl Scope {
    // Create a new named scope and increase the reference counter on all ancestors
    pub fn new_named(name: String, parent: Option<&Scope>, sc_type: ScopeType) -> Self {
        let mut scope = Scope::new(parent, sc_type);
        scope.name = Some(Rc::new(name));
        scope
    }

    pub fn new(parent: Option<&Scope>, sc_type: ScopeType) -> Self {
        let mut parents;
        let parent_name: String;
        if let Some(parent) = parent {
            // Cloning the entire vec produce a new vec with updated reference count
            parents = parent.parents.clone();

            if let Some(parent_name) = &parent.name {
                parents.push(Rc::clone(&parent_name));
            }
        } else {
            parents = vec![];
        }

        Scope {
            name: None,
            parents,
            childs: vec![],
            token_stream: vec![],
            scope_type: sc_type,
        }
    }

    // Used only once to init the root scope
    pub fn init_root(parent: Option<Scope>) -> Self {
        let mut parents;
        let parent_name: String;
        if let Some(parent) = parent {
            parents = parent.parents.clone();

            if let Some(parent_name) = parent.name {
                parents.push(Rc::clone(&parent_name));
            }
        } else {
            parents = vec![];
        }
        Scope {
            name: None,
            parents, // No parent, like Batman
            childs: vec![],
            token_stream: vec![],
            scope_type: ScopeType::Root,
        }
    }

    // Explore the AST to generate new scopes
    pub fn build(&mut self, statements: &Vec<Box<Statement>>) {
        statements.iter().for_each(|statement| {
            self.explore_statement(statement)
        });
    }

    // 1. Extract named reference into scope child
    // 2. Tokenize expression on the fly
    fn explore_statement(&mut self, statement: &Box<Statement>) -> () {
        // Todo make name condition inner so we can handle error on unkown statement
        if let Some(name) = statement.get_name().clone() {
            match statement {
                box Statement::FunctionDeclaration(f) =>
                    {
                        let mut function_scope = Scope::new_named(name, Some(self), Function);
                        let body = &f.body.body;
                        function_scope.build(&body);
                        let name = f.id.name.clone();
                        self.childs.push(function_scope);
                    }
                box Statement::VariableDeclarator(declarator) =>
                    {
                        let var_name = declarator.id.name.clone();
                        let mut var_scope = Scope::new_named(var_name.clone(), Some(self), Var);

                        if let Some(initialization) = &declarator.init {
                            var_scope.token_stream.extend(initialization.to_token());
                        } else {
                            var_scope.token_stream.push(Token::Undefined);
                        };

                        self.childs.push(var_scope);
                    }
                _ => ()
            };
        } else {
            match statement {
                box Statement::VariableDeclaration(v) => self.build(&v.declarations),
                box Statement::BlockStatement(block) => self.build_block_scope(&block),
                box Statement::ExpressionStatement(exp_stmt) => self.tokenize(&exp_stmt.expression),
                box Statement::SwitchStatement(switch) => self.build_switch_scope(&switch),
                box Statement::SwitchCase(case) => self.build_case_scope(&case),
                box Statement::IfStatement(if_stmt) => self.build_if_scope(&if_stmt),
                box Statement::ForStatement(for_stmt) => self.build_for_scope(&for_stmt),
                box Statement::WhileStatement(while_stmt) => self.build_while_scope(&while_stmt),
                box Statement::ContinueStatement(continue_statment) => self.build_continue_scope(&continue_statment),
                box Statement::BreakStatement(break_statement) => self.build_break_scope(&break_statement),
                box Statement::ReturnStatement(return_statement) => self.build_return_scope(&return_statement),
                _ => ()
            };
        }
    }

    fn build_while_scope(&mut self, while_stmt: &WhileStatement) {
        let mut while_scope = Scope::new(Some(self), While);
        while_scope.tokenize(&while_stmt.test);
        while_scope.explore_statement(&while_stmt.body);
        self.childs.push(while_scope);
    }

    fn build_for_scope(&mut self, for_stmt: &ForStatement) {
        let mut for_scope = Scope::new(Some(self), For);
        if let Some(init) = &for_stmt.init { for_scope.tokenize(init); };
        if let Some(test) = &for_stmt.test { for_scope.tokenize(test); };
        if let Some(update) = &for_stmt.update { for_scope.tokenize(update); };
        for_scope.explore_statement(&for_stmt.body);
        self.childs.push(for_scope);
    }

    fn build_if_scope(&mut self, if_stmt: &IfStatement) {
        let mut if_scope = Scope::new(Some(self), If);
        if_scope.tokenize(&if_stmt.test);
        if_scope.explore_statement(&if_stmt.consequent);
        if let Some(alt) = &if_stmt.alternate {
            let mut else_scope = Scope::new(Some(self), Else);
            else_scope.tokenize(&if_stmt.test);
            else_scope.explore_statement(alt);
            if_scope.explore_statement(alt);
        };
        self.childs.push(if_scope);
    }

    fn build_case_scope(&mut self, case: &SwitchCase) {
        let mut case_scope = Scope::new(Some(self), Case);
        if let Some(test) = &case.test { case_scope.tokenize(test); };
        case_scope.build(&case.consequent);
        self.childs.push(case_scope);
    }

    fn build_switch_scope(&mut self, switch: &SwitchStatement) {
        let mut switch_scope = Scope::new(Some(self), Switch);
        switch_scope.tokenize(&switch.discriminant);
        switch_scope.build(&switch.cases);
        self.childs.push(switch_scope);
    }

    fn build_block_scope(&mut self, block: &BlockStatement) {
        let mut block_scope = Scope::new(Some(self), Block);
        block_scope.build(&block.body);
        self.childs.push(block_scope);
    }

    fn build_continue_scope(&mut self, continue_stmt: &ContinueStatement) {
        self.token_stream.push(Token::ControlFlow(Continue));
    }

    fn build_return_scope(&mut self, return_stmt: &ReturnStatement) {
        let mut return_scope = Scope::new(Some(self), ScopeType::Return);
        if let Some(args) = &return_stmt.argument {
            return_scope.tokenize(args)
        }
        self.childs.push(return_scope);

    }

    fn build_break_scope(&mut self, break_stmt: &BreakStatement) {
        self.token_stream.push(Token::ControlFlow(Break));
    }

    // Dispatch expression arrays
    fn explore_expression(&mut self, expression: &Vec<Box<Expression>>) {}

    // Push expressions as token on the scope token_stream
    fn tokenize(&mut self, expression: &Box<Expression>) {
        self.token_stream.extend(expression.to_token());
    }
}

