# update_qcloud_firewall

## 1. Build for openwrt x86_64
1. Pull openwrt docker container with specific SDK version
    ```bash
    $ docker pull openwrtorg/sdk:x86_64-openwrt-22.03
    ```
2. Run docker container
    ```bash
    $ docker run -it openwrtorg/sdk:x86_64-openwrt-22.03 /bin/bash --name x86_64_openwrt_SDK_2203
    # optional: to re-launch the stopped container and continue the work
    $ docker start x86_64_openwrt_SDK_2203
    $ docker exec -it x86_64_openwrt_SDK_2203 /bin/bash
    ```
3. Verify SDK works
    ```bash
    # in container
    $ echo $PWD
    /home/build
    $ export STAGING_DIR="$PWD/staging_dir"
    $ export PATH="$PWD/$(echo staging_dir/toolchain-*/bin):$PATH"

    $ x86_64-openwrt-linux-musl-gcc -v
    Reading specs from /home/build/openwrt/staging_dir/toolchain-x86_64_gcc-11.2.0_musl/bin/../lib/gcc/x86_64-openwrt-linux-musl/11.2.0/specs
    ...omit a lot of output...
    gcc version 11.2.0 (OpenWrt GCC 11.2.0 r19803-9a599fee93)
    $ cat hello.c
    #include <stdio.h>

    int main() {
        printf("Hello, world!");
    }

    $ x86_64-openwrt-linux-musl-gcc hello.c
    $ file a.out
    a.out: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-musl-x86_64.so.1, with debug_info, not stripped

    $ scp a.out root@192.168.1.1:/tmp
    $ ssh root@192.168.1.1
    $ cd /tmp
    $ ./hello
    Hello, world!
    ```
4. Install rust toolchian
    ```bash
    $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # use default choose
    $ source "$HOME/.cargo/env"
    $ rustup target add x86_64-unknown-linux-musl
    ```
5. Build project
    ```bash
    $ git clone git@github.com:pdckxd/update_qcloud_firewall.git
    $ cd update_qcloud_firewall
    $ make build_hello_x86_musl
    ```
6. Run on openwrt OS
    ```bash
    $ scp hello root@192.168.1.1:/tmp
    $ ssh root@192.168.1.1
    $ cd /tmp
    $ ./hello
    ```
## 2. Build for Synology ds6.2.2 (MARVELL Armada 3720 88F3720)

## 3. Build for Ios 15.3 (for flutter FFI)

## TODO: to reduce the size of binary