---
title: 第三章：多道程序与分时多任务 · rCore 2024S 随记 
date: 2024-04-29
description: OSComp 2024 随记 · 三
---

- 本部分将试着毛估估的完成 ch3。由于本人同时在学习 RISC-V 汇编，涉及的内容可能解释的不那么详细。

# 前言

实际实践参考的文档：[rCore Tutorial Guide 2024S](https://learningos.cn/rCore-Tutorial-Guide-2024S/)。

拥有更为详细细节介绍的 rCore OS 官方文档：[rCore-Tutorial-Book-v3](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/0intro.html)。

# 第三章：多道程序与分时多任务

## 引言

### 概述

> 本章的目标是实现分时多任务系统，它能并发地执行多个用户程序，并调度这些程序。为此需要实现：

> - 一次性加载所有用户程序，减少任务切换开销；
> - 支持任务切换机制，保存切换前后程序上下文；
> - 支持程序主动放弃处理器，实现 yield 系统调用；
> - 以时间片轮转算法调度用户程序，实现资源的时分复用。

在 qemu 模拟器上运行本章代码：

```bash
$ cd os
$ make run
```

运行代码，看到用户程序交替输出信息：

```bash
$ make run
(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add riscv64gc-unknown-none-elf
riscv64gc-unknown-none-elf (installed)
cargo install cargo-binutils
    Updating crates.io index
     Ignored package `cargo-binutils v0.3.6` is already installed, use --force to override
rustup component add rust-src
info: component 'rust-src' is up to date
rustup component add llvm-tools-preview
info: component 'llvm-tools' for target 'x86_64-unknown-linux-gnu' is up to date
make[1]: Entering directory '/home/ezra/2024s-rcore-255doesnotexist/user'
     Removed 138 files, 3.3MiB total
target/riscv64gc-unknown-none-elf/release/ch2b_bad_address target/riscv64gc-unknown-none-elf/release/ch2b_bad_instructions target/riscv64gc-unknown-none-elf/release/ch2b_bad_register target/riscv64gc-unknown-none-elf/release/ch2b_hello_world target/riscv64gc-unknown-none-elf/release/ch2b_power_3 target/riscv64gc-unknown-none-elf/release/ch2b_power_5 target/riscv64gc-unknown-none-elf/release/ch2b_power_7 target/riscv64gc-unknown-none-elf/release/ch3b_yield0 target/riscv64gc-unknown-none-elf/release/ch3b_yield1 target/riscv64gc-unknown-none-elf/release/ch3b_yield2
   Compiling scopeguard v1.2.0
   Compiling spin v0.5.2
   Compiling spin v0.7.1
   Compiling bitflags v1.3.2
   Compiling lock_api v0.4.6
   Compiling lazy_static v1.4.0
   Compiling buddy_system_allocator v0.6.0
   Compiling spin v0.9.8
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 1.19s
[build.py] application ch2b_bad_address start with address 0x80400000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.36s
[build.py] application ch2b_bad_instructions start with address 0x80420000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.36s
[build.py] application ch2b_bad_register start with address 0x80440000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.44s
[build.py] application ch2b_hello_world start with address 0x80460000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.41s
[build.py] application ch2b_power_3 start with address 0x80480000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.53s
[build.py] application ch2b_power_5 start with address 0x804a0000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.37s
[build.py] application ch2b_power_7 start with address 0x804c0000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.37s
[build.py] application ch3b_yield0 start with address 0x804e0000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.35s
[build.py] application ch3b_yield1 start with address 0x80500000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.36s
[build.py] application ch3b_yield2 start with address 0x80520000
make[1]: Leaving directory '/home/ezra/2024s-rcore-255doesnotexist/user'
Platform: qemu
   Compiling spin v0.7.1
   Compiling riscv v0.6.0 (https://github.com/rcore-os/riscv#11d43cf7)
   Compiling os v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/os)
   Compiling buddy_system_allocator v0.6.0
    Finished `release` profile [optimized] target(s) in 1.82s
[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
[rustsbi] Platform Name      : riscv-virtio,qemu
[rustsbi] Platform SMP       : 1
[rustsbi] Platform Memory    : 0x80000000..0x88000000
[rustsbi] Boot HART          : 0
[rustsbi] Device Tree Region : 0x87000000..0x87000ef2
[rustsbi] Firmware Address   : 0x80000000
[rustsbi] Supervisor Address : 0x80200000
[rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
[rustsbi] pmp02: 0x80000000..0x80200000 (---)
[rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
[rustsbi] pmp04: 0x88000000..0x00000000 (-wr)
[kernel] Hello, world!
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
Hello, world from user mode program!
power_3 [10000/200000]
power_3 [20000/200000]
power_3 [30000/200000]
power_3 [40000/200000]
power_3 [50000/200000]
power_3 [60000/200000]
power_3 [70000/200000]
power_3 [80000/200000]
power_3 [90000/200000]
power_3 [100000/200000]
power_3 [110000/200000]
power_3 [120000/200000]
power_3 [130000/200000]
power_5 [10000/140000]
power_5 [20000/140000]
power_5 [30000/140000]
power_5 [40000/140000]
power_5 [50000/140000]
power_5 [60000/140000]
power_5 [70000/140000]
power_5 [80000/140000]
power_5 [90000/140000]
power_5 [100000/140000]
power_5 [110000/140000]
power_5 [120000/140000]
power_5 [130000/140000]
power_5 [140000/140000]
5^140000 = 386471875(MOD 998244353)
Test power_5 OK!
power_7 [10000/160000]
power_7 [20000/160000]
power_7 [30000/160000]
power_7 [40000/160000]
power_7 [50000/160000]
power_7 [60000/160000]
AAAAAAAAAA [1/5]
BBBBBBBBBB [1/5]
CCCCCCCCCC [1/5]
power_3 [140000/200000]
power_3 [150000/200000]
power_3 [160000/200000]
power_3 [170000/200000]
power_3 [180000/200000]
power_3 [190000/200000]
power_3 [200000/200000]
3^200000 = 871008973(MOD 998244353)
Test power_3 OK!
power_7 [70000/160000]
power_7 [80000/160000]
power_7 [90000/160000]
power_7 [100000/160000]
power_7 [110000/160000]
power_7 [120000/160000]
power_7 [130000/160000]
power_7 [140000/160000]
power_7 [150000/160000]
power_7 [160000/160000]
AAAAAAAAAA [2/5]
BBBBBBBBBB [2/5]
CCCCCCCCCC [2/5]
7^160000 = 667897727(MOD 998244353)
Test power_7 OK!
AAAAAAAAAA [3/5]
BBBBBBBBBB [3/5]
CCCCCCCCCC [3/5]
AAAAAAAAAA [4/5]
BBBBBBBBBB [4/5]
CCCCCCCCCC [4/5]
AAAAAAAAAA [5/5]
BBBBBBBBBB [5/5]
CCCCCCCCCC [5/5]
Test write A OK!
Test write B OK!
Test write C OK!
[kernel] Panicked at src/task/mod.rs:135 All applications completed!
```

- 由于我们还没有写任何代码，因此出现 panic 是正常的。

### 本章代码树

```bash
$ tree --gitignore
.
├── bootloader
│   └── rustsbi-qemu.bin
├── Dockerfile
├── LICENSE
├── Makefile
├── os
│   ├── build.rs
│   ├── Cargo.toml
│   ├── Makefile
│   └── src
│       ├── config.rs
│       ├── console.rs
│       ├── entry.asm
│       ├── heap_alloc.rs
│       ├── lang_items.rs
│       ├── linker.ld
│       ├── loader.rs
│       ├── logging.rs
│       ├── main.rs
│       ├── sbi.rs
│       ├── sync
│       │   ├── mod.rs
│       │   └── up.rs
│       ├── syscall
│       │   ├── fs.rs
│       │   ├── mod.rs
│       │   └── process.rs
│       ├── task
│       │   ├── context.rs
│       │   ├── mod.rs
│       │   ├── switch.rs
│       │   ├── switch.S
│       │   └── task.rs
│       ├── timer.rs
│       └── trap
│           ├── context.rs
│           ├── mod.rs
│           └── trap.S
├── README.md
├── rust-toolchain.toml
└── user

9 directories, 33 files
```