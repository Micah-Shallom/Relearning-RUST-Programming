

pub fn ifletfn() {
    let config_max= Some(3u8);
    match config_max {
        Some(max) => println!("The meximum is configured to be {}", max),
        _ => (),
    }

    //using if let syntax
    //this is nice but it limit the exhaustive searching capabilties provided by the match keyword
    if let Some(max) = config_max {
        println!("The meximum is configured to be {}", max)
    }
}