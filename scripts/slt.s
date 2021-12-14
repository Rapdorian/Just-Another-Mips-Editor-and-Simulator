li $t0, 4
li $t1, 5

li $v0, 1

slt $a0, $t0, $t1
syscall
slt $a0, $t1, $t0
syscall

li $v0, 10
syscall
