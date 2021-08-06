use std::io;
use std::fmt::Display;
use std::fmt::Debug;
use std::any::type_name;
use std::f64::consts::PI;

fn take_number<T>() -> T where // Generic function syntax convention (for me)
T: Debug + Display + std::str::FromStr,
<T as std::str::FromStr>::Err: Debug {

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    let trimmed = input.trim();
    //match trimmed.parse::<T>() {
    //    Ok(i) => println!("your {} input: {}", type_name::<T>(), i),
    //    Err(..) => println!("this wasn't {}: {}", type_name::<T>(), trimmed),
    //};
    trimmed.parse::<T>().unwrap()
}

fn take_string<T>() -> Vec<T> where
T: Debug + Display + std::str::FromStr,
<T as std::str::FromStr>::Err: Debug {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    // Takes a string, splits and parses each element as type T into a Vec<T> object
    let trimmed: Vec<T> = input
        .trim().split(" ")
        .map(|s| s.parse::<T>().expect("parse error"))
        .collect();
    trimmed
}

fn main() {
    //println!("amount of test cases:");
    let test_case_amount = take_number::<u32>();
    if test_case_amount < 1 || test_case_amount > 25 {
        panic!();
    }

    for i in 0..test_case_amount { 
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut angle: f64 = 90.0;
        //println!();
        //println!("Segment {}", i);
        //println!("amount of segments:");
        let segment_amount = take_number::<u32>();
        if segment_amount < 1 || segment_amount > 10 {
            panic!();
        }

        for _ in 0..segment_amount {
            //println!();
            let segments = take_string::<f64>();
            if segments[0] < -360.0 || segments[0] > 360.0 ||
                segments[1] < -100.0 || segments[1] > 100.0 {
                panic!();
            }
            angle += segments[0];
            //println!("segments: {:?}", segments);
            let coordinates = (
                (segments[1] * (angle * (PI/180.0)).cos()),
                (segments[1] * (angle * (PI/180.0)).sin())
            );
            //println!("coordinates: {:?}", coordinates);

            x += coordinates.0;
            y += coordinates.1;
        }
        println!("{:.6} {:.6}", x, y);

    }

}
