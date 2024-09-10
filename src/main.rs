mod deps_resolver {

    use std::collections::HashSet;
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use serde_json::Value;

    type Dependencies = HashSet<String>;


    pub fn obtain_package_deps(package_folder: &str, package_coordinate: &str) -> Result<Dependencies, Box<dyn Error>> {

        let root_deps = get_package_deps(package_folder, package_coordinate)?;

        Ok(root_deps)
    }// .obtain_package_deps 
    

    fn get_package_deps(package_folder: &str, package_coordinate: &str) -> Result<Dependencies, Box<dyn Error>> {
        
        let package_meta = get_package_meta(package_folder, package_coordinate)?;

        let package_deps = normalize_package_deps(package_meta["dependencies"].as_array());

        Ok(package_deps)
    }// .get_package_deps


    fn get_package_meta(packages_folder: &str, package_coordinate: &str) -> Result<Value, Box<dyn Error>> {
        
        let file_path = format!("./{}/{}_package.ndjson", packages_folder, package_coordinate); 

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


}// .mod deps_resolver


use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Started FHIR Validator based on FHIR Schema...");
    
    let package_folder = "ig-packages";
    let package_coordinate = "hl7.fhir.us.core#3.1.1";
    //let package_coordinate = "hl7.fhir.us.health-care-surveys-reporting#1.0.0";

    println!("Set up validator for profile: {}", package_coordinate);

    let deps = deps_resolver::obtain_package_deps(package_folder, package_coordinate)?;
    println!("Result dependencies: {:?}", deps);
 
    Ok(())
    
} // .main

