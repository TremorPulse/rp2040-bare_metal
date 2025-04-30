PROJECT = main

SRCDIR = src
BOOT2DIR = boot
BUILDDIR = build
BUILDBOOT2DIR = $(BUILDDIR)/$(BOOT2DIR)
TOOLSDIR = tools

BOOT2 = bootStage2
COMPCRC = crc32
CRCVALUE = crc

LNKSCRIPT = rp2040.ld

SRCS = $(wildcard $(SRCDIR)/*.c)
OBJS = $(SRCS:$(SRCDIR)/%.c=$(BUILDDIR)/%.o)

TOOLCHAIN = arm-none-eabi-
GCC = $(TOOLCHAIN)gcc
LNK = $(TOOLCHAIN)ld
DMP = $(TOOLCHAIN)objdump
CPY = $(TOOLCHAIN)objcopy

HOST_GPP = g++

GCCFLAGS ?= -mcpu=cortex-m0plus -g -I$(TOOLSDIR)
LNKFLAGS ?= -T $(LNKSCRIPT) -nostdlib 

HOST_GPPFLAGS ?= -I$(TOOLSDIR) -std=c++11

ifeq ($(OS),Windows_NT)
    RM = rmdir /s /q
    MKDIR = mkdir
    CP = copy
    HOST_GPP = g++
else
    RM = rm -rf
    MKDIR = mkdir -p
    CP = cp
    HOST_GPP = g++
endif

.PHONY: all clean setup build

all: setup build

build: makeDir $(BUILDBOOT2DIR)/$(BOOT2).elf $(BUILDBOOT2DIR)/$(CRCVALUE).c $(BUILDDIR)/$(PROJECT).elf $(BUILDDIR)/$(PROJECT).uf2 copyUF2

makeDir:
	$(MKDIR) "$(BUILDDIR)"
	$(MKDIR) "$(BUILDBOOT2DIR)"

$(BUILDBOOT2DIR)/$(BOOT2).elf: $(BOOT2DIR)/$(BOOT2).c
	$(GCC) $< $(GCCFLAGS) $(LNKFLAGS) -o $@
	$(DMP) -hSD $@ > $(BUILDBOOT2DIR)/$(BOOT2).objdump

$(BUILDDIR)/%.o: $(SRCDIR)/%.c
	$(GCC) $(GCCFLAGS) -c $< -o $@

$(BUILDBOOT2DIR)/$(CRCVALUE).c: $(BUILDBOOT2DIR)/$(BOOT2).elf $(BOOT2DIR)/$(COMPCRC).cpp
	$(CPY) -O binary $(BUILDBOOT2DIR)/$(BOOT2).elf $(BUILDBOOT2DIR)/$(BOOT2).bin
	$(HOST_GPP) $(HOST_GPPFLAGS) $(BOOT2DIR)/$(COMPCRC).cpp -o $(BUILDBOOT2DIR)/$(COMPCRC).exe
	$(BUILDBOOT2DIR)/$(COMPCRC).exe $(BUILDBOOT2DIR)/$(BOOT2).bin

$(BUILDDIR)/$(PROJECT).elf: $(OBJS) $(BOOT2DIR)/$(BOOT2).c $(BUILDBOOT2DIR)/$(CRCVALUE).c $(LNKSCRIPT)
	$(GCC) $(OBJS) $(BOOT2DIR)/$(BOOT2).c $(BUILDBOOT2DIR)/$(CRCVALUE).c $(GCCFLAGS) $(LNKFLAGS) -o $@
	$(DMP) -hSD $@ > $(BUILDDIR)/$(PROJECT).objdump

$(BUILDDIR)/$(PROJECT).uf2: $(BUILDDIR)/$(PROJECT).elf
	$(CPY) -O binary $< $(BUILDDIR)/$(PROJECT).bin
	python3 $(TOOLSDIR)/uf2/utils/uf2conv.py -b 0x10000000 -f 0xe48bff56 -c $(BUILDDIR)/$(PROJECT).bin -o $@

copyUF2: $(BUILDDIR)/$(PROJECT).uf2
	$(CP) "$<" "$(PROJECT).uf2"

clean:
	$(RM) "$(BUILDDIR)"
	$(RM) "$(PROJECT).uf2"