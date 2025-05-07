use cargo_toml::Manifest;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::{BTreeMap, BTreeSet};
use cargo_toml::Dependency;


fn main() {
    let argument : Vec<String> = std::env::args().collect();
    let n_arg = argument.len();
    println!("n_arg={}", n_arg);
    if n_arg == 1 {
        println!("Program is used as search_unused_dependencies [Directory]");
        println!("");
        println!("INPUT:");
        println!("Directory: the directory of the package");
        println!("");
        println!("OUTPUT:");
        println!("--Some candidates for unused packages");
        std::process::exit(1)
    }
    let directory = argument[1].clone();
    println!("directory={directory}");
    let cargo_toml = format!("{directory}/Cargo.toml");
    println!("cargo_toml={cargo_toml}");


    let manifest = Manifest::from_path(cargo_toml).expect("obtain a manifest");

    println!("manifest={manifest:?}");

}

