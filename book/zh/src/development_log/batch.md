# 批处理系统

## 用户故事 1
> 作为开发者，我希望能够在裸机上运行一个最简内核入口点，以便启动系统。

### xtask实现
1. cargo bin 实现对内核的build，并转为裸机二进制代码

通过Command直接拼接指令，先实现需求再进行重构

```rust
//cargo build
let _ = Command::new("cargo")
    .env("RUSTFLAGS", "-Clink-arg=-Tarch/riscv/linker.ld -Cforce-frame-pointers=yes")
    // 设置环境变量，指定链接文件
    .arg("build")
    .args(["-Z", "build-std=core,alloc"])
    // 使用 unstable 功能 -Z build-std：告诉 cargo 用 nightly 编译标准库core,alloc：
    // 仅构建 core 和 alloc，因为裸机环境不支持完整的 std
    .args(["--target", "riscv64gc-unknown-none-elf"])
    // 设置目标平台为裸机 RISC-V 64：无操作系统、无标准库的环境
    .args(["--manifest-path", "kernel/Cargo.toml"])
    // 指定要构建的 Cargo.toml 路径
    .status();
    
// kernel -> kernel.bin
let _ = Command::new("rust-objcopy")
    .args([
        "--binary-architecture=riscv64",
        "target/riscv64gc-unknown-none-elf/debug/kernel",
    ])
    .args(["--strip-all", "-O", "binary"])
    .arg("target/riscv64gc-unknown-none-elf/debug/kernel.bin")
    .status();
```

2. cargo qemu 通过qemu调用

使用默认的bios为opensbi，并用-kernel引导内核

```rust
// 用qemu执行代码
let _ = Command::new("qemu-system-riscv64")
    .args(["-M", "128m"])
    .args(["-machine", "virt"])
    .args(["-nographic"])
    .args(["-bios", "default"])
    .args(["-kernel", "target/riscv64gc-unknown-none-elf/debug/kernel.bin"])
    .status();
```

3. cargo objdump通过反汇编可以查看生成的代码和地址

可以直接将汇编代码输出到一个文件中

```rust
let output_path = "kernel.S";
let output = Command::new("riscv64-unknown-elf-objdump")
    .args(["-d", "target/riscv64gc-unknown-none-elf/debug/kernel"])
    .output();
if let Ok(output) = output {
    if output.status.success() {
        let file = File::create(output_path);
        file?.write_all(output.stdout.as_slice())?;
    }
}
```

### linker.ld实现

代码从0x80200000开始

```
OUTPUT_ARCH(riscv)
ENTRY(_start)
SECTIONS
{
    . = 0x80200000;

    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    ...
```

### boot.S实现

一些前置流程后调用rust_main跳转到代码部分

先给将sp偏移到栈底之后开始调用入口函数

```
.section .text.entry
.global _start
_start:
    csrw sie, zero
    la sp, stacks_start
    li t0, 4096
    add sp, sp, t0
    tail rust_main

.section .bss.stack
.align 12
.global stacks_start
stacks_start:
    .skip 4096
```

### 函数的print实现

使用global_asm加载boot.S

```rust
global_asm!(include_str!("../../arch/riscv/boot.S"));
```

入口函数
```rust
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    print!("这是伟大的第一步");
    loop {}
}
```

print实现通过对UART串口进行输出调用， print!宏，参考 Rust std

```rust
fn uart_write_byte(byte: u8) {
    unsafe {
        const UART0: *mut u8 = 0x1000_0000 as *mut u8;
        core::ptr::write_volatile(UART0, byte);
    }
}
```

之后进行文件打包和在qemu中执行，能得到期望输出
> 实现commit: f524998ac3b7e57a62470f570cbf51f8bd61c251