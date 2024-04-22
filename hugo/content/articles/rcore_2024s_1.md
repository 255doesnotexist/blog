---
title: 第一章：应用程序与基本执行环境 · rCore 2024S 随记 
date: 2024-04-19
description: OSComp 2024 随记 · 一
---

# 前言

实际实践参考的文档：[rCore Tutorial Guide 2024S](https://learningos.cn/rCore-Tutorial-Guide-2024S/)。

拥有更为详细细节介绍的 rCore OS 官方文档：[rCore-Tutorial-Book-v3](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/0intro.html)。

# 第一章：应用程序与基本执行环境

## 引言

本章预期代码树：

```bash
├── bootloader (内核依赖的运行在 M 特权级的 SBI 实现，本项目中我们使用 RustSBI)
│   └── rustsbi-qemu.bin
├── os
│   ├── Cargo.toml (cargo 项目配置文件)
│   ├── Makefile
│   └── src
│       ├── console.rs (将打印字符的 SBI 接口进一步封装实现更加强大的格式化输出)
│       ├── entry.asm (设置内核执行环境的的一段汇编代码)
│       ├── lang_items.rs (需要我们提供给 Rust 编译器的一些语义项，目前包含内核 panic 时的处理逻辑)
│       ├── linker.ld (控制内核内存布局的链接脚本以使内核运行在 qemu 虚拟机上)
│       ├── logging.rs (为本项目实现了日志功能)
│       ├── main.rs (内核主函数)
│       └── sbi.rs (封装底层 SBI 实现提供的 SBI 接口)
└── rust-toolchain (整个项目的工具链版本)

cloc os
-------------------------------------------------------------------------------
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                             5             25              6            155
make                             1             11              4             34
Assembly                         1              1              0             11
TOML                             1              2              1              7
-------------------------------------------------------------------------------
SUM:                             8             39             11            207
-------------------------------------------------------------------------------
```

## 应用程序执行环境与平台支持

### 执行应用程序

按文档要求，我们切回主分支 main。执行命令。

```bash
cargo new os
```

```bash
$ tree
.
├── os
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── README.md

3 directories, 4 files
```

```bash
$ cd os/
$ cargo run
   Compiling os v0.1.0 (/home/ezra/rCore-Tutorial-Code-2024S/os)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/os`
Hello, world!
```

> 我们在屏幕上看到了一行 Hello, world! ，但为了打印出 Hello, world!，我们需要的不止几行源代码。

### 理解应用程序执行环境

![](https://learningos.cn/rCore-Tutorial-Guide-2024S/_images/app-software-stack.png)

> 我们的应用程序通过调用标准库或第三方库提供的接口，仅需少量源代码就能完成复杂的功能； Hello, world! 程序调用的 println! 宏就是由 Rust 标准库 std 和 GNU Libc 等提供的。 这些库属于应用程序的 执行环境 (Execution Environment)，而它们的实现又依赖于操作系统提供的系统调用。

- 所以我们需要剥离标准库，才能在裸机上执行。操作系统则提供关于标准库的系统调用。

### 平台与目标三元组

> 编译器在编译、链接得到可执行文件时需要知道，程序要在哪个 平台 (Platform) 上运行， 目标三元组 (Target Triplet) 描述了目标平台的 CPU 指令集、操作系统类型和标准运行时库。

> 我们研究一下现在 Hello, world! 程序的目标三元组是什么：

- 在我的环境下：

```bash
$ rustc --verbose --version
rustc 1.77.2 (25ef9e3d8 2024-04-09)
binary: rustc
commit-hash: 25ef9e3d85d934b27d9dada2f9dd52b1dc63bb04
commit-date: 2024-04-09
host: x86_64-unknown-linux-gnu
release: 1.77.2
LLVM version: 17.0.6
```

- 也就是 x86_64 (amd64)、Linux、GNU glibc。

> 接下来，我们希望把 Hello, world! 移植到 RICV 目标平台 riscv64gc-unknown-none-elf 上运行。

- 注意，接下来没有标准库可供调用。

### 修改目标平台

> 将程序的目标平台换成 riscv64gc-unknown-none-elf，试试看会发生什么：

```bash
$ cargo run --target riscv64gc-unknown-none-elf
   Compiling os v0.1.0 (/home/ezra/rCore-Tutorial-Code-2024S/os)
error[E0463]: can't find crate for `std`
  |
  = note: the `riscv64gc-unknown-none-elf` target may not be installed
  = help: consider downloading the target with `rustup target add riscv64gc-unknown-none-elf`

error: cannot find macro `println` in this scope
 --> src/main.rs:2:5
  |
2 |     println!("Hello, world!");
  |     ^^^^^^^

error: requires `sized` lang_item

For more information about this error, try `rustc --explain E0463`.
error: could not compile `os` (bin "os") due to 3 previous errors
```

> 报错的原因是目标平台上确实没有 Rust 标准库 std，也不存在任何受 OS 支持的系统调用。 这样的平台被我们称为 裸机平台 (bare-metal)。

> 幸运的是，除了 std 之外，Rust 还有一个不需要任何操作系统支持的核心库 core， 它包含了 Rust 语言相当一部分核心机制，可以满足本门课程的需求。 有很多第三方库也不依赖标准库 std，而仅仅依赖核心库 core。

> 为了以裸机平台为目标编译程序，我们要将对标准库 std 的引用换成核心库 core。

## 移除标准库依赖

- 编辑 os/.cargo/config 文件，加入以下内容：

```conf
[build]
target = "riscv64gc-unknown-none-elf"
```

- 这将会持久化设定目标平台的设定为 riscv64gc。

> 这将使 cargo 工具在 os 目录下默认会使用 riscv64gc-unknown-none-elf 作为目标平台。 这种编译器运行的平台（x86_64）与可执行文件运行的目标平台不同的情况，称为 交叉编译 (Cross Compile)。

### 移除 println! 宏

> 我们在 main.rs 的开头加上一行 #![no_std]， 告诉 Rust 编译器不使用 Rust 标准库 std 转而使用核心库 core。重新编译，报错如下：

```bash
$ cargo run
   Compiling os v0.1.0 (/home/ezra/rCore-Tutorial-Code-2024S/os)
error[E0463]: can't find crate for `core`
  |
  = note: the `riscv64gc-unknown-none-elf` target may not be installed
  = help: consider downloading the target with `rustup target add riscv64gc-unknown-none-elf`

error[E0463]: can't find crate for `compiler_builtins`

error: cannot find macro `println` in this scope
 --> src/main.rs:3:5
  |
3 |     println!("Hello, world!");
  |     ^^^^^^^

error: requires `sized` lang_item

For more information about this error, try `rustc --explain E0463`.
error: could not compile `os` (bin "os") due to 4 previous errors
```

- 注意到环境配置似乎有些问题，没有安装 riscv64gc-unknown-none-elf 的 target。解决方案如下：

```bash
$ rustup target add riscv64gc-unknown-none-elf
```

> println! 宏是由标准库 std 提供的，且会使用到一个名为 write 的系统调用。 无论如何，我们先将这行代码注释掉。

### 提供语义项 panic_handler

> 标准库 std 提供了 Rust 错误处理函数 #[panic_handler]，其大致功能是打印出错位置和原因并杀死当前应用。 但核心库 core 并没有提供这项功能，得靠我们自己实现。

- 例子：移除了 println! 后，依然存在以下错误。

```bash
$ cargo run
   Compiling os v0.1.0 (/home/ezra/rCore-Tutorial-Code-2024S/os)
error: `#[panic_handler]` function required, but not found

error: could not compile `os` (bin "os") due to 1 previous error
```

> 新建一个子模块 os/src/lang_items.rs，在里面编写 panic 处理函数，通过标记 #[panic_handler] 告知编译器采用我们的实现：

```rust
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```