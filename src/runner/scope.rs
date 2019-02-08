use crate::token::binary_operator::BinaryOp;
use crate::token::token::Token;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::token::literal::Literal::*;
use crate::token::token::Token::Literal;
use std::rc::Weak;


// First step to solve recursive scoping is to define a way to manage reference cycle,
// and dangling reference. Luckily , the rust manual is here to help :
// https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
// https://doc.rust-lang.org/std/rc/struct.Rc.html


// The RefCell type enforce borrowing rules at runtime instead of compile time,
// it allows unsafe scenario such as reference cycle. To prevent this kind of disaster
// we will use Reference counter ( Rc<T> ) and weak pointer ( Weak<T>)
pub type Children = RefCell<HashMap<String, Rc<Scope>>>;
pub type Parent = RefCell<Weak<Scope>>;

pub struct Scope {
    pub name: String,
    pub stack: Vec<Token>,
    pub parent: Parent,
    pub children: Children,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            name: "".to_string(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(HashMap::new()),
            stack: vec![],
        }
    }
    fn give_birth(&mut self, name: String) {

    }
    fn new_named(name: &str) -> Self {
        Scope {
            name: name.to_string(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(HashMap::new()),
            stack: vec![],
        }
    }
    fn add_child(&mut self, scope: &Rc<Scope>) {
        self.children.borrow_mut()
            .insert(scope.name.to_string(), Rc::clone(scope));
    }
}


#[test]
fn should_add_child() {
    let scope = Scope::new_named("Luke");
    let child = Rc::new(scope);
    let parent = RefCell::new(Scope::new_named("Anakin"));
    parent.borrow_mut().add_child(&child);

    assert_eq!(true, parent.borrow().children.borrow().contains_key("Luke"));
}

#[test]
fn should_add_parent() {
    let scope = Scope::new_named("Leia");
    let child = Rc::new(scope);
    let parent = Rc::new(Scope::new_named("Anakin"));

    *child.parent.borrow_mut() = Rc::downgrade(&parent);
    let parent_from_child = child.parent.borrow_mut().upgrade().unwrap();

    assert_eq!(parent_from_child.name, "Anakin");
}

#[test]
fn should_get_ref_count() {
    let leia = Scope::new_named("Leia");
    let anakin = Scope::new_named("Anakin");

    let rc_leia = Rc::new(leia);
    assert_eq!(0, Rc::weak_count(&rc_leia));
    assert_eq!(1, Rc::strong_count(&rc_leia));

    let rc_anakin = Rc::new(anakin);
    assert_eq!(0, Rc::weak_count(&rc_anakin));
    assert_eq!(1, Rc::strong_count(&rc_anakin));

    // declare leia's father as a cloned "weak" version of anakin's pointer
    *rc_leia.parent.borrow_mut() = Rc::downgrade(&rc_anakin);
    assert_eq!(1, Rc::weak_count(&rc_anakin));

    // access anakin via an updrade of the weak pointer
    let parent_from_child = rc_leia.parent
                                   .borrow_mut()
                                   .upgrade()
                                   .unwrap();
    assert_eq!(2, Rc::strong_count(&rc_anakin));


    // mutate leia from upgraded weak pointer
    parent_from_child.children
                     .borrow_mut()
                     .insert(rc_leia.name.clone(),
                             Rc::clone(&rc_leia));
    assert_eq!(2, Rc::strong_count(&rc_leia));
}

