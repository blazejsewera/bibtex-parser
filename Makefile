build:
	cargo build

release-build-macos:
	cargo build --release \
		--target x86_64-apple-darwin \
		--target aarch64-apple-darwin

release-build-linux:
	cargo build --release \
		--target x86_64-unknown-linux-gnu
