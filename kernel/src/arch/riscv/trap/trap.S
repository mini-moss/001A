.text

.balign 4
.global trap_entry
trap_entry:
    # -----------------------
    # 1. 分配 TrapFrame 空间
    # -----------------------
    addi sp, sp, -34 * 8
    mv t0, sp

    # -----------------------
    # 2. 保存通用寄存器
    # -----------------------
    sd x1,  1*8(t0)
    sd x3,  3*8(t0)
    sd x4,  4*8(t0)
    sd x5,  5*8(t0)
    sd x6,  6*8(t0)
    sd x7,  7*8(t0)
    sd x8,  8*8(t0)
    sd x9,  9*8(t0)
    sd x10, 10*8(t0)
    sd x11, 11*8(t0)
    sd x12, 12*8(t0)
    sd x13, 13*8(t0)
    sd x14, 14*8(t0)
    sd x15, 15*8(t0)
    sd x16, 16*8(t0)
    sd x17, 17*8(t0)
    sd x18, 18*8(t0)
    sd x19, 19*8(t0)
    sd x20, 20*8(t0)
    sd x21, 21*8(t0)
    sd x22, 22*8(t0)
    sd x23, 23*8(t0)
    sd x24, 24*8(t0)
    sd x25, 25*8(t0)
    sd x26, 26*8(t0)
    sd x27, 27*8(t0)
    sd x28, 28*8(t0)
    sd x29, 29*8(t0)
    sd x30, 30*8(t0)
    sd x31, 31*8(t0)

    # -----------------------
    # 3. 保存 sstatus, sepc
    # -----------------------
    csrr t1, sstatus
    csrr t2, sepc
    sd t1, 32*8(t0)
    sd t2, 33*8(t0)

    # -----------------------
    # 4. 调用 Rust trap handler
    # -----------------------
    mv a0, t0            # 第一个参数 = TrapFrame*
    call trap_handler

    # -----------------------
    # 5. 恢复寄存器
    # -----------------------
    ld x1,  1*8(t0)
    ld x3,  3*8(t0)
    ld x4,  4*8(t0)
    ld x5,  5*8(t0)
    ld x6,  6*8(t0)
    ld x7,  7*8(t0)
    ld x8,  8*8(t0)
    ld x9,  9*8(t0)
    ld x10, 10*8(t0)
    ld x11, 11*8(t0)
    ld x12, 12*8(t0)
    ld x13, 13*8(t0)
    ld x14, 14*8(t0)
    ld x15, 15*8(t0)
    ld x16, 16*8(t0)
    ld x17, 17*8(t0)
    ld x18, 18*8(t0)
    ld x19, 19*8(t0)
    ld x20, 20*8(t0)
    ld x21, 21*8(t0)
    ld x22, 22*8(t0)
    ld x23, 23*8(t0)
    ld x24, 24*8(t0)
    ld x25, 25*8(t0)
    ld x26, 26*8(t0)
    ld x27, 27*8(t0)
    ld x28, 28*8(t0)
    ld x29, 29*8(t0)
    ld x30, 30*8(t0)
    ld x31, 31*8(t0)

    ld t1, 32*8(t0)
    ld t2, 33*8(t0)
    csrw sstatus, t1
    csrw sepc, t2

    addi sp, sp, 34*8      # 恢复栈
    sret