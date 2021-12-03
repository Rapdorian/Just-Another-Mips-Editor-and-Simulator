.data
msg: 
.asciiz "Hello There\n"

.text
la $a0, msg
li $v0, 4
syscall