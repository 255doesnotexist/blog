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