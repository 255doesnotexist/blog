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

