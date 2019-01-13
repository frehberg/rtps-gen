#[allow(unused_imports)]
use std::vec::Vec;
#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub enum Foo {
    LABEL0(i32),
    LABEL1(i16),
    LABEL2(i16),
    default(u8),
}
//
// TODO custom de-/serializer
//
