// Copyright (C) 2017  Kevin Pansky
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate pest;
extern crate pest_idl_v4_grammar;

use std::fs::File;
use std::io::Read;
use pest::Parser;
use pest_idl_v4_grammar::{Rule, IdlParser};

#[test]
fn decimal_integer_literal() {
    parses_to! {
        parser: IdlParser,
        input: "1234",
        rule: Rule::decimal_integer_literal,
        tokens: [
            decimal_integer_literal(0, 4)
        ]
    };
}

#[test]
fn character_literal() {
    parses_to! {
        parser: IdlParser,
        input: "'A'",
        rule: Rule::character_literal,
        tokens: [
            character_literal(0, 3)
        ]
    };
}

#[test]
fn floating_pt_literal() {
    parses_to! {
        parser: IdlParser,
        input: "1234.56",
        rule: Rule::floating_pt_literal,
        tokens: [
            floating_pt_literal(0, 7)
        ]
    };
}

#[test]
fn example() {
    let mut file = File::open("tests/example.idl").unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    IdlParser::parse(Rule::specification, &data).unwrap_or_else(|e| panic!("{}", e));
}
