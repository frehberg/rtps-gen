[![Apache 2.0 licensed][licence-badge]][licence-url]
# RTPS IDL to Rust code generator

A tool reading an IDL and generating corresponding Rust code.

Usage:
```shell
rtps-gen -I <include-dir> data.idl -o output.rs
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


[licence-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[licence-url]: LICENSE.md
