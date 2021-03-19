mod astro_comp;

fn main() {
    let star_data = astro_comp::read_from_stdin();

    //error checks, spits an error if the file was invalid. This is why I love rust!
    let star_data = match star_data {
        Ok(data) => data,
        Err(e) => panic!("Error reading .csv file! {:?}", e),
    };

    
    let mut xyz: Vec<(f64,f64,f64)> = vec![];
    for star in star_data.iter() {
        xyz.push(star.get_xyz());
    }

    if let Err(e) = astro_comp::write_xyz_to_stdout(xyz){
        panic!("Error writing to stdout! {:?}", e);
    }
    

}