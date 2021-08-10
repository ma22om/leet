use std::io;
use std::fmt::Display;
use std::fmt::Debug;
use std::f64::consts::PI;

fn take_number<T>() -> T where // Generic function syntax convention (for me)
T: Debug + Display + std::str::FromStr,
<T as std::str::FromStr>::Err: Debug {

    // Takes any type of number from a String
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    // Parses the String to become the number type T
    let trimmed = input.trim(); 
    trimmed.parse::<T>().unwrap()
}

fn take_segment() -> Segment {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    // Takes a string, splits and parses each element as type f64 into a Vec<f64> object
    let trimmed: Vec<f64> = input
        .trim().split(" ")
        .map(|s| s.parse::<f64>().expect("parse error"))
        .collect();
    
    Segment {
        angle: trimmed[0],
        distance: trimmed[1],
    }
}

struct Segment {
    angle: f64,
    distance: f64,
}

struct Coordinates {
    x: f64,
    y: f64,
}

fn main() {
    let test_case_amount = take_number::<u32>();

    if test_case_amount < 1 || test_case_amount > 25 {
        panic!();
    }

    for i in 0..test_case_amount { 
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut curr_angle: f64 = 90.0;
        let segment_amount = take_number::<u32>();

        if segment_amount < 1 || segment_amount > 10 {
            panic!();
        }

        for _ in 0..segment_amount {
            let segment = take_segment();

            if segment.angle < -360.0 || segment.angle > 360.0 ||
                segment.distance < -100.0 || segment.distance > 100.0 {
                panic!();
            }

            curr_angle += segment.angle;

            let coordinates = Coordinates {
                x: (segment.distance * (curr_angle * (PI/180.0)).cos()),
                y: (segment.distance * (curr_angle * (PI/180.0)).sin()),
            };

            x += coordinates.x;
            y += coordinates.y;
        }
        println!("{:.6} {:.6}", x, y);
    }
}
