BIN=$(HOME)/dots/personal/.local/bin

build:
	cargo build --release
	@make load_bin

test:
	@sh ./test

# copies built binary to a path specified by $BIN
load_bin:
	@rm -f $(BIN)/statc
	@cp ./target/release/statc $(BIN)

.PHONY: test
