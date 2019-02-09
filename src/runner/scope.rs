use crate::token::token::Token;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;
use crate::ast::statement::Statement;
use crate::ast::statement::FunctionDeclaration;
use crate::ast::expression::Identifier;
use crate::ast::statement::BlockStatement;
use crate::ast::expression::Loc;
use crate::ast::expression::Pos;


// First step to solve recursive scoping is to define a way to manage reference cycle,
// and dangling reference. Luckily , the rust manual is here to help :
// https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
// https://doc.rust-lang.org/std/rc/struct.Rc.html


// The RefCell type enforce borrowing rules at runtime instead of compile time,
// it allows unsafe scenario such as reference cycle. To prevent this kind of disaster
// we will use Reference counter ( Rc<T> ) and weak pointer ( Weak<T>)
pub type Children<T> = RefCell<HashMap<String, Rc<Scope<T>>>>;
pub type Parent<T> = RefCell<Weak<Scope<T>>>;

#[derive(Debug)]
pub struct Scope<T> {
    pub stack: Vec<Token>,
    pub content: Rc<T>,
    pub parent: Parent<T>,
    pub children: Children<T>,
}

pub trait Scoped {
    fn get_name(&self) -> Option<String>;
}

impl<T> Scope <T> where T : Scoped {
    // get a new scope with a reference counter on it
    pub fn new(s: T) -> Rc<Self> {
        Rc::new(Scope {
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(HashMap::new()),
            stack: vec![],
            content: Rc::new(s)
        })
    }

    pub fn get_content(&self) -> Rc<T> where T: Scoped {
        Rc::clone(&self.content)
    }

    // convert AST content to a scoped reference
    // add a new weak pointer to the parent weak count
    // add the newly created scope to the child list of the current scope
    // and then increment the strong counter reference on the child
    pub fn add_child(&self, self_ref: Rc<Scope<T>>,  content: T) where T : Scoped{
        let child_scope = Scope::new(content);
        *child_scope.parent.borrow_mut() = Rc::downgrade(&self_ref);

        self.children.borrow_mut()
            .insert(child_scope.content.get_name().unwrap(), Rc::clone(&child_scope));
    }
}


#[test]
fn should_add_child() {
    let function = FunctionDeclaration {
        id: Identifier { name: "luke".to_string(),
            loc: Loc {
                start: Pos { line: 0, column: 0 },
                end: Pos { line: 0, column: 0 }
            }
        },
        params: vec![],
        body: BlockStatement {
            body: vec![Box::new(Statement::EmptyStatement)]
        }
    };

    let statement = Statement::FunctionDeclaration(function);
    let anakin = Scope::new(Statement::EmptyStatement);
    let ref_anakin = Rc::new(anakin);
    ref_anakin.add_child( Rc::clone(&ref_anakin), statement);

    assert_eq!(true, ref_anakin.children.borrow().contains_key("luke"));

    let luke = ref_anakin.children.borrow().get("luke").unwrap().clone();
    let the_empire_strikes_back = luke.parent.borrow_mut();

    assert_eq!(true, the_empire_strikes_back.upgrade().unwrap().children.borrow().contains_key("luke"));
}

