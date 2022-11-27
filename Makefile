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
	@echo "  make build_rust_x86_musl"

build_hello: build_rust
	gcc -o hello hello.c target/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl

build_hello_x86_musl: build_rust_x86_musl
	x86_64-openwrt-linux-musl-gcc -o hello hello.c target/x86_64-unknown-linux-musl/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl

build_rust:
	cargo build --release --verbose

build_rust_x86_musl:
	CC=x86_64-openwrt-linux-musl-gcc cargo build --target=x86_64-unknown-linux-musl --release --verbose

# optional step
test_rust:
	cargo test -- --nocapture

