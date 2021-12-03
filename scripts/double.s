.data
prompt: .asciiz "Enter a number to double: "
disp: .asciiz "Result: "

.text
la $a0, prompt
li $v0, 4
syscall

li $v0, 5
syscall

add $t0, $v0, $v0

la $a0, disp
li $v0, 4
syscall

move $a0, $t0
li $v0, 1
syscall

li $a0, 0xA # print newline
li $v0, 11
syscall