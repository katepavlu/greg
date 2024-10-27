.data
    constant: .word 125
    buffer: .space 8
    display: .addr 0xffff0000

.text
    la $t1, display
    la $t2, constant
    la $s1, buffer
    addi $t0, $zero, 7 #initialize t0
    loop_start:
        beq $t0, $zero, loop_end
            addi $t0, $t0, -1
            sw $t0, $t1
        beq $zero, $zero, loop_start
    loop_end:
    add $zero, $zero, $zero
    ja $s0, loop_beginning