.data
    constant2: .word 125
    buffer2: .space 8
    display2: .addr 0xffff0004

.text
    la $t1, display2
    la $t2, constant2
    la $s1, buffer2

    addi $t0, $zero, 7 #initialize t0
    loop_2_start:
        beq $t0, $zero, loop_2_end
            addi $t0, $t0, -1
            sw $t0, $t1
        beq $zero, $zero, loop_2_start
    loop_2_end:
    add $zero, $zero, $zero
    ja $s0, loop_1_start