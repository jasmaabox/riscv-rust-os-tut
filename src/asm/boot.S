# boot.S
# bootloader for SoS
# Stephen Marz
# 8 February 2019
.option norvc
.section .data

.section .text.init
.global _start
_start:
	# Any hardware threads (hart) that are not bootstrapping
	# need to wait for an IPI
	csrr	t0, mhartid
	bnez	t0, 3f
	# SATP should be zero, but let's make sure
	csrw	satp, zero
.option push
.option norelax
	la		gp, _global_pointer
.option pop
	# The BSS section is expected to be zero
	la 		a0, _bss_start
	la		a1, _bss_end
	bgeu	a0, a1, 2f
1:
	sd		zero, (a0)
	addi	a0, a0, 8
	bltu	a0, a1, 1b
2:	
	# Control registers, set the stack, mstatus, mepc,
	# and mtvec to return to the main function.
	la		sp, _stack
	# We use mret here so that the mstatus register
	# is properly updated.

	# Turn on machine mode
	li		t0, (0b11 << 11) | (1 << 7) | (1 << 3)
	csrw	mstatus, t0
	# Set machine execution program counter to rust entrypoint
	la		t1, kmain
	csrw	mepc, t1
	# Set machine trap vector to trap vectory entrypoint
	la		t2, asm_trap_vector
	csrw	mtvec, t2
	# Turn on machine software, timer, and external interrupts
	li		t3, (1 << 3) | (1 << 7) | (1 << 11)
	csrw	mie, t3
	# Park thread after rust code finishes
	la		ra, 4f
	# Boot into rust
	mret

# Parked threads go here
3:

4:
	wfi
	j	4b