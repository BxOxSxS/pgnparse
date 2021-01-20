use pgnparse::parser::*;

fn main(){
	let pgn = r#"[FEN "8/8/8/8/8/7k/8/7K w - - 0 1"]
[White "White"]
[Black "Black"]
[Variant "Atomic"]

1. Kh2 Kg2
"#;
	
	let result = parse_pgn_to_rust_struct(pgn);
	
	println!("{:?}", result);
	
	let result = parse_pgn_to_json_string(pgn);
	
	println!("{}", result);

	let iter = PgnIterator::new("test.pgn");

	let mut i = 0;

	if let Some(iter) = iter {
		for pgn in iter {
			println!("************\npgn {}\n************\n\n{}", i + 1, pgn);

			i += 1;
		}
	}
}
