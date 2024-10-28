//declarations.rs
//Sahil Athalye: a.sahil@wustl.edu
//declares Play type, program constants, and whinge boolean

use std::sync::atomic::AtomicBool;

pub type Play = Vec<(usize, String, String)>;

pub const SCRIPT_GEN_FAILURE:u8 = 2;

pub static SHOULD_COMPLAIN: AtomicBool = AtomicBool::new(false);