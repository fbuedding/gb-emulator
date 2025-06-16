# Define the assembler, linker, and fixer commands
ASM = rgbasm
LINK = rgblink
FIX = rgbfix

# Define the output extensions
OBJ_EXT = .o
GB_EXT = .gb

# Define the source and object file patterns
SRCS = $(wildcard *.asm)
OBJS = $(SRCS:%.asm=%.o)
GBS = $(SRCS:%.asm=%.gb)

# Default target
all: $(GBS)

# Pattern rule to compile .asm files to .o files
%.o: %.asm
	$(ASM) -o $@ $<

# Pattern rule to link .o files to .gb files
%.gb: %.o
	$(LINK) -o $@ $<
	$(FIX) -v $@

# Clean target to remove generated files
clean:
	rm -f $(OBJS) $(GBS)

.PHONY: all clean
