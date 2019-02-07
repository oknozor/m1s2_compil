use std::collections::HashMap;
use std::any::Any;
use self::LexicalEnvType::*;
use crate::ast::expression::Loc;

pub struct RunnerOption;
impl RunnerOption {
    pub fn with_loc(loc: &Loc) -> bool{
        true
    }
}

// https://www.ecma-international.org/ecma-262/#sec-executable-code-and-execution-contexts
pub struct LexicalEnv<'scp> {
    pub parent: &'scp Option<LexicalEnv<'scp>>,
    pub env_type: &'scp LexicalEnvType,
    pub env_record: HashMap<&'scp str, &'scp LexicalEnv<'scp>>,
    pub used_references: HashMap<&'scp str, &'scp Any>,
    pub declared_references: HashMap<&'scp str, &'scp Any>
}

impl<'scp> LexicalEnv<'scp> {

   pub fn init_root() -> Self {
        LexicalEnv {
            parent: &None,
            env_type: &Root,
            env_record: HashMap::new(),
            used_references: HashMap::new(),
            declared_references: HashMap::new()
        }
    }
}

pub enum LexicalEnvType {
    Root,
    FunctionDeclaration,
    BlockStatement,
    Catch,
    TryStatement,
}