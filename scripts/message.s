#------- Data Segment ----------
    .data
# declare several strings just to demonstrate how strings and values are stored into memory
mesg:   .asciiz "\tAvengers Assemble!!\n"
mesg2:  .ascii  "Foo"                       # .ascii vs .asciiz  
mesg3:  .ascii  "bars"
num:    .word   0xBad
num2:   .word   0x111
num3:   .byte   1 2 3

    .text
main:
         
    # Execute the "exit" system call
    li $v0, 10
    syscall