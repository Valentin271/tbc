# Show the disassembly of the generated program
# This is necessary because objdump cannot figure out where is the entrypoint
dis:
	objdump -b binary --start-address 0xb0 -m i386:x86-64 -D dump.elf

# Compile generated assembly with nasm to prove it works
asm: dump.asm
	nasm -f elf64 dump.asm
	\ld dump.o -o asm.out

# volontarily omit optimizations
c: data/count.c
	gcc data/count.c

clean:
	rm -rf \
		dump.* \
		udump.* \
		ast.dot* \
		ost.dot* \
		parse_tree.dot* \
		*.o \
		*.out
