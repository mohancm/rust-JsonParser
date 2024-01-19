use reqwest::blocking::get;
use serde_json::Value;
use rand::Rng;

fn main()  {

    // call fetch_json
    let _ = fetch_json();
    
}

fn fetch_json() -> Result<(), Box<dyn std::error::Error>> {

    // add json url
    let json_url = "https://gist.githubusercontent.com/mohancm/60f0f18b88cf4e7cb6644cc1b3329131/raw/f8c7ddc5619fc78fb1ffbe339ff4cd2c59e9e985/inspirations.json";

    // fetch json data to a variable
    let response= get(json_url)?.text()?;

    //  parse json data into Value object
    let json_value: Value = serde_json::from_str(&response)?;

    // print out the number of json objects after parsing
    let num_objects = json_value.as_array().unwrap().len();
    println!("Number of Objects in the Array: {}", num_objects);

    // generate random number between number of json objects
    let rand_num = get_random_number(1, num_objects as i32) as usize;

    // print out the message of the first object
    println!(
        "Value of the first object 'message' property: {}", json_value[rand_num]["message"]
    );

    Ok(())

}

// returns a random integer btw 'min' and 'max' (inclusive)
pub fn get_random_number(min: i32, max: i32) -> i32 {
    assert!(min <= max, "Minimum must be less than or equal to maximum.");

    // create a new thread-local random number generator (RNG) instance
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}
