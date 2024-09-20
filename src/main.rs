use std::{collections::HashMap, env, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 4 {
		println!("usage: ./modify_color_num ui_chara_db.prc color_num.txt output");
		return;
	}

	match file_to_hashmap(Path::new(&args[2])) {
		Err(_) => println!("Failed open txt file"),
		Ok(map) => match prcx::open(Path::new(&args[1])) {
			Err(_) => println!("Failed open prc file"),
			Ok(source) => {
				let mut modded = source.clone();
				if let prcx::ParamKind::List(db_root) = &mut modded.0[0].1 {
					for elem in &mut db_root.0 {
						change_value(elem, &map);
					}
				}
				gen_xml_diff(&source, &modded, &Path::new(&args[3]));
			}
		}
	}
}

fn file_to_hashmap(path: &Path) -> Result<HashMap<String, Option<u8>>, ()> {
	match File::open(path) {
		Err(_) => Result::Err(()),
		Ok(file) => {
			let file = BufReader::new(file);
			let mut map: HashMap<String, Option<u8>> = HashMap::new();
			for line in file.lines() {
				if let Ok(line) = line {
					let split_line: Vec<&str> = line.split('=').collect();
					match split_line[1].parse::<u8>() {
						Ok(value) => map.insert(split_line[0].to_string(), Option::Some(value)),
						Err(_) => map.insert(split_line[0].to_string(), Option::None)
					};
				}
			}
			Result::Ok(map)
		}
	}
}

fn change_value(fighter: &mut prcx::ParamKind, values: &HashMap<String, Option<u8>>) {
	if let prcx::ParamKind::Struct(fighter) = fighter {
		if let prcx::ParamKind::Str(name) = &mut fighter.0[1].1 {
			if let Some(Some(value)) = values.get(name) {
				fighter.0[33].1 = prcx::ParamKind::U8(*value);
			}
		}
	}
}

fn gen_xml_diff(source: &prcx::ParamStruct, modded: &prcx::ParamStruct, output: &Path) {
	let diff = prcx::generate_patch(&source, &modded).unwrap();
	match diff {
		Some(diff) => {
			let mut file = std::io::BufWriter::new(std::fs::File::create(output).unwrap());
			match prcx::write_xml(&diff, &mut file) {
				Ok(_) => println!("Successfuly generated xml diff !"),
				Err(_) => print!("Failed to create xml diff")
			}
		}
		None => println!("No differences were found between the two files"),
	}
}