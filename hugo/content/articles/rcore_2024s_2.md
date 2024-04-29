---
title: 第二章：批处理系统 · rCore 2024S 随记 
date: 2024-04-29
description: OSComp 2024 随记 · 二
---

# 前言

实际实践参考的文档：[rCore Tutorial Guide 2024S](https://learningos.cn/rCore-Tutorial-Guide-2024S/)。

拥有更为详细细节介绍的 rCore OS 官方文档：[rCore-Tutorial-Book-v3](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/0intro.html)。

# 第二章：应用程序与基本执行环境

## 引言

> 批处理系统 (Batch System) 出现于计算资源匮乏的年代，其核心思想是： 将多个程序打包到一起输入计算机；当一个程序运行结束后，计算机会 自动 执行下一个程序。

## 实践体验

> 本章我们引入了用户程序。为了将内核与应用解耦，我们将二者分成了两个仓库，分别是存放内核程序的 rCore-Tutorial-Code-20xxx （下称代码仓库，最后几位 x 表示学期）与存放用户程序的 rCore-Tutorial-Test-20xxx （下称测例仓库）。 你首先需要进入代码仓库文件夹并 clone 用户程序仓库（如果已经执行过该步骤则不需要再重复执行）：

```bassh
$ git clone https://github.com/LearningOS/rCore-Tutorial-Code-2024S.git
$ cd rCore-Tutorial-Code-2024S
$ git checkout ch2
$ git clone https://github.com/LearningOS/rCore-Tutorial-Test-2024S.git user
```

> 上面的指令会将测例仓库克隆到代码仓库下并命名为 user ，注意 /user 在代码仓库的 .gitignore 文件中，因此不会出现 .git 文件夹嵌套的问题，并且你在代码仓库进行 checkout 操作时也不会影响测例仓库的内容。

> 在 qemu 模拟器上运行本章代码：

```bash
$ cd os
$ make run LOG=INFO
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
     Removed 0 files
target/riscv64gc-unknown-none-elf/release/ch2b_bad_address target/riscv64gc-unknown-none-elf/release/ch2b_bad_instructions target/riscv64gc-unknown-none-elf/release/ch2b_bad_register target/riscv64gc-unknown-none-elf/release/ch2b_hello_world target/riscv64gc-unknown-none-elf/release/ch2b_power_3 target/riscv64gc-unknown-none-elf/release/ch2b_power_5 target/riscv64gc-unknown-none-elf/release/ch2b_power_7
    Updating crates.io index
  Downloaded lazy_static v1.4.0
  Downloaded spin v0.5.2
  Downloaded buddy_system_allocator v0.6.0
  Downloaded spin v0.9.8
  Downloaded scopeguard v1.2.0
  Downloaded spin v0.7.1
  Downloaded lock_api v0.4.6
  Downloaded 7 crates (127.2 KB) in 0.32s
   Compiling scopeguard v1.2.0
   Compiling spin v0.5.2
   Compiling spin v0.7.1
   Compiling bitflags v1.3.2
   Compiling lock_api v0.4.6
   Compiling lazy_static v1.4.0
   Compiling buddy_system_allocator v0.6.0
   Compiling spin v0.9.8
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 2.89s
[build.py] application ch2b_bad_address start with address 0x80400000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.33s
[build.py] application ch2b_bad_instructions start with address 0x80400000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.34s
[build.py] application ch2b_bad_register start with address 0x80400000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.34s
[build.py] application ch2b_hello_world start with address 0x80400000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.35s
[build.py] application ch2b_power_3 start with address 0x80400000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.36s
[build.py] application ch2b_power_5 start with address 0x80400000
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
    Finished `release` profile [optimized] target(s) in 0.35s
[build.py] application ch2b_power_7 start with address 0x80400000
make[1]: Leaving directory '/home/ezra/2024s-rcore-255doesnotexist/user'
Platform: qemu
    Updating crates.io index
    Updating git repository `https://gitee.com/rcore-os/riscv`
  Downloaded bit_field v0.10.2
  Downloaded riscv-target v0.1.2
  Downloaded rustc_version v0.2.3
  Downloaded semver v0.9.0
  Downloaded semver-parser v0.7.0
  Downloaded bare-metal v0.2.5
  Downloaded 6 crates (64.0 KB) in 0.63s
   Compiling memchr v2.7.2
   Compiling semver-parser v0.7.0
   Compiling regex-syntax v0.8.3
   Compiling lazy_static v1.4.0
   Compiling bitflags v1.3.2
   Compiling spin v0.5.2
   Compiling bit_field v0.10.2
   Compiling log v0.4.21
   Compiling os v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/os)
   Compiling semver v0.9.0
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling aho-corasick v1.1.3
   Compiling regex-automata v0.4.6
   Compiling regex v1.10.4
   Compiling riscv-target v0.1.2
   Compiling riscv v0.6.0 (https://gitee.com/rcore-os/riscv#11d43cf7)
    Finished `release` profile [optimized] target(s) in 8.96s
[rustsbi] RustSBI version 0.3.0-alpha.4, adapting to RISC-V SBI v1.0.0
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
[ INFO] [kernel] .data [0x80209000, 0x80226000)
[ WARN] [kernel] boot_stack top=bottom=0x80236000, lower_bound=0x80226000
[ERROR] [kernel] .bss [0x80236000, 0x80237000)
[kernel] num_app = 7
[kernel] app_0 [0x80209048, 0x8020d0f0)
[kernel] app_1 [0x8020d0f0, 0x80211198)
[kernel] app_2 [0x80211198, 0x80215240)
[kernel] app_3 [0x80215240, 0x802192e8)
[kernel] app_4 [0x802192e8, 0x8021d390)
[kernel] app_5 [0x8021d390, 0x80221438)
[kernel] app_6 [0x80221438, 0x802254e0)
[kernel] Loading app_0
[kernel] PageFault in application, kernel killed it.
[kernel] Loading app_1
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] Loading app_2
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] Loading app_3
Hello, world from user mode program!
[kernel] Loading app_4
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
power_3 [140000/200000]
power_3 [150000/200000]
power_3 [160000/200000]
power_3 [170000/200000]
power_3 [180000/200000]
power_3 [190000/200000]
power_3 [200000/200000]
3^200000 = 871008973(MOD 998244353)
Test power_3 OK!
[kernel] Loading app_5
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
[kernel] Loading app_6
power_7 [10000/160000]
power_7 [20000/160000]
power_7 [30000/160000]
power_7 [40000/160000]
power_7 [50000/160000]
power_7 [60000/160000]
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
7^160000 = 667897727(MOD 998244353)
Test power_7 OK!
All applications completed!
```
> 批处理系统自动加载并运行了所有的用户程序，尽管某些程序出错了。

## 本章代码树

```bash
── os
│   ├── Cargo.toml
│   ├── Makefile (修改：构建内核之前先构建应用)
│   ├── build.rs (新增：生成 link_app.S 将应用作为一个数据段链接到内核)
│   └── src
│       ├── batch.rs(新增：实现了一个简单的批处理系统)
│       ├── console.rs
│       ├── entry.asm
│       ├── lang_items.rs
│       ├── link_app.S(构建产物，由 os/build.rs 输出)
│       ├── linker.ld
│       ├── logging.rs
│       ├── main.rs(修改：主函数中需要初始化 Trap 处理并加载和执行应用)
│       ├── sbi.rs
│       ├── sync(新增：包装了RefCell，暂时不用关心)
│       │   ├── mod.rs
│       │   └── up.rs
│       ├── syscall(新增：系统调用子模块 syscall)
│       │   ├── fs.rs(包含文件 I/O 相关的 syscall)
│       │   ├── mod.rs(提供 syscall 方法根据 syscall ID 进行分发处理)
│       │   └── process.rs(包含任务处理相关的 syscall)
│       └── trap(新增：Trap 相关子模块 trap)
│           ├── context.rs(包含 Trap 上下文 TrapContext)
│           ├── mod.rs(包含 Trap 处理入口 trap_handler)
│           └── trap.S(包含 Trap 上下文保存与恢复的汇编代码)
└── user(新增：应用测例保存在 user 目录下)
   ├── Cargo.toml
   ├── Makefile
   └── src
      ├── bin(基于用户库 user_lib 开发的应用，每个应用放在一个源文件中)
      │   ├── ...
      ├── console.rs
      ├── lang_items.rs
      ├── lib.rs(用户库 user_lib)
      ├── linker.ld(应用的链接脚本)
      └── syscall.rs(包含 syscall 方法生成实际用于系统调用的汇编指令，
                     各个具体的 syscall 都是通过 syscall 来实现的)

cloc os
-------------------------------------------------------------------------------
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            14             62             21            435
Assembly                         3              9             16            106
make                             1             12              4             36
TOML                             1              2              1              9
-------------------------------------------------------------------------------
SUM:                            19             85             42            586
-------------------------------------------------------------------------------
```