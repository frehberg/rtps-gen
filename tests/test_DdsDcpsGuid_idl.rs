extern crate rtps_idl;

use std::env;
use std::path::Path;
use std::io::{Error, ErrorKind};
use rtps_idl::{Configuration, generate_with_search_path};
use std::io::{Read, stdout};
use std::fs::File;

const IDL_DIR: &str = "files/";
const IDL_INFILE: &str = "files/dds/DdsDcpsGuid.idl";

#[test]
fn convert_idl() -> Result<(), Error> {
    let search_path = vec![IDL_DIR.to_owned()];
    let config = Configuration::default();
    let mut data = String::new();

    let _ = File::open(Path::new(IDL_INFILE))
        .and_then(|mut inf| inf.read_to_string(&mut data))
        .map_err(|err| {
            println!("{:?}", err);
            err
        });

    generate_with_search_path(&mut stdout(), search_path, &config, &data)
        .map_err(|_| Error::from(ErrorKind::NotFound))
}
