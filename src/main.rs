mod deps_resolver;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Started FHIR Validator based on FHIR Schema...");
    
    let package_folder = "ig-packages";
    // let package_coordinate = "hl7.fhir.us.core#3.1.1";
    let package_coordinate = "hl7.fhir.us.health-care-surveys-reporting#1.0.0";

    println!("Set up validator for profile: {}", package_coordinate);

    let deps= deps_resolver::obtain_package_deps(package_folder, package_coordinate)?;
    println!("Result dependencies: {:?}", deps);
 
    Ok(())
    
} // .main

