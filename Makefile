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
	@echo "  make build_hello_openwrt_x86_musl"
	@echo "  make build_hello_aarch64"
	@echo "  make build_hello_aarch64_apple"
	@echo "  make build_hello_x86_apple"
	@echo "  make build_hello_ios_apple"
	@echo "  make build_hello_aarch64_ios_sim_apple"
	@echo "  make build_rust_openwrt_x86_musl"
	@echo "  make build_rust_synology_aarch64"
	@echo "  make build_rust_aarch64_apple"
	@echo "  make build_rust_x86_apple"
	@echo "  make build_rust_ios_apple"
	@echo "  make build_rust_aarch64_ios_sim_apple"

# general build with default sdk in system
build_hello: build_rust
	gcc -o hello hello.c dotenv.c target/release/libupdate_qcloud_firewall.a -I. -lpthread -lm -ldl -lcrypto -lssl

build_hello_openwrt_x86_musl: build_rust_openwrt_x86_musl
	x86_64-openwrt-linux-musl-gcc -o hello hello.c target/x86_64-unknown-linux-musl/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl

build_hello_aarch64: build_rust_synology_aarch64
	aarch64-unknown-linux-gnueabi-gcc -o hello hello.c target/aarch64-unknown-linux-gnu/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl

build_hello_aarch64_apple: build_rust_aarch64_apple
	gcc -o hello hello.c target/aarch64-apple-darwin/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl -framework CoreFoundation -framework Security

build_hello_x86_apple: build_rust_x86_apple
	gcc -o hello hello.c target/x86_64-apple-darwin/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl -framework CoreFoundation -framework Security

build_hello_ios_apple: build_rust_ios_apple
	$(eval COMPILER=$(shell xcrun --sdk iphoneos --find clang))
	$(eval SYSROOT=$(shell xcrun --sdk iphoneos --show-sdk-path))
	@echo $(COMPILER)
	@echo $(SYSROOT)
	# @COMPILER="`xcrun --sdk iphoneos --find clang` -isysroot `xcrun --sdk iphoneos --show-sdk-path` -arch arm64"
	$(COMPILER) -o hello hello.c target/aarch64-apple-ios/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl -isysroot $(SYSROOT) -framework CoreFoundation -framework Security -arch arm64

build_hello_aarch64_ios_sim_apple: build_rust_aarch64_ios_sim_apple
	$(eval COMPILER=$(shell xcrun --sdk iphoneos --find clang))
	$(eval SYSROOT=$(shell xcrun --sdk iphonesimulator --show-sdk-path))
	@echo $(COMPILER)
	@echo $(SYSROOT)
	# @COMPILER="`xcrun --sdk iphoneos --find clang` -isysroot `xcrun --sdk iphoneos --show-sdk-path` -arch arm64"
	$(COMPILER) -o hello hello.c target/aarch64-apple-ios-sim/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl -isysroot $(SYSROOT) -framework CoreFoundation -framework Security -arch arm64



build_rust:
	cargo build --release --verbose

build_rust_openwrt_x86_musl:
	CC=x86_64-openwrt-linux-musl-gcc cargo build --target=x86_64-unknown-linux-musl --release --verbose

build_rust_synology_aarch64:
	CC=aarch64-unknown-linux-gnueabi-gcc cargo build --target aarch64-unknown-linux-gnu --release --verbose

build_rust_x86_apple:
	cargo build --target x86_64-apple-darwin --release --verbose

build_rust_aarch64_apple:
	cargo build --target aarch64-apple-darwin --release --verbose
	
build_rust_ios_apple:
	cargo build --target aarch64-apple-ios --release --verbose

build_rust_aarch64_ios_sim_apple:
	cargo build --target aarch64-apple-ios-sim --release --verbose

# optional step
test_rust:
	cargo test -- --test-threads=1 --nocapture

