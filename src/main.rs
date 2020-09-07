use serde::{Serialize, Deserialize};
use abomonation_derive::Abomonation;

extern crate savefile;
#[macro_use]
extern crate savefile_derive;

#[allow(dead_code, unused_imports)]
#[path = "../storeddata_generated.rs"]
mod flat;

#[allow(dead_code, unused_imports)]
#[path = "../storeddata_capnp.rs"]
mod storeddata_capnp;

#[derive(Savefile, Abomonation, Serialize, Deserialize)]
pub enum StoredVariants {
    YesNo(bool),
    Small(u8),
    Signy(i64),
    Stringy(String),
}

#[derive(Savefile, Abomonation, Serialize, Deserialize)]
pub struct StoredData {
    pub variant: StoredVariants,
    pub opt_bool: Option<bool>,
    pub vec_strs: Vec<String>,
    pub range: std::ops::Range<usize>,
}


fn main() {

    println!("Hello, world!");

}
