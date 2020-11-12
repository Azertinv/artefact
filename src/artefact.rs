// use crate::trit::Trit;
// use crate::byte::Byte;
// use crate::operation::Operation;

use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Artefact;

#[methods]
impl Artefact {
    fn new(_owner: &Node) -> Self {
        Artefact
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Artefact Initialized");
    }

    #[export]
    fn prout(&self, _owner: &Node, value: i64) {
        godot_print!("arte, {}", value);
    }
}
