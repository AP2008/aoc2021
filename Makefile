build:
	cargo build --release

run:
	cargo run --release

bench:
	cargo bench

readme:
	python3 parse_bench.py | python3 gen_readme.py > README.md
