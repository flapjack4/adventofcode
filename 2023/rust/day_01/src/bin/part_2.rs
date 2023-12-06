use std::fs;

const ARR: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
	let file = fs::read_to_string("./input.txt").expect("wtf where's my input");

	let a: [String; 9] = ["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(),
					 "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(),
					  "nine".to_string()];


	let x = "two1nine
	eightwothree
	abcone2threexyz
	xtwone3four
	4nineeightseven2
	zoneight234
	7pqrstsixteen";

	let mut total = 0;
	for line in file.trim().split_whitespace() {
		let mut num_str: String = String::new();
		println!("Current line: {line}");
		// println!("Size of current line: {}", line.len());
		// for c in line.chars() {
		// 	println!("C: {}", c);
		// }

		// replace
		// let l = line
		// 			.replace("one", "1")
		// 			.replace("two", "2")
		// 			.replace("three", "3")
		// 			.replace("four", "4")
		// 			.replace("five", "5")
		// 			.replace("six", "6")
		// 			.replace("seven", "7")
		// 			.replace("eight", "8")
		// 			.replace("nine", "9");
			
		// println!("l: {l}");
		let mut i = 0;
		for c in line.chars() {
			if c.is_ascii_digit() {
				num_str.push_str(&c.to_string());
				break;
			} else {
				let mut done = false;
				for n in &a {
					// println!("n: {n}");
					// println!("i: {i}");
					// println!("n len: {}", n.len());
					// println!("line len: {}", line.len());
					// println!("line: {}", line);
					// println!("ls: {}", &line[i..(i+n.len())]);
					if (i+n.len() < line.len()) && &line[i..(i+n.len())] == n.as_str() {
						// println!("Answer: {n}");
						if n.as_str() == "one" { 
							num_str.push_str("1");
						} else if n.as_str() == "two" {
							num_str.push_str("2");
						} else if n.as_str() == "three" {
							num_str.push_str("3");
						} else if n.as_str() == "four" {
							num_str.push_str("4");
						} else if n.as_str() == "five" {
							num_str.push_str("5");
						} else if n.as_str() == "six" {
							num_str.push_str("6");
						} else if n.as_str() == "seven" {
							num_str.push_str("7");
						} else if n.as_str() == "eight" {
							num_str.push_str("8");
						} else if n.as_str() == "nine" {
							num_str.push_str("9");
						}
						done = true;
						break;
					}
				}
				if done {
					break;
				}
			}
			i += 1;
		}

		i = line.len()-1;
		for c in line.chars().rev() {
			if c.is_ascii_digit() {
				num_str.push_str(&c.to_string());
				break;
			} else {
				let mut done = false;
				for n in &a {
					// println!("n: {n}");
					// println!("i: {i}");
					// println!("n len: {}", n.len());
					// println!("line len: {}", line.len());
					// println!("line: {}", line);
					// if i+n.len() < line.len() {
					// 	println!("ls: {}", &line[i..(i+n.len())]);
					// }
					// println!("{n}");
					if (i+n.len() <= line.len()) && &line[i..(i+n.len())] == n.as_str() {
						if n.as_str() == "one" { 
							num_str.push_str("1");
						} else if n.as_str() == "two" {
							num_str.push_str("2");
						} else if n.as_str() == "three" {
							num_str.push_str("3");
						} else if n.as_str() == "four" {
							num_str.push_str("4");
						} else if n.as_str() == "five" {
							num_str.push_str("5");
						} else if n.as_str() == "six" {
							num_str.push_str("6");
						} else if n.as_str() == "seven" {
							num_str.push_str("7");
						} else if n.as_str() == "eight" {
							num_str.push_str("8");
						} else if n.as_str() == "nine" {
							num_str.push_str("9");
						}
						done = true;
						break;
					}
				}
				if done {
					break;
				}
			}
			i -= 1;
		}
		println!("num to add: {}", num_str);
		total += match num_str.parse() {
			Ok(num) => num,
			Err(_) => 0,
		};
		// println!("Total: {total}");
	}
	println!("Total: {total}");




}