extern crate time;
use csv;
use std::error::Error;
use time::PreciseTime;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let _headers = reader.headers();

    // First answer
    let mut rowcounter = 0;
    let mut counter = 1;
    let mut previous = 0;
    for result in reader.deserialize() {
        let record = result?;

        if rowcounter > 0 {
            if record > previous {
                //println!["{} > {}, {}", record, previous, counter];
                counter = counter + 1;
            } else {
                //println!["{} < {}, {}", record, previous, counter];
            }
        } else {
            //println!["{}", rowcounter];
        }
        rowcounter = rowcounter + 1;
        previous = record;
    }
    println!("The first answer is {}", counter);

    // second answer
    reader = csv::Reader::from_path(path)?;
    rowcounter = 0;
    let mut sum;
    let mut previous_sum = 0;
    let mut previous_1 = 0;
    let mut previous_2 = 0;
    let mut sum_counter = 0;

    for result in reader.deserialize() {
        let record = result?;

        // Init
        if rowcounter == 0 {
            previous_2 = record;
        }
        if rowcounter == 1 {
            previous_1 = record;
        }

        if rowcounter >= 3 {
            //println!("{} {}", previous_1, previous_2);

            sum = previous_2 + previous_1 + record;
            if sum > previous_sum {
                sum_counter = sum_counter + 1;
            }
            previous_sum = sum;
            previous_2 = previous_1;
            previous_1 = record;
        }

        rowcounter = rowcounter + 1;
    }
    println!("The second answer is: {}", sum_counter - 1);
    Ok(())
}

fn main() {
    let start = PreciseTime::now();
    if let Err(err) = read_from_file("./src/1_input.txt") {
        println!("error running {}", err);
    }
    let end = PreciseTime::now();
    println!("{} seconds for whatever you did.", start.to(end));
}
