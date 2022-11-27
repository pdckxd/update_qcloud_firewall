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
4. Install rust toolchain
    ```bash
    # in container
    $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # use default choose
    $ source "$HOME/.cargo/env"
    $ rustup target add x86_64-unknown-linux-musl
    ```
5. Build project
    ```bash
    # in container
    $ git clone git@github.com:pdckxd/update_qcloud_firewall.git
    $ cd update_qcloud_firewall
    $ make build_hello_x86_musl
    ```
6. Run in openwrt OS
    ```bash
    # in container
    $ scp hello root@192.168.1.1:/tmp
    $ ssh root@192.168.1.1
    $ cd /tmp
    $ ./hello
    ```
## 2. Build for Synology ds6.2.2 (MARVELL Armada 3720 88F3720)
1. In Ubuntu system (better to have 20.04+), Download toolchain from [this link](https://master.dl.sourceforge.net/project/dsgpl/Tool%20Chain/DSM%206.2.2%20Tool%20Chains/Marvell%20Armada%2037xx%20Linux%204.4.59/armada37xx-gcc494_glibc220_armv8-GPL.txz?viasf=1)
    ```bash
    $ sudo apt-get install build-essential
    $ mkdir ~/synology_dev
    $ cd ~/synology_dev
    $ tar Jxvf armada37xx-gcc494_glibc220_armv8-GPL.txz
    $ export PATH=$HOME/Downloads/aarch64-unknown-linux-gnueabi/bin:$PATH
    # test if toolchain works well
    $ cat hello.c
    #include <stdio.h>

    int main() {
        printf("Hello, world!");
    }
    $ aarch64-unknown-linux-gnueabi-gcc main.c
    $ file a.out
    a.out: ELF 64-bit LSB executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, for GNU/Linux 3.7.0, not stripped
    # scp to the synology device
    $ scp a.out admin@192.168.3.25:/tmp
    $ ssh admin@192.168.3.25
    # in device
    $ cd /tmp
    $ ./a.out
    Hello, world!
    ```
2. Install rust toolchain
    ```bash
    # in ubuntu os
    $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # use default choose
    $ source "$HOME/.cargo/env"
    $ rustup target add aarch64-unknown-linux-gnu
    ```
3. Build project
    ```bash
    # in ubuntu os
    $ git clone git@github.com:pdckxd/update_qcloud_firewall.git
    $ cd update_qcloud_firewall
    $ make build_hello_aarch64
    ```
4. Run on synology OS
    ```bash
    # in ubuntu os
    $ scp hello root@192.168.3.25:/tmp
    $ ssh root@192.168.3.25
    # in device
    $ cd /tmp
    $ ./hello
    ```

## 3. Build for Ios 15.3 (for flutter FFI)

## TODO: to reduce the size of binary