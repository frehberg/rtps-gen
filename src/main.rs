extern crate rtps_idl;
extern crate getopts;

use rtps_idl::{IdlLoader, Configuration, generate_with_loader};
use std::io::{Error, ErrorKind};
use std::io::{self, Read};
use std::fs::File;
use getopts::Options;
use std::env;
use std::collections::HashMap;

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
            .map_err(|_| Error::new(ErrorKind::NotFound, ""))?;

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
    use rtps_idl::{generate_with_search_path, Configuration};
    use super::Loader;
    use std::io::Cursor;
    use std::str;
    use std::fs::File;
    use std::path::Path;
    use std::io::{Error, ErrorKind};
    use std::io::{Write, Read};

    #[test]
    fn typedef_long() {
        testvector_verify("files/test-vectors/typedef_long/");
    }

    #[test]
    fn typedef_long_long() {
        testvector_verify("files/test-vectors/typedef_long_long/");
    }

    #[test]
    fn typedef_short() {
        testvector_verify("files/test-vectors/typedef_short/");
    }

    #[test]
    fn typedef_octet() {
        testvector_verify("files/test-vectors/typedef_octet/");
    }

    #[test]
    fn typedef_unsigned_short() {
        testvector_verify("files/test-vectors/typedef_unsigned_short/");
    }

    #[test]
    fn typedef_unsigned_long() {
        testvector_verify("files/test-vectors/typedef_unsigned_long");
    }

    #[test]
    fn typedef_unsigned_long_long() {
        testvector_verify("files/test-vectors/typedef_unsigned_long_long");
    }

    #[test]
    fn typedef_char() {
        testvector_verify("files/test-vectors/typedef_char");
    }

    #[test]
    fn typedef_wchar() {
        testvector_verify("files/test-vectors/typedef_wchar");
    }

    #[test]
    fn typedef_string() {
        testvector_verify("files/test-vectors/typedef_string");
    }

    #[test]
    fn typedef_wstring() {
        testvector_verify("files/test-vectors/typedef_wstring");
    }

    #[test]
    fn typedef_string_bounded() {
        testvector_verify("files/test-vectors/typedef_string_bounded");
    }

    #[test]
    fn typedef_wstring_bounded() {
        testvector_verify("files/test-vectors/typedef_wstring_bounded");
    }

    #[test]
    fn typedef_sequence() {
        testvector_verify("files/test-vectors/typedef_sequence");
    }

    #[test]
    fn typedef_array_dim_1() {
        testvector_verify("files/test-vectors/typedef_array_dim_1");
    }

    #[test]
    fn typedef_array_dim_2() {
        testvector_verify("files/test-vectors/typedef_array_dim_2");
    }

    #[test]
    fn struct_members() {
        testvector_verify("files/test-vectors/struct_members");
    }

    #[test]
    fn struct_module() {
        testvector_verify("files/test-vectors/struct_module");
    }

    #[test]
    fn const_op_and() {
        testvector_verify("files/test-vectors/const_op_and");
    }

    #[test]
    fn const_op_add() {
        testvector_verify("files/test-vectors/const_op_add");
    }

    #[test]
    fn const_op_sub() {
        testvector_verify("files/test-vectors/const_op_sub");
    }

    #[test]
    fn const_op_lshift() {
        testvector_verify("files/test-vectors/const_op_lshift");
    }

    #[test]
    fn const_op_rshift() {
        testvector_verify("files/test-vectors/const_op_rshift");
    }

    #[test]
    fn const_op_or() {
        testvector_verify("files/test-vectors/const_op_or");
    }

    #[test]
    fn const_op_xor() {
        testvector_verify("files/test-vectors/const_op_xor");
    }

    #[test]
    fn const_op_mul() {
        testvector_verify("files/test-vectors/const_op_mul");
    }

    #[test]
    fn const_op_div() {
        testvector_verify("files/test-vectors/const_op_div");
    }

    #[test]
    fn const_op_mod() {
        testvector_verify("files/test-vectors/const_op_mod");
    }

    #[test]
    fn include_directive() {
        testvector_verify("files/test-vectors/include_directive/");
    }

    #[test]
    fn union_members() {
        testvector_verify("files/test-vectors/union_members");
    }

    fn testvector_verify(testvector: &str) {
        let input_path = Path::new(testvector).join("input.idl");
        let expected_path = Path::new(testvector).join("expected.rs");

        let mut input_file = match File::open(input_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{}", err);
                panic!();
            }
        };
        let mut input = String::new();
        assert!(input_file.read_to_string(&mut input).is_ok());

        let mut expected_file = match File::open(expected_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{}", err);
                panic!();
            }
        };
        let mut expected = String::new();
        assert!(expected_file.read_to_string(&mut expected).is_ok());

        let config = Configuration::default();
        let search_path = vec![testvector.to_owned()];

        // Create fake "file"
        let mut out = Cursor::new(Vec::new());
        match generate_with_search_path(&mut out, search_path, &config, &input) {
            Ok(_) => (),
            Err(err) => {
                eprint!("parse error {:?}", err);
                panic!();
            }
        };
        print_buffer(out.get_ref());
        let expected_bytes: &[u8] = expected.as_ref();
        assert_eq!(expected_bytes, out.get_ref().as_slice());
    }

    fn print_buffer(buf: &Vec<u8>) {
        let content = str::from_utf8(&buf).unwrap();

        println!("{}", content);
    }
}
