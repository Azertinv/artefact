use crate::trit::Trit;
use crate::byte::Byte;
use crate::operation::Operation;

use gdnative::prelude::*;

const ZERO: i64 = 0i64;
const ONE: i64 = 1i64;
const TERN: i64 = 2i64;
const EQUAL: i64 = 3i64;
const ADD: i64 = 4i64;
const SUB: i64 = 5i64;
const MUL: i64 = 6i64;
const DIV: i64 = 7i64;
const CLEAR: i64 = 8i64;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Calculator {
    lhs: Byte,
    rhs: Option<Byte>,
    op: Option<Operator>,
    last_lhs: Option<Byte>,
    last_rhs: Option<Byte>,
    last_op: Option<Operator>,
}

#[methods]
impl Calculator {
    fn new(_owner: &Node) -> Self {
        Calculator{
            lhs: Byte::ZERO,
            rhs: None,
            op: None,
            last_lhs: None,
            last_rhs: None,
            last_op: None,
        }
    }

    fn get_value_helper(opt_value: &Option<Byte>) -> GodotString {
        if let Some(value) = opt_value {
            GodotString::from_str(format!("{}", value))
        } else {
            GodotString::from_str("")
        }
    }

    #[export]
    fn get_lhs(&self, _owner: &Node) -> GodotString {
        GodotString::from_str(format!("{}", self.lhs))
    }

    #[export]
    fn get_last_lhs(&self, _owner: &Node) -> GodotString {
        Self::get_value_helper(&self.last_lhs)
    }

    #[export]
    fn get_rhs(&self, _owner: &Node) -> GodotString {
        Self::get_value_helper(&self.rhs)
    }

    #[export]
    fn get_last_rhs(&self, _owner: &Node) -> GodotString {
        Self::get_value_helper(&self.last_rhs)
    }

    fn get_op_helper(opt_op: &Option<Operator>) -> GodotString {
        if let Some(op) = opt_op {
            GodotString::from_str(match op {
                Operator::Add => "+",
                Operator::Sub => "-",
                Operator::Mul => "*",
                Operator::Div => "/",
            })
        } else {
            GodotString::from_str("")
        }
    }

    #[export]
    fn get_op(&self, _owner: &Node) -> GodotString {
        Self::get_op_helper(&self.op)
    }

    #[export]
    fn get_last_op(&self, _owner: &Node) -> GodotString {
        Self::get_op_helper(&self.last_op)
    }

    #[export]
    fn push_button(&mut self, _owner: &Node, button: i64) {
        match button {
            ZERO | ONE | TERN if self.op.is_some() => {
                let trit = match button {
                    ZERO => Trit::ZERO,
                    ONE => Trit::ONE,
                    TERN => Trit::TERN,
                    _ => panic!(),
                };
                if self.rhs.is_none() {
                    self.rhs = Some(Byte::ZERO)
                }
                if let Some(mut rhs) = self.rhs {
                    rhs = Byte::shift(rhs, 1).0;
                    rhs.trits[0] = trit;
                    self.rhs = Some(rhs);
                }
            },
            ZERO | ONE | TERN => {
                let trit = match button {
                    ZERO => Trit::ZERO,
                    ONE => Trit::ONE,
                    TERN => Trit::TERN,
                    _ => panic!(),
                };
                self.lhs = Byte::shift(self.lhs, 1).0;
                self.lhs.trits[0] = trit;
            },
            EQUAL if self.rhs.is_some() && self.op.is_some() => {
                let op = self.op.unwrap();
                let rhs = self.rhs.unwrap();
                if !(op == Operator::Div && rhs == Byte::ZERO) {
                    let lhs = match op {
                        Operator::Add => Byte::add(self.lhs, rhs, Byte::ZERO).0,
                        Operator::Sub => Byte::sub(self.lhs, rhs, Byte::ZERO).0,
                        Operator::Mul => Byte::mul(self.lhs, rhs).0,
                        Operator::Div => Byte::div(self.lhs, rhs).0,
                    };
                    self.last_lhs = Some(self.lhs);
                    self.last_rhs = self.rhs;
                    self.last_op = self.op;
                    self.lhs = lhs;
                    self.op = None;
                    self.rhs = None;
                }
            },
            EQUAL if self.last_rhs.is_some() && self.last_op.is_some() => {
                let op = self.last_op.unwrap();
                let rhs = self.last_rhs.unwrap();
                if !(op == Operator::Div && rhs == Byte::ZERO) {
                    let lhs = match op {
                        Operator::Add => Byte::add(self.lhs, rhs, Byte::ZERO).0,
                        Operator::Sub => Byte::sub(self.lhs, rhs, Byte::ZERO).0,
                        Operator::Mul => Byte::mul(self.lhs, rhs).0,
                        Operator::Div => Byte::div(self.lhs, rhs).0,
                    };
                    self.last_lhs = Some(self.lhs);
                    self.lhs = lhs;
                    self.op = None;
                    self.rhs = None;
                }
            },
            EQUAL => {},
            ADD => { self.op = Some(Operator::Add); },
            SUB => { self.op = Some(Operator::Sub); },
            MUL => { self.op = Some(Operator::Mul); },
            DIV => { self.op = Some(Operator::Div); },
            CLEAR => {
                self.last_lhs = Some(self.lhs);
                self.last_rhs = self.rhs;
                self.last_op = self.op;
                self.lhs = Byte::ZERO;
                self.op = None;
                self.rhs = None;
            },
            _ => { godot_print!("Unrecognized button: {}", button)},
        }
    }
}
