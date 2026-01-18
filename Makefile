# Makefile for mini-moss/001A RISC-V kernel

# Configuration
TARGET = riscv64gc-unknown-none-elf
KERNEL_ELF = target/$(TARGET)/debug/kernel
KERNEL_BIN = target/$(TARGET)/debug/kernel.bin
LINKER_SCRIPT = kernel/arch/riscv/linker.ld
OUTPUT_ASM = kernel.S

# Default target
.PHONY: all
all: build

# Build kernel and convert to binary
.PHONY: build
build:
	@echo "Building kernel..."
	cargo clean
	RUSTFLAGS="-Clink-arg=-T$(LINKER_SCRIPT) -Cforce-frame-pointers=yes" \
		cargo build -Z build-std=core,alloc --target $(TARGET) --manifest-path kernel/Cargo.toml
	rust-objcopy --binary-architecture=riscv64 $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)
	@echo "Kernel built: $(KERNEL_BIN)"

# Run in QEMU
.PHONY: qemu
qemu: build
	@echo "Running in QEMU..."
	qemu-system-riscv64 -M 128m -machine virt -nographic -bios default -kernel $(KERNEL_BIN)

# Generate disassembly
.PHONY: objdump
objdump: build
	@echo "Generating disassembly..."
	riscv64-unknown-elf-objdump -d $(KERNEL_ELF) > $(OUTPUT_ASM)
	@echo "Disassembly saved to: $(OUTPUT_ASM)"

# Debug server (QEMU with GDB)
.PHONY: gdbserver
gdbserver: build
	@echo "Starting QEMU with GDB server..."
	qemu-system-riscv64 -M 128m -machine virt -nographic -bios default -kernel $(KERNEL_BIN) -s -S

# Debug client
.PHNY: gdbclient
gdbclient:
	@echo "Starting GDB client..."
	gdb -ex "file $(KERNEL_ELF)" -ex "set arch riscv:rv64" -ex "target remote localhost:1234"

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# Help
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build      - Build kernel binary"
	@echo "  qemu     - Run kernel in QEMU"
	@echo "  objdump  - Generate disassembly"
	@echo "  gdbserver- Start QEMU with GDB server"
	@echo "  gdbclient- Connect GDB to QEMU"
	@echo "  clean    - Clean build artifacts"
	@echo "  help     - Show this help"