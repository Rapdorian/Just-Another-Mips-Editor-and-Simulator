li $t0, 0
li $t1, 100
loop:
	li $v0, 1
	move $a0, $t0
	syscall
	li $v0, 11
	li $a0, 0xA
	syscall
	
	addi $t0, $t0, 1
	
	beq $t0, $t1, done
    nop
j loop
nop
done:
li $v0, 10
syscall
