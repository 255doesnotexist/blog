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

## 多道程序放置和加载

### 多道程序放置

> 在第二章中，内核让所有应用都共享同一个固定的起始地址。 正因如此，内存中同时最多只能驻留一个应用，

> 要一次加载运行多个程序，就要求每个用户程序被内核加载到内存中的起始地址都不同。 为此，我们编写脚本 user/build.py 为每个应用定制各自的起始地址。 它的思路很简单，对于每一个应用程序，使用 cargo rustc 单独编译， 用 -Clink-args=-Ttext=xxxx 选项指定链接时 .text 段的地址为 0x80400000 + app_id * 0x20000 。

> qemu 预留的内存空间是有限的，如果加载的程序过多，程序地址超出内存空间，可能出现 core dumped.

> 与第二章相同，所有应用的 ELF 格式执行文件都经过 objcopy 工具丢掉所有 ELF header 和符号变为二进制镜像文件，随后以同样的格式通过在操作系统内核中嵌入 link_user.S 文件，在编译时直接把应用链接到内核的数据段中。不同的是，我们对相关模块进行了调整：在第二章中应用的加载和执行进度控制都交给 batch 子模块，而在第三章中我们将应用的加载这部分功能分离出来在 loader 子模块中实现，应用的执行和切换功能则交给 task 子模块。

> 注意，我们需要调整每个应用被构建时使用的链接脚本 linker.ld 中的起始地址 BASE_ADDRESS ，这个地址是应用被内核加载到内存中的起始地址。也就是要做到：应用知道自己会被加载到某个地址运行，而内核也确实能做到将应用加载到它指定的那个地址。这算是应用和内核在某种意义上达成的一种协议。之所以要有这么苛刻的条件，是因为目前的操作系统内核的能力还是比较弱的，对应用程序通用性的支持也不够（比如不支持加载应用到内存中的任意地址运行），这也进一步导致了应用程序编程上不够方便和通用（应用需要指定自己运行的内存地址）。事实上，目前应用程序的编址方式是基于绝对位置的，并没做到与位置无关，内核也没有提供相应的地址重定位机制。

- 目前使用如下脚本进行链接器定制。

```python
 # user/build.py

 import os

 base_address = 0x80400000
 step = 0x20000
 linker = 'src/linker.ld'

 app_id = 0
 apps = os.listdir('src/bin')
 apps.sort()
 for app in apps:
     app = app[:app.find('.')]
     lines = []
     lines_before = []
     with open(linker, 'r') as f: # 找到 BASE_ADDRESS 那行，替换成我们需要的
         for line in f.readlines():
             lines_before.append(line)
             line = line.replace(hex(base_address), hex(base_address+step*app_id))
             lines.append(line)
     with open(linker, 'w+') as f:
         f.writelines(lines)
     os.system('cargo build --bin %s --release' % app) # 构建当前应用
     print('[build.py] application %s start with address %s' %(app, hex(base_address+step*app_id)))
     with open(linker, 'w+') as f: # 把 linker 脚本恢复原样
         f.writelines(lines_before)
     app_id = app_id + 1
```

- 还真有够脏的。但能用。

### 多道程序加载

> 在第二章中负责应用加载和执行的子模块 batch 被拆分为 loader 和 task ， 前者负责启动时加载应用程序，后者负责切换和调度。

> 其中， loader 模块的 load_apps 函数负责将所有用户程序在内核初始化的时一并加载进内存。

```rust
pub fn load_apps() {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    // clear i-cache first
    unsafe {
        asm!("fence.i");
    } // 还记得吗，上节说过实机不清理会出现奇奇怪怪的问题，但qemu不一定会 

    /*
    i-cache（指令缓存）是用于存储指令的高速缓存。它的目的是提高指令的访问速度，从而加快程序的执行。

    在加载程序之前执行 i-cache 清理的原因是确保之前执行的指令不会对新加载的程序产生影响。在处理器的流水线中，指令是按照一定的顺序进行执行的。如果之前的指令被缓存到 i-cache 中，而这些指令与新加载的程序有相关性，可能会导致错误的执行结果。

    通过执行 fence.i 指令，可以确保在加载新程序之前，所有之前的指令都已经完成，并且对 i-cache 中的指令进行了清理。fence.i 是一种屏障指令，它会强制处理器在执行后续指令之前，等待之前的所有指令都完成。

    清理 i-cache 的目的是为了确保加载的新程序在一个干净的执行环境中开始执行，从而避免由于之前的指令对新程序造成的不正确的影响。这样可以增加程序的可靠性和可预测性，并且减少由于指令缓存引起的错误。
    */

    // load apps
    for i in 0..num_app {
        let base_i = get_base_i(i);
        // clear region
        (base_i..base_i + APP_SIZE_LIMIT)
            .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });
        // load app from data section to memory
        let src = unsafe {
            core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
        };
        let dst = unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };
        dst.copy_from_slice(src);
    }
}
```

```rust
/// Get base address of app i.
fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}
```

> 我们可以在 config 子模块中找到 APP_BASE_ADDRESS、APP_SIZE_LIMIT 这两个常数， APP_BASE_ADDRESS 被设置为 0x80400000 ， 而 APP_SIZE_LIMIT 和上一章一样被设置为 0x20000 。这种放置方式与 user/build.py 的实现一致。

### 执行应用程序

> 当多道程序的初始化放置工作完成，或者是某个应用程序运行结束或出错的时候，我们要调用 run_next_app 函数切换到下一个应用程序。此时 CPU 运行在 S 特权级的操作系统中，而操作系统希望能够切换到 U 特权级去运行应用程序。这一过程与上章的 执行应用程序 一节的描述类似。相对不同的是，操作系统知道每个应用程序预先加载在内存中的位置，这就需要设置应用程序返回的不同 Trap 上下文（Trap 上下文中保存了 放置程序起始地址的 epc 寄存器内容）：

> - 跳转到应用程序（编号 i）的入口点 entry

> - 将使用的栈切换到用户栈 

> 我们的“锯齿螈”初级多道程序操作系统就算是实现完毕了。它支持把多个应用的代码和数据放置到内存中，并能够依次执行每个应用，提高了应用切换的效率，这就达到了本章对操作系统的初级需求。但“锯齿螈”操作系统在任务调度的灵活性上还有很大的改进空间，下一节我们将开始改进这方面的问题。

- 多道程序的操作系统目前还是只能一个个执行任务，除非它提前完成或中途出错。
- 很是期待分时多任务操作系统部分。

## 任务切换

- 这一节的任务是实现《任务切换》的核心功能 ```__switch()```

### 本节导读

- 看导读的意思，本节的重点是实现内核的 ```__switch``` 函数。
- 本节引入了任务、任务切换、任务上下文的概念。
- 切换任务最初被提出，主要是因为当时的人们希望程序访问别的外设或睡眠时，其他程序能够继续执行。

### 任务的概念形成

- 255 你能不能写点代码别搁这里看文档了我看得焦虑。
- 应用程序的一次执行过程是一个任务。执行过程中的一个片是计算任务片。
- 在任务片间切换需要保存和恢复上下文。以便同步不同程序变化的资源（寄存器、栈等）。

### 不同类型的上下文与切换

> - 第一章“应用程序与基本执行环境”中，我们介绍了 函数调用与栈 。当时提到过，为了支持嵌套函数调用，不仅需要硬件平台提供特殊的跳转指令，还需要保存和恢复 函数调用上下文 。注意在上述定义中，函数调用包含在普通控制流（与异常控制流相对）之内，且始终用一个固定的栈来保存执行的历史记录，因此函数调用并不涉及控制流的特权级切换。但是我们依然可以将其看成调用者和被调用者两个执行过程的“切换”，二者的协作体现在它们都遵循调用规范，分别保存一部分通用寄存器，这样的好处是编译器能够有足够的信息来尽可能减少需要保存的寄存器的数目。虽然当时用了很大的篇幅来说明，但其实整个过程都是编译器负责完成的，我们只需设置好栈就行了。

> - 第二章“批处理系统”中第一次涉及到了某种异常（Trap）控制流，即两条控制流的特权级切换，需要保存和恢复 系统调用（Trap）上下文 。当时，为了让内核能够 完全掌控 应用的执行，且不会被应用破坏整个系统，我们必须利用硬件提供的特权级机制，让应用和内核运行在不同的特权级。应用运行在 U 特权级，它所被允许的操作进一步受限，处处被内核监督管理；而内核运行在 S 特权级，有能力处理应用执行过程中提出的请求或遇到的状况。

- 两者的区别是有无涉及不同控制流的特权级控制。

- 硬件提供的 Trap 机制是 APP 与 OS 打交道的核心。Trap 对 APP 透明，因此需要自己处理全部的上下文保存、恢复机制。

### 任务切换的设计与实现

- 任务切换主要针对先前针对不同应用程序定义的任务片进行片间切换，因此不涉及特权级切换、对应用也是透明的（APP 感受不到它的存在）。

- Trap 控制流间可以通过 ```__switch``` 函数实现的换栈操作切换到不同的控制流。

- ```__switch``` 基本上四步走：

1. 调用 ```__switch``` 前，A 任务上下文、B 任务上下文不变，当前 SP 在 A 任务上下文上。

2. A 任务保存 CPU 当前必要的寄存器快照到 A 任务的上下文空间。

3. 读取下一任务指针（指向任务 B！）指向的任务上下文，根据它复原 CPU 寄存器。

4. 把 SP 栈换到 B 的内核栈上。完成控制流切换。

- 以下给出 rCore 的 switch 实现。

```asm
# os/src/task/switch.S

.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .globl __switch
__switch:
    # 阶段 [1]
    # __switch(
    #     current_task_cx_ptr: *mut TaskContext,
    #     next_task_cx_ptr: *const TaskContext
    # )
    # 阶段 [2]
    # save kernel stack of current task
    sd sp, 8(a0)
    # save ra & s0~s11 of current execution
    sd ra, 0(a0)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n + 1
    .endr
    # 阶段 [3]
    # restore ra & s0~s11 of next execution
    ld ra, 0(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr
    # restore kernel stack of next task
    ld sp, 8(a1)
    # 阶段 [4]
    ret
```

- 值得一提的是，在 Rust 代码的定义里，我们可以实际看到 TaskContent 保存了哪些寄存器。

```rust
// os/src/task/context.rs

pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}
```

- rCore 的文档真的很详细。它告诉我们，ra 寄存器保存了 __switch 返回后应该跳转到哪里继续执行。 s0~s11 这些被调用者保存的寄存器也有必要手动保存，因为 __switch 是一个汇编手工编写的函数，不包含在 Rust 编译器自动保存的范围里。

> 我们会将这段汇编代码中的全局符号 __switch 解释为一个 Rust 函数：

```rust
// os/src/task/switch.rs

global_asm!(include_str!("switch.S"));

use super::TaskContext;

extern "C" {
    pub fn __switch(
        current_task_cx_ptr: *mut TaskContext,
        next_task_cx_ptr: *const TaskContext
    );
}
```

> 我们会调用该函数来完成切换功能而不是直接跳转到符号 __switch 的地址。因此在调用前后 Rust 编译器会自动帮助我们插入保存/恢复调用者保存寄存器的汇编代码。

- TaskContext 很像是一个普通函数栈帧的内容喔。但是它会进行换栈，以便在不同的控制流之间切来切去。

- 有一道思考题：当内核仅运行单个应用的时候，无论该任务主动/被动交出 CPU 资源最终都会交还给自己，这将导致传给 __switch 的两个参数相同，也就是某个 Trap 控制流自己切换到自己的情形。

- 我觉得应该问题不大，只是会多一层保存寄存器到自身 TaskContext，然后再从自身 TaskContext 加载的开销吧。

- Collaborator wyfcyx 的评论很准确：

> TrapContext中保存的寄存器记录了应用陷入S特权级之前的CPU状态，而TaskContext则可以看成一个应用在S特权级进行Trap处理的过程中调用__switch之前的CPU状态。当恢复TaskContext之后会继续进行Trap处理，而__restore恢复TrapContext之后则是会回到用户态执行应用。
> 另外，保存TrapContext之后进行Trap处理的时候，s0-s11寄存器可能会被覆盖，后面进行任务切换时这些寄存器会被保存到TaskContext中，也就是说这两个Context中的s0-s11也很可能是不同的。

> 第二章为了简单起见，每个task的栈空间仅会被使用一次，对应的task退出之后就会永久闲置。等到第五章引入了进程之后，可以看到在进程退出之后，它的栈空间所在的物理内存会被回收并可以供其他进程使用。