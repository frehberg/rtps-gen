# OMG IDL v4 grammar for pest

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Object Management Group [Interface Definition Language v4.1](http://www.omg.org/spec/IDL/4.1/) grammar for [pest](https://github.com/pest-parser/pest)

## Status

This grammar was manually converted from the EBNF Consolidated IDL Grammar Annex of the specification. 

## Usage

pest_idl_v4_grammar requires [Cargo and Rust](https://www.rust-lang.org/en-US/downloads.html).

Add the following to `Cargo.toml`:

```toml
rtps-idl-grammar = "*"
```

and in your Rust `lib.rs` or `main.rs`:

```rust
extern crate pest;
use pest::Parser;
extern crate rtps_idl_grammar;
use rtps_idl_grammar::{Rule,IdlParser};


```

## License:

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Credits

Kevin Pansky: [pest_idl_v4_grammar](https://github.com/kpansky/pest_idl_v4_grammar)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

