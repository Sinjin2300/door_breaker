use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Range Based Code Bruteforcer",
    long_about = None,
)]
struct Args {
    /// File to read in
    #[arg(short, long)]
    input: String,
}

fn main() {
    //Read input file
    let args = Args::parse();
    let mut information: Vec<Vec<Digit>> = Vec::new();
    if let Ok(lines) = read_file(&args.input) {
        for line in lines {
            let digits = parse_entry(&line);
            match digits {
                Ok(digits) => {
                    if digits.len() > 0 {
                        information.push(digits);
                    }
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
    } else {
        println!("Couldn't Read File: {}", args.input);
    }

    //Begin Eliminating possibilities
    let pruned = prune_info(information);
    let mut expanded: Vec<Vec<u8>> = Vec::new();
    expanded.push(Vec::new());
    match pruned {
        Ok(vals) => {
            for digit in vals {
                match digit {
                    Digit::Single(num) => {
                        expanded.iter_mut().for_each(|x| x.push(num));
                    }
                    Digit::Range(nums) => {
                        let chunk_size = expanded.len();
                        for _ in 1..nums.len() {
                            expanded.append(&mut expanded.clone());
                        }
                        let mut num_iter = nums.into_iter();
                        let mut curr = 0;
                        for (i, val) in expanded.iter_mut().enumerate() {
                            if i % chunk_size == 0 {
                                curr = num_iter.next().unwrap();
                            }

                            val.push(curr);
                        }
                    }
                }
            }
        }
        Err(error) => {
            println!("Error pruning info: {}", error);
        }
    }
    println!("-Avaliable Combinations-");
    for guess in expanded {
        let crunch = guess
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        println!("[{}]", crunch);
    }
}

#[derive(Debug, Clone)]
enum Digit {
    Range(Vec<u8>),
    Single(u8),
}

fn prune_info(info: Vec<Vec<Digit>>) -> Result<Vec<Digit>, Error> {
    //Trivial Cases
    if info.len() == 0 {
        return Err(Error::new(io::ErrorKind::InvalidData, "Zero Length Input"));
    }

    if info.len() == 1 {
        return Ok(info.first().unwrap().to_vec());
    }
    let mut output: Vec<Digit> = Vec::new();

    //Check if they are all the same lengths
    let length_uniform = info.iter().all(|v| v.len() == info[0].len());
    if !length_uniform {
        // info.iter().for_each(|x| println!("Size: {}", x.len()));
        return Err(Error::new(
            io::ErrorKind::InvalidData,
            "Size Mismatch Error",
        ));
    }

    //Fill the Vec
    let mut tranposed: Vec<Vec<Digit>> = Vec::new();
    for _ in 0..info[0].len() {
        tranposed.push(Vec::new());
    }

    for row in info {
        for (i, column) in row.into_iter().enumerate() {
            tranposed[i].push(column);
        }
    }
    //Start Eliminating Data
    for set in tranposed {
        //Wildcard Digit
        let mut acc: Digit = Digit::Range(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        for data in set {
            match data {
                Digit::Range(nums) => match acc {
                    Digit::Range(vals) => {
                        let mut pruned: Vec<u8> = Vec::new();
                        let _ = nums.into_iter().for_each(|f| {
                            if vals.contains(&f) {
                                pruned.push(f);
                            }
                        });

                        if pruned.len() == 0 {
                            return Err(Error::new(
                                io::ErrorKind::InvalidInput,
                                "Conflicting Digits in Ranges",
                            ));
                        } else if pruned.len() == 1 {
                            acc = Digit::Single(pruned[0]);
                        } else {
                            acc = Digit::Range(pruned);
                        }
                    }
                    Digit::Single(val) => {
                        if !nums.contains(&val) {
                            return Err(Error::new(
                                io::ErrorKind::InvalidInput,
                                "Conflicting Digits in Range to Single",
                            ));
                        }
                    }
                },
                Digit::Single(num) => match acc {
                    Digit::Single(val) => {
                        if num != val {
                            return Err(Error::new(
                                io::ErrorKind::InvalidInput,
                                "Conflicting Digits in Singles",
                            ));
                        }
                    }
                    Digit::Range(vals) => {
                        if vals.contains(&num) {
                            acc = Digit::Single(num);
                        } else {
                            return Err(Error::new(
                                io::ErrorKind::InvalidInput,
                                "Conflicting Digits in Single to Range",
                            ));
                        }
                    }
                },
            }
        }
        output.push(acc);
    }
    //Exit Ok
    return Ok(output);
}

fn parse_entry(input: &String) -> Result<Vec<Digit>, Error> {
    let mut output: Vec<Digit> = Vec::new();
    let mut chars = input.chars();
    let mut current = chars.next();
    while current != None {
        match &current.unwrap() {
            ',' | '[' | ']' | ' ' => {}
            '*' => output.push(Digit::Range(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
            c if c.is_numeric() => output.push(Digit::Single(c.to_digit(10).unwrap() as u8)),
            '(' => {
                let mut nums: Vec<u8> = Vec::new();
                current = chars.next();
                while current != None {
                    match &current.unwrap() {
                        ',' | ' ' => {}
                        ')' => break,
                        c if c.is_numeric() => nums.push(c.to_digit(10).unwrap() as u8),
                        _ => {
                            return Err(Error::new(
                                io::ErrorKind::InvalidData,
                                "Invalid Char in Group",
                            ))
                        }
                    }
                    current = chars.next();
                }
                output.push(Digit::Range(nums));
            }
            _ => {
                return Err(Error::new(io::ErrorKind::InvalidData, "Invalid Char"));
            }
        }
        current = chars.next();
    }
    return Ok(output);
}
fn read_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    //Try to read the file and error if failure
    let file = File::open(filename)?;

    //Make the buf reader to read the lines of the file
    let reader = BufReader::new(file);

    //Output vector for the strings
    let mut output: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(linestr) => output.push(linestr),
            Err(error) => {
                println!("{}", error)
            }
        }
    }
    Ok(output)
}
