//astro_comp.rs

//contains functions dealing witht the computation of the star position calculator program
use std::error::Error;
use std::io;

use csv;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct StarData {

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


impl StarData {
    pub fn get_xyz(&self) -> (f64, f64, f64) {

        let r = 1.0 / self.parallax * 1000.;
        let th = self.ra.to_radians().sin_cos(); // returns (sin(t), cos(t))
        let p = (std::f64::consts::FRAC_PI_2 - self.dec.to_radians()).sin_cos();
        //handy functions

        let x = r * th.1 * p.0; // rsin(p)cos(t)
        let y = r * th.0 * p.0; // rsin(p)sin(t)
        let z = r * p.1;       // rsin(p)

        (x,y,z)
    }
}

pub fn deg_to_rad(deg:f64) -> f64 {
    deg * 0.0174533
}

pub fn read_from_stdin() -> Result<Vec<StarData>, Box<dyn Error>>{

    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut data: Vec<StarData> = vec![];

    for result in reader.deserialize(){
        let record: StarData = result?;
            data.push(record);
        //println!("{:?}", record);
    }

    Ok(data)
}


pub fn write_xyz_to_stdout(xyz: Vec<(f64,f64,f64)>) -> Result<(), Box<dyn Error>>{

    let mut writer = csv::Writer::from_writer(io::stdout());

    //first line
    writer.write_record(&["x", "y", "z"])?;

    for i in xyz.iter() {
        let x = i.0.to_string();
        let y = i.1.to_string();
        let z = i.2.to_string();

        if x != "NaN"{
            writer.write_record(&[x,y,z])?;
        }    
    }

    writer.flush()?;

    Ok(())
}