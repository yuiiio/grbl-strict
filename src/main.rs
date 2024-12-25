use std::env;
use std::fs;

fn main() {
    let file_name: &str = &env::args().last().unwrap();
    if let Ok(contents) = fs::read_to_string(file_name) {
        for line in contents.lines() {
            if !line.contains("G2") {
                if !line.contains("G3") {
                    // does not contain G2 or G3
                    println!("{}", line);
                    continue;
                }
            }

            let mut should_replace: bool = false;
            let elements = line.split_ascii_whitespace();
            for element in elements {
                if element.contains("I") {
                    // default freecad persision is 3 (x.xxx)
                    // too large G2,G3 I,J,K is strait line,
                    // so can convert to G1
                    //string cast to f64
                    let i_num: f64 = element.strip_prefix("I").unwrap().parse().unwrap();
                    // (+/-9999.xxx) <= under 9 charactars so OK for grbl?
                    if i_num.abs() >= 9999.0 {
                        //println!("{}", i_num); // debug
                        should_replace = true;
                    }
                }
                if element.contains("J") {
                    let j_num: f64 = element.strip_prefix("J").unwrap().parse().unwrap();
                    if j_num.abs() >= 9999.0 {
                        //println!("{}", j_num); // debug
                        should_replace = true;
                    }
                }
                if element.contains("K") {
                    let k_num: f64 = element.strip_prefix("K").unwrap().parse().unwrap();
                    if k_num.abs() >= 9999.0 {
                        //println!("{}", j_num); // debug
                        should_replace = true;
                    }
                }
            }

            if should_replace == true {
                let replace_to_g1 = line.replace("G2", "G1").replace("G3", "G1");
                let (replace_to_g1, f_num) = replace_to_g1.split_once("F").unwrap();
                let (replace_to_g1, _) = replace_to_g1.split_once("I").unwrap();

                let mut replace_to_g1 = replace_to_g1.to_string();
                replace_to_g1.push_str("F");
                replace_to_g1.push_str(f_num);

                println!("{}", replace_to_g1);
            } else {
                println!("{}", line); // I, J is under float32
            }
        }
    } else {
        println!("failed to read file: {}", file_name);
    }
}
