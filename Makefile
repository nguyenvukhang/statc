BIN=$(HOME)/dots/personal/.local/bin

build:
	cargo test
	cargo build --release
	make load_bin

test:
	cargo test --no-fail-fast

version:
	@CARGO_MANIFEST_DIR=$(PWD) bash scripts/bump-ver.sh

# copies built binary to a path specified by $BIN
load_bin:
	@rm -f $(BIN)/statc
	@cp ./target/release/statc $(BIN)

.PHONY: test
