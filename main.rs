use std::env;
use std::process;

fn	usage() {
	println!("
		Usage: ./main [-a|-d] [\"numbers\"]
	");
	process::exit(0)
}

fn	paarse_numbers(s: &str) -> Vec<i32> {
	return s.split(' ')
			.map(|num| num.trim().parse()
			.expect(&format!("Error: '{}' is not a valid integer", num)))
			.collect();
}

fn	simple_sort(v: &mut Vec<i32>, option: bool) {
	let n = v.len();

	for i in 0..n-1 {
		for j in 0..n-1-i {
			if (option && v[j] > v[j + 1]) || (!option && v[j] < v[j + 1]) {
				let tmp = v[j];
				v[j] = v[j+1];
				v[j+1] = tmp;
			}
		}
	}
}

fn	parse_option(a: &str) -> bool {
	return match a {
		"-a" => true,
		"-d" => false,
		_ => {
			eprintln!("Invalid argument: {}", a);
			std::process::exit(1);
		}
	};
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 3 {
		usage();
	}

	let flag = &args[1];
	let numbers_str = &args[2];

	let option = parse_option(flag);
	let mut numbers_list = paarse_numbers(numbers_str);
	simple_sort(&mut numbers_list, option);

	println!("Result: {:?}", numbers_list);
}
