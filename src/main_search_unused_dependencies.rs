use cargo_toml::Manifest;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::Command;


fn read_lines<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename)?;

    // Create a new BufReader, which will allow us to read the file line by line.
    let reader = BufReader::new(file);

    // Collect the lines of the file into a Vec<String>.
    let lines: Vec<String> = reader.lines()
        .collect::<Result<_, _>>()?;
    let mut entry = String::new();
    for line in lines {
        entry += &line;
    }
    Ok(entry)
}


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

//    println!("manifest={manifest:?}");
//    println!();

    let mut dependencies = Vec::new();
    for (key, _value) in manifest.dependencies {
        let dependency: String = key.replace("-", "_").to_string();
        dependencies.push(dependency);
    }
//    println!("dependencies={dependencies:?}");
//    println!();

    let output = Command::new("find")
        .arg(".")
        .current_dir(&directory)
        .output()
        .expect("Failed to execute command");

//    println!("output={output:?}");
    let stdout = String::from_utf8(output.stdout).unwrap();
//    println!("stdout={:?}", stdout);

    let mut files = Vec::new();
    for entry in stdout.split("\n") {
        if entry.ends_with(".rs") {
            files.push(entry);
        }
    }
//    println!("files={files:?}");

    let mut full_string = String::new();
    for file in files {
        let full_file = format!("{directory}/{file}");
        let the_read = read_lines(full_file).expect("Reading full_file={full_file}");
        full_string += &the_read;
    }

    let mut n_found = 0;
    for dependency in dependencies {
        let l_vec = full_string.split(&dependency).collect::<Vec<_>>();
        if l_vec.len() == 1 {
            println!("dependency={dependency} may be not needed");
            n_found += 1;
        }
    }
    println!("n_found={n_found}");
}

