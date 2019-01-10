extern crate rtps_idl;
extern crate getopts;

use rtps_idl::{IdlLoader, Configuration, generate_with_loader};
use std::io::{Error, ErrorKind};
use std::io::{self, Read};
use std::fs::File;
use getopts::Options;
use std::env;
use std::collections::HashMap;

const IDL_DIR: &str = "./files/";

#[derive(Debug, Clone, Default)]
struct Loader {
    search_path: Vec<String>,
}


fn load_from(prefix: &std::path::Path, filename: &str) -> Result<String, Error> {
    let fullname = prefix.join(filename);

    let mut file = File::open(fullname)?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    return Ok(data);
}

impl Loader {
    pub fn new(search_path: Vec<String>) -> Loader {
        Loader { search_path: search_path }
    }
}

impl IdlLoader for Loader {
    fn load(&self, filename: &str) -> Result<String, Error> {
        for prefix in &self.search_path {
            let prefix_path = std::path::Path::new(&prefix);
            match load_from(&prefix_path, filename) {
                Ok(data) => return Ok(data),
                _ => continue,
            }
        }
        Err(Error::from(ErrorKind::NotFound))
    }
}

//
fn print_usage(program: &str, opts: Options) -> Result<(), std::io::Error> {
    let brief = format!("Usage: {} [-o <outfile>] [-I <include_dir>] <idlfile>", program);
    print!("{}", opts.usage(&brief));
    return Err(Error::from(ErrorKind::NotFound));
}


fn main() -> Result<(), std::io::Error> {
    let mut opts = Options::new();
    opts.optmulti("I", "",
                  "Add the specified 'directory' to the search path for include files.", "directory");
    opts.optmulti("D", "",
                  "Predefine 'name' as a macro, with definition 1.", "name");
    opts.optopt("o", "",
                "Write output to 'outfile'.", "outfile");
     opts.optflag("v", "",
                "Verbose output for debugging'.");
    opts.optflag("h", "help", "print this help menu");
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        return print_usage(&program, opts);
    }

    let search_path = matches.opt_strs("I");

    let defs = matches.opt_strs("D")
        .into_iter()
        .map(|d| {
            let mut iter = d.splitn(2, "=");
            match (iter.next(), iter.next()) {
                (Some(key), None) => (key.to_owned(), "1".to_owned()),
                (Some(key), Some(val)) => (key.to_owned(), val.to_owned()),
                (None, _) => panic!(),
            }
        })
        .collect::<HashMap<String, String>>();

    let mut loader = Loader::new(search_path);

    let infile = match matches.free.len() {
        1 => matches.free[0].clone(),
        _ => return print_usage(&program, opts),
    };

    let data =
        load_from(&env::current_dir().unwrap(), &infile)
            .map_err(|err| Error::new(ErrorKind::NotFound, ""))?;

    let config = Configuration::new(defs, matches.opt_present("v"));

    let result = match matches.opt_str("o") {
        Some(outfile) => {
            let mut of = File::create(std::path::Path::new(&outfile))?;
            generate_with_loader(&mut of, &mut loader, &config, &data)
        }
        _ => generate_with_loader(&mut io::stdout(), &mut loader, &config, &data),
    };

    match result {
        Ok(_) => Ok(()),
        Err(err) => {
            eprint!("parse error {:?}", err);
            Err(Error::new(ErrorKind::InvalidData, "parse error"))
        }
    }
}


#[cfg(test)]
mod tests {
    use rtps_idl::{generate_with_loader, Configuration};
    use super::Loader;
    use std::io::Cursor;
    use std::str;

    const MODULE_PRELUDE: &[u8] = b"#[allow(unused_imports)]
use std::vec::Vec;
";

    fn print_buffer(buf: &Vec<u8>) {
        let content = str::from_utf8(&buf).unwrap();

        println!("{}", content);
    }

    fn generate_and_verify(expect: &[u8], input: &str) {
        let config = Configuration::default();
        let mut loader = Loader::default();

        // Create fake "file"
        let mut out = Cursor::new(Vec::new());
        match generate_with_loader(&mut out, &mut loader, &config, input) {
            Ok(_) => (),
            Err(err) => {
                eprint!("parse error {:?}", err);
                panic!();
            }
        };
        print_buffer(out.get_ref());
        assert_eq!(expect.as_ref(), out.get_ref().as_slice());
    }

    #[test]
    fn typedef_long() {
        const IDL: &str = "typedef long Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = i32;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_long_long() {
        const IDL: &str = "typedef long long Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = i64;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_short() {
        const IDL: &str = "typedef short Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = i16;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_octet() {
        const IDL: &str = "typedef octet Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = u8;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_unsigned_short() {
        const IDL: &str = "typedef unsigned short Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = u16;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_unsigned_long() {
        const IDL: &str = "typedef unsigned long Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = u32;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_unsigned_long_long() {
        const IDL: &str = "typedef unsigned long long Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = u64;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_char() {
        const IDL: &str = "typedef char Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = char;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_wchar() {
        const IDL: &str = "typedef wchar Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = char;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_string() {
        const IDL: &str = "typedef string Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = String;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_wstring() {
        const IDL: &str = "typedef wstring Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = String;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_string_bounded() {
        const IDL: &str = "typedef string<2> Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = String;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_wstring_bounded() {
        const IDL: &str = "typedef wstring<2> Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = String;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_sequence() {
        const IDL: &str = "typedef sequence<octet> Foo;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = Vec<u8>;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_array_dim_1() {
        const IDL: &str = "typedef octet Foo[2];";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = [u8;2];\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn typedef_array_dim_2() {
        const IDL: &str = "typedef octet Foo[2][3+3];";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};

//
//
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub type Foo = [[u8;2];3+3];\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn struct_members() {
        const IDL: &str = "struct Foo { long m_l1,m_l2; double m_d; };";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
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
}\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn struct_module() {
        const IDL: &str = "module A { struct Foo { long m_l1,m_l2; double m_d; }; };";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(non_snake_case)]
mod A {
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
}\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn const_op_and() {
        const IDL: &str = "const long Foo = 2&1;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(dead_code)]
const Foo: i32 = 2&1;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn const_op_add() {
        const IDL: &str = "const long Foo = 1+1;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(dead_code)]
const Foo: i32 = 1+1;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn const_op_sub() {
        const IDL: &str = "const long Foo = 2-1;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(dead_code)]
const Foo: i32 = 2-1;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn const_op_lshift() {
        const IDL: &str = "const long Foo = 1<<1;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(dead_code)]
const Foo: i32 = 1<<1;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn const_op_rshift() {
        const IDL: &str = "const long Foo = 2>>1;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(dead_code)]
const Foo: i32 = 2>>1;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn const_op_or() {
        const IDL: &str = "const long Foo = 2|1;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(dead_code)]
const Foo: i32 = 2|1;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn const_op_xor() {
        const IDL: &str = "const long Foo = 2^1;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(dead_code)]
const Foo: i32 = 2^1;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn const_op_mul() {
        const IDL: &str = "const long Foo = 2*1;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(dead_code)]
const Foo: i32 = 2*1;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn const_op_div() {
        const IDL: &str = "const long Foo = 2/1;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(dead_code)]
const Foo: i32 = 2/1;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

    #[test]
    fn const_op_mod() {
        const IDL: &str = "const long Foo = 2%1;";
        let mut expect = MODULE_PRELUDE.to_owned();
        expect.extend_from_slice(b"#[allow(unused_imports)]
use serde_derive::{Serialize, Deserialize};
#[allow(dead_code)]
const Foo: i32 = 2%1;\n");

        generate_and_verify(expect.as_ref(), IDL);
    }

//    #[test]
//    fn scoped_name() {
//        const IDL: &str = "const long Foo = ::DDS::Bla";
//        let mut expect = MODULE_PRELUDE.to_owned();
//        expect.extend_from_slice(b"#[allow(unused_imports)]
//use serde_derive::{Serialize, Deserialize};
//#[allow(dead_code)]
//const Foo: i32 = crate::DDS::Bla;\n");
//
//        transfer_and_verify(expect.as_ref(), IDL);
//    }
}
