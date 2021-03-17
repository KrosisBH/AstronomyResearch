use std::error::Error;
use std::io;

use csv;
use serde::Deserialize;



#[derive(Debug, Deserialize)]
struct StarData {

    designation: String,

    ra: f64,
    ra_error: f64,
    dec: f64,	
    dec_error: f64,
    parallax: f64,
    parallax_error: f64,	
    parallax_over_error: f64,	
    
    bp_rp: f64,
    bp_g: f64,
    g_rp: f64,
    
    l: f64,
    b: f64,
    
    distance: f64

}

fn read_from_stdin() -> Result<Vec<StarData>, Box<dyn Error>>{

    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut data: Vec<StarData> = vec![];

    for result in reader.deserialize(){
        let record: StarData = result?;
        data.push(record);
        //println!("{:?}", record);
    }

    Ok(data)
}

fn main() {



    let star_data = read_from_stdin();

    //error checks, spits an error if the file was invalid. This is why I love rust!
    let star_data = match star_data {
        Ok(data) => data,
        Err(e) => panic!("Error reading .csv file! {:?}", e),
    };

    println!("{:?}", star_data[0]);

}