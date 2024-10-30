.data
    constant: .word 125
.text
    addi $1, $0, 1
    addi $2, $0, 2
    addi $3, $0, 3

    and $4, $2, $1
    or $4, $2, $1
    xor $4, $2, $1
    not $4, $2

    addi $4, $4, 1 # reg 4 should not conatain -4
    addi $3, $4, 4 # reg 3 should be 0