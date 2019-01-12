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
pub struct Foo {
    m_l1: i32,
    m_l2: i32,
    m_d: f64,
}
