# step 2, after that, run ./hello to see the output
# Sample output:
# 	Creating webapi client
# 	got ip_config
# 	ip: 127.0.0.1
# 	user_agent address: (nil)
# 	Freeing webapi client
all:
	@echo Usage:
	@echo "  make build_hello"
	@echo "  make build_rust"
	@echo "  make test_rust"
	@echo "  make build_hello_x86_musl"
	@echo "  make build_hello_aarch64"
	@echo "  make build_hello_aarch64_apple"
	@echo "  make build_hello_x86_apple"
	@echo "  make build_rust_x86_musl"
	@echo "  make build_rust_aarch64"
	@echo "  make build_rust_aarch64_apple"
	@echo "  make build_rust_x86_apple"

# general build with default sdk in system
build_hello: build_rust
	gcc -o hello hello.c target/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl

build_hello_x86_musl: build_rust_x86_musl
	x86_64-openwrt-linux-musl-gcc -o hello hello.c target/x86_64-unknown-linux-musl/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl

build_hello_aarch64: build_rust_aarch64
	aarch64-unknown-linux-gnueabi-gcc -o hello hello.c target/aarch64-unknown-linux-gnu/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl

build_hello_aarch64_apple: build_rust_aarch64_apple
	gcc -o hello hello.c target/aarch64-apple-darwin/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl -framework CoreFoundation -framework Security

build_hello_x86_apple: build_rust_x86_apple
	gcc -o hello hello.c target/x86_64-apple-darwin/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl -framework CoreFoundation -framework Security

build_rust:
	cargo build --release --verbose

build_rust_x86_musl:
	CC=x86_64-openwrt-linux-musl-gcc cargo build --target=x86_64-unknown-linux-musl --release --verbose

build_rust_aarch64:
	CC=aarch64-unknown-linux-gnueabi-gcc cargo build --target aarch64-unknown-linux-gnu --release --verbose

build_rust_x86_apple:
	cargo build --target x86_64-apple-darwin --release --verbose

build_rust_aarch64_apple:
	cargo build --target aarch64-apple-darwin --release --verbose

# optional step
test_rust:
	cargo test -- --nocapture

