[![Apache 2.0 licensed][licence-badge]][licence-url]
# RTPS IDL to Rust code generator library

A library reading IDL input and generating corresponding Rust data types

## Intended Features
* Converting IDL types to Rust-types
* Converting IDL types to TokenStreams (hygiene proc-macros not stable yet)

## Add dependency
Put this in your Cargo.toml:
```toml
## Cargo.toml file
[dependencies]
rtps-idl = "0.1"
```

## RTPS-IDL to Rust Mapping
The IDL types are mapped onto Rust as follows. 
If a type-mapping has not been decided, it is marked with 'NA'.  
As RTPS is a data-centric framework in contrast to 
the the original OO background, the focus is put onto data structures, and ignoring interfaces and structures so far.

|  IDL-Type  | Rust-Type |
| ------------- |:-------------:| 
| module     | module | 
| boolean      | bool      | 
| char/wchar | char      | 
| octet | u8  | 
| string/wstring    | std::string::String  | 
| short | i16  | 
| long |  i32 | 
| long long | i64  | 
| unsigned short | u16  | 
| unsigned long |  u32 | 
| unsigned long long | u64  | 
| float | f32  | 
| double | f64  | 
| fixed  |  _NA_ | 
| enum | enum  | 
| union  | enum  | 
| struct | struct  | 
| sequence | std::vec::Vec  | 
| array, eg. 'T a[N]' | native array '[T;N]'  | 
| interface (non abstract) |  _NA_  | 
| interface (abstract) |  _NA_   | 
| constant (not within interface) | const  | 
| constant (within an interface)   |  _NA_    | 
| exception |  std::result::Result   | 
| Any | _NA_   | 
| type declarations nested within interfaces  | _NA_   | 
| typedef | type  | 
| pseudo objects  | _NA_  | 
| readonly attribute | _NA_  | 
| readwrite attribute |  _NA_   | 
| operation |  _NA_  | 

## Credits
The underlying parser-generator  being used is [PEST][pest-url]

The original IDL-v4 grammar stems from [kpansky][idl-v4-grammar-url], and has been adapted for the needs of this project.

The CDR Serde implementation will be the  [cdr-rs
][cdr-rs-url] project at github.

[licence-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[licence-url]: LICENSE.md
[pest-url]: https://pest.rs/
[idl-v4-grammar-url]: https://github.com/kpansky
[cdr-rs-url]: https://github.com/hrektts/cdr-rs