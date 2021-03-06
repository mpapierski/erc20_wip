CONTRACT_TARGET_DIR       = target/wasm32-unknown-unknown/release

prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p erc20 --target wasm32-unknown-unknown
	wasm-strip $(CONTRACT_TARGET_DIR)/erc20.wasm

test-only:
	cargo test -- tests

copy-wasm-file-to-test:
	cp $(CONTRACT_TARGET_DIR)/erc20.wasm contract-tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf tests/wasm/contract.wasm
