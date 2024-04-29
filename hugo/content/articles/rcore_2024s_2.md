---
title: 第二章：批处理系统 · rCore 2024S 随记 
date: 2024-04-29
description: OSComp 2024 随记 · 二
---

- 由于 ch2 已经被实现好了，本部分基本为摘抄。

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

> 用户库看起来很复杂，它预留了直到 ch7 内核才能实现的系统调用接口，console 模块还实现了输出缓存区。它们不是为本章准备的，你只需关注本节提到的部分即可。

## 应用程序设计

- 弱链接实现检测 main。
 
> 我们在 lib.rs 中看到了另一个 main ：

```bash
#![feature(linkage)]    // 启用弱链接特性

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}
```

> 我们使用 Rust 宏将其标志为弱链接。这样在最后链接的时候， 虽然 lib.rs 和 bin 目录下的某个应用程序中都有 main 符号， 但由于 lib.rs 中的 main 符号是弱链接， 链接器会使用 bin 目录下的函数作为 main 。 如果在 bin 目录下找不到任何 main ，那么编译也能通过，但会在运行时报错。

### 内存布局

> 我们使用链接脚本 user/src/linker.ld 规定用户程序的内存布局：

> - 将程序的起始物理地址调整为 0x80400000 ，三个应用程序都会被加载到这个物理地址上运行；

- 实际查看文件时地址是 0x0，其实是因为在文档后期该内容已经被修改了。

> - 将 _start 所在的 .text.entry 放在整个程序的开头 0x80400000； 批处理系统在加载应用后，跳转到 0x80400000，就进入了用户库的 _start 函数；

> - 提供了最终生成可执行文件的 .bss 段的起始和终止地址，方便 clear_bss 函数使用。

> 其余的部分和第一章基本相同。

### 系统调用

- 按照 RISC-V 调用规范实现了 sys_write 和 sys_exit 两个 syscall。
> 简而言之，这条汇编代码的执行结果是以寄存器 a0~a2 来保存系统调用的参数，以及寄存器 a7 保存 syscall ID， 返回值通过寄存器 a0 传递给局部变量 ret。

- 详情请查阅原版文档。

> 于是 sys_write 和 sys_exit 只需将 syscall 进行包装：

```rust
// user/src/syscall.rs

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len)])
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}
```

> 我们将上述两个系统调用在用户库 user_lib 中进一步封装，像标准库一样：

```rust
// user/src/lib.rs
use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }
```

> 在 console 子模块中，借助 write，我们为应用程序实现了 println! 宏。 传入到 write 的 fd 参数设置为 1，代表标准输出 STDOUT，暂时不用考虑其他的 fd 选取情况。

## 实现批处理操作系统

### 将应用程序链接到内核

> 在 os/src/main.rs 中能够找到这样一行：

```rust
core::arch::global_asm!(include_str!("link_app.S"));
```

> 这里我们引入了一段汇编代码 link_app.S ，它是在 make run 构建操作系统时自动生成的。

> 第 13 行开始的三个数据段分别插入了三个应用程序的二进制镜像， 并且各自有一对全局符号 ```app_*_start```, ```app_*_end``` 指示它们的开始和结束位置。 而第 3 行开始的另一个数据段相当于一个 64 位整数数组。 数组中的第一个元素表示应用程序的数量，后面则按照顺序放置每个应用程序的起始地址， 最后一个元素放置最后一个应用程序的结束位置。这样数组中相邻两个元素记录了每个应用程序的始末位置， 这个数组所在的位置由全局符号 _num_app 所指示。

> 这个文件是在 cargo build 时，由脚本 os/build.rs 控制生成的。

### 找到并加载应用程序二进制码

> 我们在 os 的 batch 子模块中实现一个应用管理器 AppManager ，结构体定义如下：

```rust
struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}
```

- load_app 的实现基本就是根据预先得到的应用程序地址把它加载为生切片，然后从生切片拷贝到目标地址（通常是大家约定的常数地址 0x80400000，取决于设置的 APP_BASE_ADDRESS）

> 清空内存前，我们插入了一条奇怪的汇编指令 fence.i ，它是用来清理 i-cache 的。 我们知道， 缓存又分成 数据缓存 (d-cache) 和 指令缓存 (i-cache) 两部分，分别在 CPU 访存和取指的时候使用。 通常情况下， CPU 会认为程序的代码段不会发生变化，因此 i-cache 是一种只读缓存。 但在这里，我们会修改会被 CPU 取指的内存区域，使得 i-cache 中含有与内存不一致的内容， 必须使用 fence.i 指令手动清空 i-cache ，让里面所有的内容全部失效， 才能够保证程序执行正确性。

> qemu 可能不需要清空 i-cache 也能正常运行，但实机上就不是这样了。
> 
> batch 子模块对外暴露出如下接口：

> - init ：调用 print_app_info 的时第一次用到了全局变量 APP_MANAGER ，它在这时完成初始化；

> - run_next_app ：批处理操作系统的核心操作，即加载并运行下一个应用程序。 批处理操作系统完成初始化，或者应用程序运行结束/出错后会调用该函数。下节再介绍其具体实现。

## 实现特权级的切换

### RISC-V特权级切换

#### 特权级切换的起因

> 批处理操作系统为了建立好应用程序的执行环境，需要在执行应用程序前进行一些初始化工作， 并监控应用程序的执行，具体体现在：

> 启动应用程序时，需要初始化应用程序的用户态上下文，并能切换到用户态执行应用程序；

> 应用程序发起系统调用后，需要切换到批处理操作系统中进行处理；

> 应用程序执行出错时，批处理操作系统要杀死该应用并加载运行下一个应用；

> 应用程序执行结束时，批处理操作系统要加载运行下一个应用。

> 这些处理都涉及到特权级切换，因此都需要硬件和操作系统协同提供的特权级切换机制。

#### 特权级切换相关的控制状态寄存器

|CSR 名|该 CSR 与 Trap 相关的功能|
|------|------------------------|
|sstatus|SPP 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息|
|sepc|当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址|
|scause|描述 Trap 的原因|
|stval|给出 Trap 附加信息|
|stvec|控制 Trap 处理代码的入口地址|

#### 用户栈与内核栈

> 在 Trap 触发的一瞬间， CPU 会切换到 S 特权级并跳转到 stvec 所指示的位置。 但是在正式进入 S 特权级的 Trap 处理之前，我们必须保存原控制流的寄存器状态，这一般通过栈来完成。 但我们需要用专门为操作系统准备的内核栈，而不是应用程序运行时用到的用户栈。

```rust
const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};
```

> 两个栈以全局变量的形式实例化在批处理操作系统的 .bss 段中。

> 我们为两个类型实现了 get_sp 方法来获取栈顶地址。由于在 RISC-V 中栈是向下增长的， 我们只需返回包裹的数组的结尾地址，以用户栈类型 UserStack 为例：

```rust
impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
```

- 其实就是取指针 + 栈顶偏置直接返回。哎呀这样居然是 safe 的真是太神奇了。
换栈是非常简单的，只需将 sp 寄存器的值修改为 get_sp 的返回值即可。

> 接下来是 Trap 上下文，即在 Trap 发生时需要保存的物理资源内容，定义如下：

```rust
use riscv::register::sstatus::{self, Sstatus, SPP};
/// Trap Context
#[repr(C)]
pub struct TrapContext {
    /// general regs[0..31]
    pub x: [usize; 32],
    /// CSR sstatus      
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
}

impl TrapContext {
    /// set stack pointer to x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read(); // CSR sstatus
        sstatus.set_spp(SPP::User); //previous privilege mode: user mode
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // entry point of app
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
}
```

> 可以看到里面包含所有的通用寄存器 x0~x31 ，还有 sstatus 和 sepc 。

> 对于通用寄存器而言，两条控制流（应用程序控制流和内核控制流）运行在不同的特权级，所属的软件也可能由不同的编程语言编写，虽然在 Trap 控制流中只是会执行 Trap 处理 相关的代码，但依然可能直接或间接调用很多模块，因此很难甚至不可能找出哪些寄存器无需保存。既然如此我们就只能全部保存了。但这里也有一些例外， 如 x0 被硬编码为 0 ，它自然不会有变化；还有 tp(x4) 寄存器，除非我们手动出于一些特殊用途使用它，否则一般也不会被用到。虽然它们无需保存， 但我们仍然在 TrapContext 中为它们预留空间，主要是为了后续的实现方便。

> 对于 CSR 而言，我们知道进入 Trap 的时候，硬件会立即覆盖掉 scause/stval/sstatus/sepc 的全部或是其中一部分。scause/stval 的情况是：它总是在 Trap 处理的第一时间就被使用或者是在其他地方保存下来了，因此它没有被修改并造成不良影响的风险。 而对于 sstatus/sepc 而言，它们会在 Trap 处理的全程有意义（在 Trap 控制流最后 sret 的时候还用到了它们），而且确实会出现 Trap 嵌套的情况使得它们的值被覆盖掉。所以我们需要将它们也一起保存下来，并在 sret 之前恢复原样。

### Trap 管理

#### Trap 上下文的保存与恢复

- 细节请阅读原文档。

- 首先修改 stvec 寄存器指向正确的 Trap 处理入口点。

- 利用 extern "C" 引入外部符号 __alltraps、__restore，将 stvec.write 以 Direct 模式指向它的地址。标记为函数。通过 global_asm! 宏将 trap.S 这段汇编代码插入进来。
- 其中上下文的保存与恢复使用外部汇编代码 __alltraps、__restore，而具体处理部分使用 Rust 编写的函数 trap_handler完成。

- 以下是两个外部汇编函数

```asm
.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text
    .globl __alltraps
    .globl __restore
    .align 2 # 四字节对齐，RISC-V 特权级规范的要求
__alltraps:
    csrrw sp, sscratch, sp # csrrw rd, csr, rs 可以将 CSR 当前的值读到通用寄存器 rd 中，然后将通用寄存器的值写入 CSR。因此这里的作用是交换 是scratch 和 sp。相当于把内核栈交换到实际的 sp 栈顶寄存器上，用户栈暂存到待交换位置。
    # now sp->kernel stack, sscratch->user stack
    # allocate a TrapContext on kernel stack
    addi sp, sp, -34*8 # 预先分配 34*8 字节栈帧。
    # save general-purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it on the kernel stack
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # set input argument of trap_handler(cx: &mut TrapContext)
    mv a0, sp
    call trap_handler

__restore:
    # case1: start running app by __restore
    # case2: back to U after handling trap
    mv sp, a0
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # release TrapContext on kernel stack
    addi sp, sp, 34*8
    # now sp->kernel stack, sscratch->user stack
    csrrw sp, sscratch, sp
    sret
```

- 作用在注释里讲的很清楚，基本是按一定的逻辑顺序备份上下文，然后调用 trap_handler。等调用结束后再按原样恢复现场。

- CSR 指令是原子的，很神奇吧！

- 然后是 trap_handler。

```rust
#[no_mangle]
/// handle an interrupt, exception, or system call from user space
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    match scause.cause() { // 根据 scause 寄存器所保存的 Trap 的原因进行分发处理。这里我们无需手动操作这些 CSR ，而是使用 Rust 第三方库 riscv 。
        Trap::Exception(Exception::UserEnvCall) => { // 系统调用出错，因此尝试让 sepc 在恢复后指向 ecall 指令后的下一条指令
            cx.sepc += 4; // 4 是 ecall 指令的码长
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize; // 尝试调用系统调用并返回结果（并不实际处理系统调用，只是根据 syscall ID 分发到具体的处理函数）
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app(); // 页面错误，下一个应用
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app(); // 非法访存，下一个应用
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            ); // 无法处理的 Trap
        }
    }
    cx
}
```
