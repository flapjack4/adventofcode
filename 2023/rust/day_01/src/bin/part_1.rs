use std::fs;

fn main() {
	let file = fs::read_to_string("./input.txt").expect("wtf where's my input");

	let mut total = 0;
	for line in file.split("\n") {
		let mut num_str: String = String::new();

		for c in line.chars() {
			if c.is_ascii_digit() {
				num_str.push_str(&c.to_string());
				break;
			}
		}

		for c in line.chars().rev() {
			if c.is_ascii_digit() {
				num_str.push_str(&c.to_string());
				break;
			}
		}

		total += match num_str.parse() {
			Ok(num) => num,
			Err(_) => 0,
		};
	}
	println!("Total: {total}");
}