use reqwest::blocking::get;
use serde_json::Value;

fn main()  {

    // call fetch_json
    fetch_json();
    
}

fn fetch_json() -> Result<(), Box<dyn std::error::Error>> {

    // add json url
    let json_url = "https://gist.githubusercontent.com/mohancm/60f0f18b88cf4e7cb6644cc1b3329131/raw/f8c7ddc5619fc78fb1ffbe339ff4cd2c59e9e985/inspirations.json";

    // fetch json data to a variable
    let response= get(json_url)?.text()?;

    //  parse json data into Value object
    let json_value: Value = serde_json::from_str(&response)?;

    // print out the number of json objects after parsing
    println!("Number of Objects in the Array: {}", json_value.as_array().unwrap().len());

    // print out the message of the first object
    println!(
        "Value of the first object 'message' property: {}", json_value[0]["message"]
    );

    Ok(())

}


