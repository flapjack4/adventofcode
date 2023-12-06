fn main() {
	let file = fs::read_to_string("./input.txt").expect("wtf where's my input");
	println!("{file}");
}