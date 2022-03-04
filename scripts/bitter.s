# registers
# $v0-1: scratch
# $t0: main data we are operating on
# $t1: byte index we are working on
# $t2: if we are summing 1s or 0s
# $t3: final sum

.data
int_prompt: .asciiz "Please enter an integer > "
byte_prompt: .asciiz "Please enter Byte position (0 - 3) > "
bit_prompt: .asciiz "Please enter a bit to sum (0 or 1) > "
response_0: .asciiz "The number of "
response_1: .asciiz "'s in byte "
response_2: .asciiz " of "
response_3: .asciiz " is: "

.text
main:

    # prompt for integer
    li $v0, 4
    la $a0, int_prompt
    syscall

    li $v0, 5
    syscall
    move $t0, $v0

    # prompt for byte pos
    li $v0, 4
    la $a0, byte_prompt
    syscall

    li $v0, 5
    syscall
    move $t1, $v0

    # prompt for 1 or 0
    li $v0, 4
    la $a0, bit_prompt
    syscall

    li $v0, 5
    syscall
    move $t2, $v0

    # the number of
    li $v0, 4
    la $a0, response_0
    syscall

    li $v0, 1
    move $a0, $t2
    syscall

    # 's in byte 
    li $v0, 4
    la $a0, response_1
    syscall

    li $v0, 1
    move $a0, $t1
    syscall

    # of
    li $v0, 4
    la $a0, response_2
    syscall

    li $v0, 1
    move $a0, $t0
    syscall

    # is:
    li $v0, 4
    la $a0, response_3
    syscall

    # convert our byte index into a usable shift $v0 = $t1 << 3
    sll $v0, $t1, 3
    srlv $t0, $t0, $v0 # shift out data by our new index
    andi $t0, $t0, 0xFF # mask our data

    # fill a register with the lowest bit from $t2
    # this is the fastest way I can think of to do this
    # It could be done quicker with a 2 byte lookup table
    move $v0, $t2
    move $v1, $v0
    sll $v0, $v0, 1
    or $v0, $v0, $v1
    move $v1, $v0
    sll $v0, $v0, 2 
    or $v0, $v0, $v1
    move $v1, $v0
    sll $v0, $v0, 4
    or $v0, $v0, $v1

    # now determine for each bit if I am counting it
    # v0 = (t0 ^ v0) nor 0
    xor $v0, $t0, $v0
    nor $v0, $v0, $zero

    # now sum up all the ones
    andi $v1, $v0, 1
    add $t3, $t3, $v1

    srl $v0, $v0, 1
    andi $v1, $v0, 1
    add $t3, $t3, $v1

    srl $v0, $v0, 1
    andi $v1, $v0, 1
    add $t3, $t3, $v1

    srl $v0, $v0, 1
    andi $v1, $v0, 1
    add $t3, $t3, $v1

    srl $v0, $v0, 1
    andi $v1, $v0, 1
    add $t3, $t3, $v1

    srl $v0, $v0, 1
    andi $v1, $v0, 1
    add $t3, $t3, $v1

    srl $v0, $v0, 1
    andi $v1, $v0, 1
    add $t3, $t3, $v1

    srl $v0, $v0, 1
    andi $v1, $v0, 1
    add $t3, $t3, $v1

    # output result
    li $v0, 1
    move $a0, $t3
    syscall

    # newline
    li $v0, 11
    li $a0, 0xA
    syscall

    # exit
    li $v0, 10
    syscall
