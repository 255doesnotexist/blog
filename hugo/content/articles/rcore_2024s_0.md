---
title: 第零章：实验环境配置 · rCore 2024S 随记 
date: 2024-04-19
description: OSComp 2024 随记 · 零
---

# 前言

实际实践参考的文档：[rCore Tutorial Guide 2024S](https://learningos.cn/rCore-Tutorial-Guide-2024S/)。

拥有更为详细细节介绍的 rCore OS 官方文档：[rCore-Tutorial-Book-v3](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/0intro.html)。

# 第零章：实验环境配置

## OS 环境配置

OS环境 Ubuntu Server 22.04 amd64。

## Rust 开发环境配置

通过 rustup 安装的当前最新版工具链。

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Qemu 模拟器安装

参考文档，从源码安装 Qemu 7.0.0。

```bash
sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
              gawk build-essential bison flex texinfo gperf libtool patchutils bc \
              zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev git tmux python3
wget https://download.qemu.org/qemu-7.0.0.tar.xz
# 解压
tar xvJf qemu-7.0.0.tar.xz
# 编译安装并配置 RISC-V 支持
cd qemu-7.0.0
./configure --target-list=riscv64-softmmu,riscv64-linux-user
make -j$(nproc)
```

## 其他工具安装

手动编译了 riscv-gnu-toolchain ，由此使得 riscv64-unknown-elf-gdb 可用。

```bash
# 构建依赖 
sudo apt-get install autoconf automake autotools-dev curl python3 python3-pip libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev ninja-build git cmake libglib2.0-dev libslirp-dev

# 尝试编译安装
./configure --prefix=/opt/riscv
make linux

# 编辑 bash profile
echo "export PATH='/opt/riscv/bin:$PATH'" > ~/.bashrc
```

## 试运行 rCore-Tutorial

得到如下输出：

```bash
(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add riscv64gc-unknown-none-elf
riscv64gc-unknown-none-elf (installed)
cargo install cargo-binutils
    Updating crates.io index
     Ignored package `cargo-binutils v0.3.6` is already installed, use --force to override
rustup component add rust-src
info: component 'rust-src' is up to date
rustup component add llvm-tools-preview
info: component 'llvm-tools' for target 'x86_64-unknown-linux-gnu' is up to date
Platform: qemu
    Finished `release` profile [optimized + debuginfo] target(s) in 0.01s
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
[DEBUG] [kernel] .rodata [0x80202000, 0x80203000)
[ INFO] [kernel] .data [0x80203000, 0x80204000)
[ WARN] [kernel] boot_stack top=bottom=0x80214000, lower_bound=0x80204000
[ERROR] [kernel] .bss [0x80214000, 0x80215000)
```

## 习题试做

### 编程题

> 在你日常使用的操作系统环境中安装并配置好实验环境。简要说明你碰到的问题/困难和解决方法。

- 尝试在本机编译安装 riscv-gnu-toolchain 时提示缺少数个库，一一安装即可。编译命令： 先按源存储库 README.md 操作配置编译参数，然后执行 ```make linux```。最后在 bash profile 中将 /opt/riscv/bin 写入 PATH。

> 在Linux环境下编写一个会产生异常的应用程序，并简要解释操作系统的处理结果。

- 可以简单编写一个会导致 SIGFPE 的程序。 ```int main(){return 1/0;}```
- 操作系统会通过中断捕获这个错误，根据不同的配置，可能执行 core dump 或 suspend、kill 掉进程。

> 在Linux环境下编写一个可以睡眠5秒后打印出一个字符串，并把字符串内容存入一个文件中的应用程序A。(基于C或Rust语言)

```rust
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    thread::sleep(Duration::from_secs(5));

    let message = "Hello, World!";
    println!("{}", message);

    let mut file = File::create("output.txt").expect("Failed to create file");
    file.write_all(message.as_bytes()).expect("Failed to write to file");
}
```

- ~~它使用了标准库，可能没法在裸机执行。~~

> 在Linux环境下编写一个应用程序B，简要说明此程序能够体现操作系统的并发性、异步性、共享性和持久性。(基于C或Rust语言)

- 支持多线程：并发性。
- 使用了异步文件API：异步性。（有点扯XD）
- 共享性：通过 Mutex 管理线程间共享数据。
- 持久性：文件存储，特别特别持久。

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::fs::OpenOptions;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

fn main() {
    // create multiple threads and execute tasks asynchronously
    let thread_handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread {} started", i);
                thread::sleep(Duration::from_secs(i as u64));

                // access shared variable using a mutex
                let shared_data = Arc::new(Mutex::new(0));
                { // using lock here 
                    let mut data = shared_data.lock().unwrap();
                    *data += i;
                    println!("Thread {} updated shared data: {}", i, *data);
                }

                // asynchrony and persistence: write data to a file
                let file_path = format!("output_{}.txt", i);
                let mut file = File::create(file_path).await.unwrap();
                let message = format!("Hello from thread {}", i);
                file.write_all(message.as_bytes()).await.unwrap();

                println!("Thread {} finished", i);
            })
        })
        .collect();

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
```

> 注： 在类Linux环境下编写尝试用GDB等调试工具调试应用程序A，能够设置断点，单步执行，显示变量信息。

- cargo project 为目标文件加入符号很简单，只需要配置 ```Cargo.toml```。

```toml
[profile.dev]
debug = true
```

- rustc 直接编译则是在命令行的文件前加入 ```--debug``` 参数。

- 然后启动 gdb，执行 ```gdb <可执行文件名>``` 即可像 C++ 程序一样进行调试。

- b / breakpoint 断点， n / next 下一步， c / continue 跳到下一个断点， print / watch 监视变量。

- 对于应用程序 A，可以执行 b 10，然后 r。这样，gdb 应该在五秒后捕获断点。此时执行 n，将会看到 Hello World 的输出。

### 问答题

> 什么是操作系统？操作系统的主要目标是什么？

- 操作系统是介于底层硬件实现和上层用户软件之间的一层系统应用软件。其主要目标是提供稳定的ABI、syscall，以期服务上层应用程序。其中ABI作为二进制接口，与应用程序接口API不同。ABI设计时就以可被多种语言调用为目标。

> 面向服务器的操作系统与面向手机的操作系统在功能上有何异同？

- 面向服务器的操作系统需要保障其运行服务的稳定性、可用性，不能随意杀死进程、需要合理分配时间片资源，同时也需要支持更大的内存。面向手机的操作系统则强调用户界面而非后台服务的流畅性，例如iOS中，动画优先级是远大于后台服务的。为了保证流畅，手机操作系统会通过墓碑机制、Doze机制对后台应用进行休眠，这一点与服务器系统大不相同。

- 他们的共同点：现代的服务器操作系统和手机操作系统在基本的抽象接口设计上基本一致，进程、线程、异步的概念一致。同样支持内存分页（智慧运存「偷笑」）、多处理器、超线程（麒麟9000S）。

> 对于目前的手机或桌面操作系统而言，操作系统是否应该包括网络浏览器？请说明理由。

- 我认为，从传统的目光看，目前的手机、桌面操作系统均不包括网络浏览器。网络浏览器应当属于其上运行的一个用户级软件。它既无直接访问硬件中断的能力，也不管理硬件级内存、实现驱动。因此不算是操作系统。

- 但换个角度看，网络浏览器可以被抽象得看作是一台完整的 JavaScript Machine。在这台机器上运行的 DOM 渲染部分是它的输出、捕获的鼠标键盘重定向是它的输入、V8 引擎是它的处理器，符合基本冯诺依曼机器的定义。V8 引擎中管理内存、线程的部分，实质上与操作系统相关的代码及其类似。如果近似地看，网络浏览器和操作系统的差别也许并不大呢！

> 操作系统的核心抽象有哪些？它们应对的对象是啥？

- 核心抽象：进程、线程、地址空间、文件、虚拟内存、设备。进程是对运行着的应用程序的抽象；线程是对应用程序中更小的任务片的抽象；地址空间是用来隔离应用程序使用内存的一种保护手段；虚拟内存是通过映射手段保证应用程序无感访问操作系统管理的各类介质的内存空间的一种方式；设备是操作系统管理的外围设备，通常通过中断或轮询与操作系统沟通。

> 操作系统与应用程序之间通过什么来进行互操作和数据交换？

- ABI，应用程序二进制接口。

> 操作系统的特征是什么？请结合你日常使用的操作系统的具体运行情况来进一步说明操作系统的特征。

- 操作系统（OS）是一个软件，它帮助用户和应用程序使用和管理计算机的资源。操作系统可能对最终用户不可见，但它控制着嵌入式设备、更通用的系统（如智能手机、台式计算机和服务器）以及巨型机等各种计算机系统。

> 请说明基于C语言应用的执行环境与基于Java语言应用的执行环境的异同。

- 同：C 语言应用需要各类 libc 用作执行库、Java 需要 JVM 作为执行环境。
- 异：C 直接编译到目标平台的机器码，Java 编译到 Universal Bytecode。

> 请简要列举操作系统的系统调用的作用，以及简要说明与程序执行、内存分配、文件读写相关的Linux系统调用的大致接口和含义。

- 30 个 syscall 的作用可见于 [这一章节的内容](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/2os-interface.html#id2) 。
- 我的意思是，既然可以查文档，就没必要把它背下来。

> 以你编写的可以睡眠5秒后打印出一个字符串的应用程序A为例，说明什么是控制流？什么是异常控制流？什么是进程、地址空间和文件？并简要描述操作系统是如何支持这个应用程序完成其工作并结束的。

- 还没写，明天再说。

> 请简要描述支持单个应用的OS、批处理OS、多道程序OS、分时共享OS的特点。

- 单个应用的 OS 只能运行单个应用便结束。批处理 OS 运行完一个程序后会按照既定安排运行下一个程序。多道程序 OS 可以以指令为分界线切换运行多个程序。分时共享 OS 可以以时间片为单位进行上下文切换运行多个程序。