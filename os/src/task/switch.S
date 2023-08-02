.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .globl __switch
__switch:
    # 阶段[1]
    # __switch(
    #    current_task_cx_ptr: *mut TaskContext
    #    next_task_cx_ptr: *const TaskContext
    # )

    # 阶段[2]
    # 将sp, ra, s0~s11寄存器中内容保存到内核栈上
    sd sp, 8(a0)
    sd ra, 0(a0)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr

    # 阶段[3]
    # 从内核栈上恢复ra, s0~s11, sp寄存器
    ld ra, 0(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n+1
    .endr
    ld sp, 8(a1)

    # 阶段[4]
    ret