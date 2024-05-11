---
title: 第四章：地址空间 · rCore 2024S 随记 
date: 2024-05-11
description: OSComp 2024 随记 · 四
---

- 唉。他妈的排课，都没什么时间做 rCore 了。我现在学校的课程从早 8 排到晚 22。简直是高中。

# 前言

实际实践参考的文档：[rCore Tutorial Guide 2024S](https://learningos.cn/rCore-Tutorial-Guide-2024S/)。

拥有更为详细细节介绍的 rCore OS 官方文档：[rCore-Tutorial-Book-v3](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/0intro.html)。

# 第四章：地址空间

## 引言

试着运行本章代码。

```bash
$ git checkout ch4
Already on 'ch4'
Your branch is up to date with 'origin/ch4'.
~/2024s-rcore-255doesnotexist$ cd os
~/2024s-rcore-255doesnotexist/os$ make run
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
     Removed 1342 files, 15.6MiB total
target/riscv64gc-unknown-none-elf/release/ch2b_bad_address target/riscv64gc-unknown-none-elf/release/ch2b_bad_instructions target/riscv64gc-unknown-none-elf/release/ch2b_bad_register target/riscv64gc-unknown-none-elf/release/ch2b_hello_world target/riscv64gc-unknown-none-elf/release/ch2b_power_3 target/riscv64gc-unknown-none-elf/release/ch2b_power_5 target/riscv64gc-unknown-none-elf/release/ch2b_power_7 target/riscv64gc-unknown-none-elf/release/ch3b_yield0 target/riscv64gc-unknown-none-elf/release/ch3b_yield1 target/riscv64gc-unknown-none-elf/release/ch3b_yield2 target/riscv64gc-unknown-none-elf/release/ch4b_sbrk
   Compiling scopeguard v1.2.0
   Compiling spin v0.5.2
   Compiling spin v0.7.1
   Compiling bitflags v1.3.2
   Compiling lock_api v0.4.6
   Compiling lazy_static v1.4.0
   Compiling buddy_system_allocator v0.6.0
   Compiling spin v0.9.8
   Compiling user_lib v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/user)
warning: creating a mutable reference to mutable static is discouraged
  --> src/bin/ch8b_race_adder.rs:18:17
   |
18 |         let a = &mut A as *mut usize;
   |                 ^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see issue #114447 <https://github.com/rust-lang/rust/issues/114447>
   = note: this will be a hard error in the 2024 edition
   = note: this mutable reference has lifetime `'static`, but if the static gets accessed (read or written) by any other means, or any other reference is created, then any further use of this mutable reference is Undefined Behavior
   = note: `#[warn(static_mut_refs)]` on by default
help: use `addr_of_mut!` instead to create a raw pointer
   |
18 |         let a = addr_of_mut!(A) as *mut usize;
   |                 ~~~~~~~~~~~~~~~

warning: unused import: `get_time`
  --> src/bin/ch7b_pipe_large_test.rs:10:29
   |
10 | use user_lib::{close, fork, get_time, pipe, read, wait, write,getpid};
   |                             ^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: creating a mutable reference to mutable static is discouraged
  --> src/bin/ch8b_race_adder_atomic.rs:26:17
   |
26 |         let a = &mut A as *mut usize;
   |                 ^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see issue #114447 <https://github.com/rust-lang/rust/issues/114447>
   = note: this will be a hard error in the 2024 edition
   = note: this mutable reference has lifetime `'static`, but if the static gets accessed (read or written) by any other means, or any other reference is created, then any further use of this mutable reference is Undefined Behavior
   = note: `#[warn(static_mut_refs)]` on by default
help: use `addr_of_mut!` instead to create a raw pointer
   |
26 |         let a = addr_of_mut!(A) as *mut usize;
   |                 ~~~~~~~~~~~~~~~

warning: `user_lib` (bin "ch8b_race_adder") generated 1 warning
warning: creating a mutable reference to mutable static is discouraged
  --> src/bin/ch8b_race_adder_mutex_spin.rs:20:17
   |
20 |         let a = &mut A as *mut usize;
   |                 ^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see issue #114447 <https://github.com/rust-lang/rust/issues/114447>
   = note: this will be a hard error in the 2024 edition
   = note: this mutable reference has lifetime `'static`, but if the static gets accessed (read or written) by any other means, or any other reference is created, then any further use of this mutable reference is Undefined Behavior
   = note: `#[warn(static_mut_refs)]` on by default
help: use `addr_of_mut!` instead to create a raw pointer
   |
20 |         let a = addr_of_mut!(A) as *mut usize;
   |                 ~~~~~~~~~~~~~~~

warning: Rust ABI is unsupported in naked functions
   --> src/bin/ch8b_stackful_coroutine.rs:259:1
    |
259 | unsafe fn switch(old: *mut TaskContext, new: *const TaskContext) {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(undefined_naked_function_abi)]` on by default

warning: creating a mutable reference to mutable static is discouraged
  --> src/bin/ch8b_race_adder_loop.rs:24:17
   |
24 |         let a = &mut A as *mut usize;
   |                 ^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see issue #114447 <https://github.com/rust-lang/rust/issues/114447>
   = note: this will be a hard error in the 2024 edition
   = note: this mutable reference has lifetime `'static`, but if the static gets accessed (read or written) by any other means, or any other reference is created, then any further use of this mutable reference is Undefined Behavior
   = note: `#[warn(static_mut_refs)]` on by default
help: use `addr_of_mut!` instead to create a raw pointer
   |
24 |         let a = addr_of_mut!(A) as *mut usize;
   |                 ~~~~~~~~~~~~~~~

warning: field `id` is never read
  --> src/bin/ch8b_stackful_coroutine.rs:36:5
   |
35 | struct Task {
   |        ---- field in this struct
36 |     id: usize,
   |     ^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `user_lib` (bin "ch8b_race_adder_atomic") generated 1 warning
warning: `user_lib` (bin "ch7b_pipe_large_test") generated 1 warning (run `cargo fix --bin "ch7b_pipe_large_test"` to apply 1 suggestion)
warning: `user_lib` (bin "ch8b_race_adder_mutex_spin") generated 1 warning
warning: `user_lib` (bin "ch8b_stackful_coroutine") generated 2 warnings
warning: `user_lib` (bin "ch8b_race_adder_loop") generated 1 warning
    Finished `release` profile [optimized] target(s) in 2.28s
make[1]: Leaving directory '/home/ezra/2024s-rcore-255doesnotexist/user'
Platform: qemu
   Compiling os v0.1.0 (/home/ezra/2024s-rcore-255doesnotexist/os)
   Compiling zero v0.1.3
   Compiling xmas-elf v0.7.0
    Finished `release` profile [optimized] target(s) in 1.25s
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
[kernel] back to world!
remap_test passed!
init TASK_MANAGER
num_app = 11
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x3ac, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
Hello, world from user mode program!
power_3 [10000/200000]
power_3 [20000/200000]
power_3 [30000/200000]
power_3 [40000/200000]
power_3 [50000/200000]
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
AAAAAAAAAA [1/5]
BBBBBBBBBB [1/5]
CCCCCCCCCC [1/5]
Test sbrk start.
origin break point = c000
one page allocated,  break point = d000
try write to allocated page
write ok
10 page allocated,  break point = 17000
11 page DEALLOCATED,  break point = c000
try DEALLOCATED more one page, should be failed.
Test sbrk almost OK!
now write to deallocated page, should cause page fault.
[kernel] PageFault in application, bad addr = 0xc000, bad instruction = 0x5c8, kernel killed it.
power_3 [60000/200000]
power_3 [70000/200000]
power_3 [80000/200000]
power_3 [90000/200000]
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
AAAAAAAAAA [2/5]
BBBBBBBBBB [2/5]
CCCCCCCCCC [2/5]
power_3 [100000/200000]
power_3 [110000/200000]
power_3 [120000/200000]
power_3 [130000/200000]
power_3 [140000/200000]
power_3 [150000/200000]
power_3 [160000/200000]
power_3 [170000/200000]
power_3 [180000/200000]
AAAAAAAAAA [3/5]
BBBBBBBBBB [3/5]
CCCCCCCCCC [3/5]
power_3 [190000/200000]
power_3 [200000/200000]
3^200000 = 871008973(MOD 998244353)
Test power_3 OK!
AAAAAAAAAA [4/5]
BBBBBBBBBB [4/5]
CCCCCCCCCC [4/5]
AAAAAAAAAA [5/5]
BBBBBBBBBB [5/5]
CCCCCCCCCC [5/5]
Test write A OK!
Test write B OK!
Test write C OK!
[kernel] Panicked at src/task/mod.rs:153 All applications completed!
```