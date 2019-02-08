use crate::runner::visitor::Visitor;
use crate::runner::scope::Scope;
use crate::runner::handler::Handler;
use crate::ast::statement::WhileStatement;

pub struct ScopeBuilder {
    pub global_scope: Scope,
}

impl Visitor for  ScopeBuilder {}
impl Handler for ScopeBuilder {
    fn handle_while_statement(&mut self, w: &WhileStatement){
    }

}

impl ScopeBuilder {
    fn new() -> Self {
        ScopeBuilder {
            global_scope: Scope::new()
        }
    }
}