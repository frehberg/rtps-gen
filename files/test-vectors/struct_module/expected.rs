#[allow(unused_imports)]
use std::vec::Vec;
#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(non_snake_case)]
pub mod A {
    #[allow(unused_imports)]
    use serde_derive::{Serialize, Deserialize};

    //
    //
    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    #[derive(Serialize, Deserialize)]
    #[derive(Clone, Debug)]
    pub struct Foo {
        pub m_l1: i32,
        pub m_l2: i32,
        pub m_d: f64,
    }
}
