use crate::scope::scope::Scope;
use crate::scope::scope::ScopeType::*;


pub trait ToCode {
    fn to_code(self)-> String;
}
impl ToCode for Scope {
    fn to_code(self) -> String {
        match self.scope_type {
            Function=> 
            Var=>
            While=>
            Switch=>
            Return=>
            Case=>
            If=>
            Else=>
            For=>
            Root=>
            Block=>
        }
    }
}