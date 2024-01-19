use reqwest::blocking::get;
use serde_json::Value;

fn main() -> Result<(), Box <dyn std::error::Error>> {
    // add json url
    let json_url = "https://gist.githubusercontent.com/mohancm/60f0f18b88cf4e7cb6644cc1b3329131/raw/f8c7ddc5619fc78fb1ffbe339ff4cd2c59e9e985/inspirations.json";
    
    // fetch json data to a variable
    let response = get(json_url)?.text()?;

    // Parse the json data into Value object
    let json_value: Value = serde_json::from_str(&response)?;

    // print out the json parsed data
    println!("Number of object of the array: {}", json_value.as_array().unwrap().len());
    println!(
        "Value of the first objects 'message' property: {}", json_value[0]["message"]
    );

    Ok(())

}


