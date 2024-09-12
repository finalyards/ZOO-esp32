/*
* Example of parsing 'pins.toml'
*/
use serde::Deserialize;
use toml;

use std::{
    collections::HashMap,
    error::Error
};

// Data objects
//
#[derive(Debug, Deserialize)]
struct PinsToml {
    generate: String,
    boards: HashMap<String,Board>
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Board {
    SDA: u32,
    SCL: u32,
    PWR_EN: u32,
    LPn: Vec<u32>
}

fn main() -> Result<(), Box<dyn Error>> {

    let s = include_str!("../../vl53l5cx_uld/pins.toml");
    /***let s = "
        [[job]]
        title = \"Lorem\"
        company = \"Ipsum\"
        description = \"abcd\"
        [[job]]
        title = \"Lorem\"
        company = \"Ipsum\"
        description = \"abcd\"
        ";***/
    //let v = s.parse::<PinsToml>().unwrap();
    //println!("{v:?}");

    let c: PinsToml = toml::from_str(s)?;

    println!("{:?}", c);

    Ok(())
}
