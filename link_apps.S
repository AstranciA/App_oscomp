// this file is auto generated by acbat
.align 8
    .section .data
    .global _num_app
_num_app:
    .quad 2
    .quad app_00fe506a1a_start
    .quad app_017525ea0b_start
    .quad app_017525ea0b_end

    .section .data
    .global app_00fe506a1a_start
    .global app_00fe506a1a_end
app_00fe506a1a_start:
    .incbin "testcases/nimbos/build/riscv64/hello_world"
app_00fe506a1a_end:

    .section .data
    .global app_017525ea0b_start
    .global app_017525ea0b_end
app_017525ea0b_start:
    .incbin "testcases/nimbos/build/riscv64/getpid"
app_017525ea0b_end:
