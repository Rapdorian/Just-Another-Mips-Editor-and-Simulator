la $a0, first
li $v0, 4
syscall
li $v0, 5
syscall
move $t0, $v0


la $a0, second
li $v0, 4
syscall
li $v0, 5
syscall
move $t1, $v0

la $a0, result
li $v0, 4
syscall

li $v0, 1

slt $a0, $t0, $t1
syscall

li $v0, 10
syscall

first:
.ascii "Enter first number :\n\0"
second:
.ascii "Enter second number:\n\0"
result:
.ascii "Result of a < b: \0"
