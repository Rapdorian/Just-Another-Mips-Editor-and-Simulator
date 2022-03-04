la $a0, prompt
li $v0, 4
syscall

li $v0, 5
syscall

add $t0 $v0, $v0

la $a0, disp
li $v0, 4
syscall

move $a0, $t0
li $v0, 1
syscall

li $v0, 10
syscall

.data
prompt:
.asciiz "Enter number to double:\n"
disp:
.asciiz "Result:\n"
