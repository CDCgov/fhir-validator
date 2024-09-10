use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use serde_json::Value;
// TODO: used to check a dep path is available, use std::path::Path;

type Dependencies = HashSet<String>;


pub fn obtain_package_deps(package_folder: &str, package_coordinate: &str) -> Result<Dependencies, Box<dyn Error>> {

    let root_deps = get_package_deps(package_folder, package_coordinate)?;
    let full_deps_tree = dependency_resolver(package_folder, HashSet::new(), root_deps)?;

    Ok(full_deps_tree)
}// .obtain_package_deps 


fn get_package_deps(package_folder: &str, package_coordinate: &str) -> Result<Dependencies, Box<dyn Error>> {
    
    let package_meta = get_package_meta(package_folder, package_coordinate)?;

    let package_deps = normalize_package_deps(package_meta["dependencies"].as_array());

    Ok(package_deps)
}// .get_package_deps


fn get_package_meta(packages_folder: &str, package_coordinate: &str) -> Result<Value, Box<dyn Error>> {
    
    let file_path = format!("./{}/{}_package.ndjson", packages_folder, package_coordinate); 
    
    // TODO: skip if not exist or block?
    //let path_exists = Path::new(&file_path).exists();
    
    // TODO: when a needed package is not available in the ig-packages folder 
    //println!("path_exists: {path_exists} for file_path: {file_path}");

    get_specific_line_from_ndjson(&file_path, 1) 
}// .get_package_meta


fn get_specific_line_from_ndjson(file_path: &str, target_line: usize) -> Result<Value, Box<dyn Error>> {
    
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut current_line = 0;

    while let Some(line) = lines.next() {
        let line = line?;
        current_line +=1;
        if current_line == target_line {
            let parsed_line: Value = serde_json::from_str(&line)?;
            return Ok(parsed_line);
        }// .if
    }// .while

    Err("Target line not found".into()) 
}// .get_specific_line_from_ndjson


fn normalize_package_deps(package_deps: Option<&Vec<Value>>) -> Dependencies {
    
    if let Some(deps) = package_deps {
        deps.iter()
            .filter_map(|d| d.as_str().map(|s| s[1..].to_string())) // removes leading (colon) character 
            .collect()
    
    } else {
        HashSet::new()
    }// .else 

}// .normalize_package_deps
 

fn dependency_resolver(package_folder: &str, mut visited_deps: Dependencies, mut enqueued_deps: Dependencies) 
    -> Result<Dependencies, Box<dyn Error>> {
        //println!("visited_deps: {:?}", visited_deps);
        //println!("enqueued_deps: {:?}", enqueued_deps);

        while !enqueued_deps.is_empty() {
            let mut new_deps = HashSet::new();
            
            for dep in enqueued_deps.drain() {
                let child_deps = get_package_deps(package_folder, &dep)?;

                visited_deps.insert(dep);

                //println!("child_deps: {:?}", child_deps);

                for child_dep in child_deps {
                    if !visited_deps.contains(&child_dep) {
                        new_deps.insert(child_dep);
                    }
                }// .for
            }// .for

            //println!("visited_deps: {:?}", visited_deps);

            visited_deps.extend(new_deps.clone());

            //println!("visited_deps(after clone): {:?}", visited_deps);

            enqueued_deps = new_deps;
        }// .while

        Ok(visited_deps)
}// .dependency_resolver


