BIN=$(HOME)/dots/personal/.local/bin/stats

build:
	cargo build --release
	rm $(BIN)
	cp ./target/release/stats $(BIN)
