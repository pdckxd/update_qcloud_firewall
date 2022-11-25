# step 2, after that, run ./hello to see the output
# Sample output:
# 	Creating webapi client
# 	got ip_config
# 	ip: 127.0.0.1
# 	user_agent address: (nil)
# 	Freeing webapi client
all:
	gcc -o hello hello.c target/release/libupdate_qcloud_firewall.a -lpthread -lm -ldl


# step 1
build_rust:
	cargo build --release --verbose


# optional step
test_rust:
	cargo test -- --nocapture

