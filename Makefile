run:
	cargo run -- input.txt

build:
	cargo build -r
	mv ./target/release/e_to_10k_check .
