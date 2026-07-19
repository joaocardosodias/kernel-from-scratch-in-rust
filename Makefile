BOOT_DIR    := boot
KERNEL_DIR  := kernel
BUILD_DIR   := build

STAGE1_SRC  := $(BOOT_DIR)/stage1.asm
STAGE2_SRC  := $(BOOT_DIR)/stage2.asm
STAGE1_BIN  := $(BUILD_DIR)/stage1.bin
STAGE2_BIN  := $(BUILD_DIR)/stage2.bin
STAGE2_PADDED := $(BUILD_DIR)/stage2-padded.bin
KERNEL_BIN  := $(BUILD_DIR)/kernel.bin
DISK_IMG    := $(BUILD_DIR)/disk.img

QEMU        := qemu-system-x86_64
QEMU_FLAGS  := -drive format=raw,file=$(DISK_IMG) -m 64 -display sdl -vga std -accel kvm -accel tcg -machine pcspk-audiodev=audio0 -audiodev pa,id=audio0

KERNEL_SRC  := $(shell find $(KERNEL_DIR)/src -type f) $(KERNEL_DIR)/Cargo.toml

.PHONY: all clean run boot kernel

all: $(DISK_IMG)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(STAGE1_BIN): $(STAGE1_SRC) | $(BUILD_DIR)
	nasm -f bin $< -o $@

$(STAGE2_BIN): $(STAGE2_SRC) | $(BUILD_DIR)
	nasm -f bin $< -o $@

$(STAGE2_PADDED): $(STAGE2_BIN)
	cp $< $@
	truncate -s 1024 $@

boot: $(STAGE1_BIN) $(STAGE2_BIN)

$(KERNEL_BIN): $(KERNEL_SRC) | $(BUILD_DIR)
	cd $(KERNEL_DIR) && cargo build --release
	objcopy -O binary $(KERNEL_DIR)/target/x86_64-unknown-none/release/kernel $@

kernel: $(KERNEL_BIN)

$(DISK_IMG): $(STAGE1_BIN) $(STAGE2_PADDED) $(KERNEL_BIN)
	cat $(STAGE1_BIN) $(STAGE2_PADDED) $(KERNEL_BIN) > $@
	truncate -s 2M $@

run: $(DISK_IMG)
	SDL_AUDIODRIVER=dummy $(QEMU) $(QEMU_FLAGS)

clean:
	rm -rf $(BUILD_DIR)
	cd $(KERNEL_DIR) && cargo clean
