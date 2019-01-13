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


## Mapping by examples

## Templates

| IDL | Rust |
| ----- | ----- |
| `sequence<octet>` | `std::vec::Vec<u8>` |

### Typedef

| IDL | Rust |
| ----- | ----- |
| typedef long Foo; | pub type Foo = i32; |
| typedef short Foo[2]; | pub type Foo = [i16;2] |
| typedef short Foo[2][3]; | pub type Foo = [[i16; 2]; 3] |
| typedef sequence<octet> Foo; | pub type Foo = std::vec::Vec<u8> |

### Struct

| IDL | Rust |
| ----- | ----- |
| struct Foo {<br>&ensp;long l;<br>&ensp;short s;<br>}; | pub struct Foo {<br>&ensp;pub l: i32,<br>&ensp;pub s: i16;<br>} |

### Enum

| IDL | Rust |
| ----- | ----- |
| enum Foo { VARIANT0, VARIANT1, VARIANT2 }; | pub enum Foo { VARIANT0, VARIANT1, VARIANT2, } |

### Union Switch

Note: Only switch types "switch (long)" is supported yet.

| IDL | Rust |
| ----- | ----- |
| union Foo switch (long) {<br>&ensp;case LABEL0: long l;<br>&ensp;case LABEL1:<br>&ensp;case LABEL2: short s;<br>&ensp;default: octet o[8];<br>}; | pub enum Foo {<br>&ensp;LABEL0{l: i32},<br>&ensp;LABEL2{s: i16},<br>&ensp;LABEL1{s: i16},<br>&ensp;default{o: [u8; 8]},<br>}  |
| /* not yet, to be developed */<br>union Result switch (long) {<br>&ensp;case None: void _dummy;<br>&ensp;case Some: T t<br>}; | /* not yet, to be developed */<br>pub enum Result\<T> {<br>&ensp;None,<br>&ensp;Some(T),<br>}  |


[licence-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[licence-url]: LICENSE.md
