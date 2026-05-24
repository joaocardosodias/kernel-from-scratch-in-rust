BOOT_DIR    := boot
KERNEL_DIR  := kernel
BUILD_DIR   := build

STAGE1_SRC  := $(BOOT_DIR)/stage1.asm
STAGE2_SRC  := $(BOOT_DIR)/stage2.asm
STAGE1_BIN  := $(BUILD_DIR)/stage1.bin
STAGE2_BIN  := $(BUILD_DIR)/stage2.bin
KERNEL_BIN  := $(BUILD_DIR)/kernel.bin
DISK_IMG    := $(BUILD_DIR)/disk.img

QEMU        := qemu-system-x86_64
QEMU_FLAGS  := -drive format=raw,file=$(DISK_IMG) -m 64 -display gtk,gl=off

.PHONY: all clean run boot kernel

all: $(DISK_IMG)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(STAGE1_BIN): $(STAGE1_SRC) | $(BUILD_DIR)
	nasm -f bin $< -o $@

$(STAGE2_BIN): $(STAGE2_SRC) | $(BUILD_DIR)
	nasm -f bin $< -o $@

boot: $(STAGE1_BIN) $(STAGE2_BIN)

$(KERNEL_BIN): | $(BUILD_DIR)
	cd $(KERNEL_DIR) && cargo build --release
	cp $(KERNEL_DIR)/target/x86_64-unknown-none/release/kernel $@

kernel: $(KERNEL_BIN)

$(DISK_IMG): $(STAGE1_BIN) $(STAGE2_BIN)
	cat $(STAGE1_BIN) $(STAGE2_BIN) > $@
	truncate -s %512 $@

run: $(DISK_IMG)
	$(QEMU) $(QEMU_FLAGS)

clean:
	rm -rf $(BUILD_DIR)
	cd $(KERNEL_DIR) && cargo clean
