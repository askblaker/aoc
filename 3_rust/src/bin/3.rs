use std::env;
use std::fs;

fn read_file(filename: &String) -> Vec<String> {
    println!("Reading file: {}", filename);
    let contents =
        fs::read_to_string(filename).expect("Something went wrong while reading the file");
    let strings = contents
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    strings
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let records = read_file(filename);
    get_epsilon_and_gamma_rate(&records, 12);
    get_ox_and_co2_rating(&records, 12);
}

fn get_epsilon_and_gamma_rate(records: &Vec<String>, len: usize) -> isize {
    let mut epsilon_rate: String = "".to_string();
    let mut gamma_rate: String = "".to_string();
    for n in 0..len {
        let mut count_0 = 0;
        let mut count_1 = 0;
        for record in records {
            let num = record.chars().nth(n).unwrap();
            if num == '0' {
                count_0 += 1;
            } else if num == '1' {
                count_1 += 1;
            }
        }
        if count_0 > count_1 {
            epsilon_rate.push('0');
            gamma_rate.push('1');
        } else {
            epsilon_rate.push('1');
            gamma_rate.push('0');
        }
    }
    let epsilon_rate_decimal = isize::from_str_radix(&epsilon_rate, 2).unwrap();
    let gamma_rate_decimal = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let result = epsilon_rate_decimal * gamma_rate_decimal;
    println!("Power consumption: {}", result);
    result
}

fn get_ox_and_co2_rating(records: &Vec<String>, bits: usize) -> isize {
    let mut ox: isize = 0;
    let mut co2: isize = 0;
    let options = vec!["oxygen", "co2"];
    for variable in options {
        let mut cloned = records.clone();
        for i in 0..bits {
            let mut count_0 = 0;
            let mut count_1 = 0;
            let mut selector = "0";
            for record in &cloned {
                let num = record.chars().nth(i).unwrap();
                if num == '0' {
                    count_0 += 1;
                } else if num == '1' {
                    count_1 += 1;
                }
            }
            if variable == "oxygen" {
                if count_0 > count_1 {
                    selector = "0";
                } else if count_0 < count_1 {
                    selector = "1";
                } else if count_0 == count_1 {
                    selector = "1";
                }
            } else if variable == "co2" {
                if count_0 > count_1 {
                    selector = "1";
                } else if count_0 < count_1 {
                    selector = "0";
                } else if count_0 == count_1 {
                    selector = "0";
                }
            }
            let check = cloned
                .iter()
                .filter(|x| x.chars().nth(i).unwrap() == selector.chars().nth(0).unwrap())
                .collect::<Vec<&String>>();
            if check.len() == 1 {
                let value = isize::from_str_radix(&check[0], 2).unwrap();
                if variable == "oxygen" {
                    ox = value;
                } else if variable == "co2" {
                    co2 = value;
                }
                println!("{} : {} : {}", variable, check[0], value);
                break;
            }
            cloned.retain(|x| x.chars().nth(i).unwrap() == selector.chars().nth(0).unwrap())
        }
    }
    println!("ox * co2: {}", ox * co2);
    ox * co2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        let strings_vec = read_file(&String::from("./src/3_test.txt"));
        assert_eq!(get_epsilon_and_gamma_rate(&strings_vec, 5), 198);
    }
    #[test]
    fn test_b() {
        let strings_vec = read_file(&String::from("./src/3_test.txt"));
        assert_eq!(get_ox_and_co2_rating(&strings_vec, 5), 230);
    }
}
