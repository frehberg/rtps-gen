// Copyright (C) 2019  Frank Rehberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>

extern crate rtps_idl;

use std::env;
use std::path::Path;
use std::io::{Error, ErrorKind};
use rtps_idl::{Configuration, generate_with_search_path};
use std::io::Read;
use std::fs::File;

// TODO: improve the generator and run over ../../files/dds/DdsDcpsDomain.idl
const IDL_DIR: &str = "../../files";
const IDL_INFILE: &str = "../../files/dds/DdsDcpsGuid.idl";
const RUST_OUTFILE: &str = "DdsElements.rs";

fn main() -> Result<(), Error> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(RUST_OUTFILE);
    let search_path = vec![IDL_DIR.to_owned()];
    let config = Configuration::default();
    let mut out = File::create(dest_path)?;
    let mut data = String::new();

    let _ = File::open(Path::new(IDL_INFILE))
        .and_then(|mut inf| inf.read_to_string(&mut data))?;

    generate_with_search_path(&mut out, search_path, &config, &data)
        .map_err(|_| Error::from(ErrorKind::NotFound))
}