#[allow(unused_imports)]
use std::vec::Vec;
#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(non_snake_case)]
pub mod ModuleA {
    #[allow(unused_imports)]
    use serde_derive::{Serialize, Deserialize};

    //
    //
    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub type dim1 = [i32;2];

    //
    //
    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub type seq_long = Vec<i32>;
}
