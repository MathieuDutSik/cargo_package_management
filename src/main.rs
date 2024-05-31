use cargo_toml::Manifest;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::{BTreeMap, BTreeSet};
use cargo_toml::Dependency;

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
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
    Ok(lines)
}



fn get_depends(manifest: Manifest, file_opt: &String) -> Vec<String> {
    let mut dependencies = Vec::new();
    if file_opt == "partial" {
        for (depend, _) in manifest.dependencies {
            dependencies.push(depend);
        }
        return dependencies;
    }
    if file_opt == "full" {
        let mut dependencies_set = BTreeSet::new();
        for (depend, _) in manifest.dependencies {
            dependencies_set.insert(depend);
        }
        for (depend, _) in manifest.dev_dependencies {
            dependencies_set.insert(depend);
        }
        for depend in dependencies_set {
            dependencies.push(depend);
        }
        return dependencies;
    }
    panic!("get_depends failed because file_opt is not adequate");
}

fn get_appendable_vertex(digraph: &Vec<Vec<usize>>, l_status: &Vec<usize>) -> Option<usize> {
    let n_vert = digraph.len();
    for u in 0..n_vert {
        let mut n_uncover = 0;
        for ent in &digraph[u] {
            if l_status[*ent] == 0 {
                n_uncover += 1;
            }
        }
        if n_uncover == 0 {
            return Some(u);
        }
    }
    None
}


fn get_ordering(digraph: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let n_vert = digraph.len();
    let mut l_status = Vec::new();
    for _ in 0..n_vert {
        l_status.push(0);
    }
    let mut l_depend = Vec::new();
    for _ in 0..n_vert {
        let opt = get_appendable_vertex(digraph, &l_status);
        if let Some(vert) = opt {
            l_depend.push(vert);
            l_status[vert] = 1;
        } else {
            return None;
        }
    }
    Some(l_depend)
}





fn main() {
    let argument : Vec<String> = std::env::args().collect();
    let n_arg = argument.len();
    println!("n_arg={}", n_arg);
    if n_arg == 1 {
        println!("Program is used as found_dependencies [FilePackage] [main_cargo] [file_opt]");
        std::process::exit(1)
    }
    let package_list_file = argument[1].clone();
    println!("package_list_file={}", package_list_file);
    let main_cargo_toml = argument[2].clone();
    println!("main_cargo_toml={}", main_cargo_toml);
    let file_opt = argument[3].clone();
    println!("file_opt={}", file_opt);


    let manifest = Manifest::from_path(main_cargo_toml).expect("obtain a manifest");

    let Some(workspace) = manifest.workspace else {
        panic!("We do not have a workspace");
    };

    let lines = read_lines(package_list_file).expect("to obtain lines");
    let mut packages_vec = Vec::new();
    let mut packages_set = BTreeSet::new();
    let mut packages_map = BTreeMap::<String,usize>::new();
    let mut pos = 0;
    for line in lines {
        if !line.starts_with("#") {
            packages_vec.push(line.clone());
            packages_set.insert(line.clone());
            packages_map.insert(line, pos);
            pos += 1;
        }
    }
    let n_packages = packages_set.len();
    for package in packages_vec {
        println!("package={}", package);
    }

    for member in workspace.members {
        println!("member={}", member);
    }

    let mut digraph_dependency = Vec::new();
    for _ in 0..n_packages {
        digraph_dependency.push(Vec::new());
    }

    for (dependency, description) in workspace.dependencies {
//        println!("dependency={} description={:?}", dependency, description);
        if packages_set.contains(&dependency) {
            println!("dependency={}", dependency);
            let Dependency::Detailed(detail) = description else {
                panic!("failed to find the detail");
            };
            let pos_package = packages_map.get(&dependency).unwrap().clone();
            println!("  pos_package={}", pos_package);
            let Some(path) = detail.path else {
                panic!("path was None which is not allowed");
            };
            let cargo_file_depend = path + "/Cargo.toml";
            let manifest_depend = Manifest::from_path(cargo_file_depend).expect("obtain a manifest");
            let dependencies = get_depends(manifest_depend, &file_opt);
            let mut entries = Vec::new();
            print!("  {} :", dependency);
            for loc_dependency in dependencies {
                if let Some(pos_depend) = packages_map.get(&loc_dependency) {
                    entries.push(pos_depend.clone());
                    print!(" {}", loc_dependency);
                }
            }
            println!("");
            digraph_dependency[pos_package] = entries;
        }
    }
    for i_vert in 0..n_packages {
        let n_dep = digraph_dependency[i_vert].len();
        println!("i_vert={} n_dep={}", i_vert, n_dep);
    }

    
    let ordering = get_ordering(&digraph_dependency);
    match ordering {
        None => {
            println!("No ordering found");
        }
        Some(_) => {
            println!("One ordering found");
        }
    };
}

