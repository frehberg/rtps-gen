// Copyright (C) 2019  Frank Rehberger
// Copyright (C) 2017  Kevin Pansky

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Object Management Group IDL v4.0 grammar
//!
//! Contains grammar for OMG IDL v4.0.

extern crate pest;
#[macro_use]
extern crate pest_derive;

/// OMG IDL v4 parser
#[derive(Parser)]
#[grammar = "grammar/idl_v4.pest"]
pub struct IdlParser;
