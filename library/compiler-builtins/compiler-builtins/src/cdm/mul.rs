use core::intrinsics;

intrinsics! {
    #[unsafe(naked)]
    #[cfg(not(feature = "no-asm"))]
    pub unsafe extern "custom" fn __mulhi3() {
        core::arch::naked_asm!(
            "ldi r2, 0",
            "tst r1",
            "bz 2f",
            "0:",
            "shr r1",
            "bcc 1f",
            "add r0, r2",
            "1:",
            "shl r0",
            "tst r1",
            "bnz 0b",
            "2:",
            "move r2, r0",
            "rts",
        );
    }

    #[unsafe(naked)]
    #[cfg(not(feature = "no-asm"))]
    pub unsafe extern "custom" fn __mulsi3() {
        core::arch::naked_asm!(
            "push r4",
            "push r5",
            "push r6",
            "ldi r4, 0",
            "ldi r5, 0",
            "or r2, r3, r6",
            "bz 2f",
            "0:",
            "shr r3",
            "rcr r2",
            "bcc 1f",
            "add r0, r4, r4",
            "addc r1, r5, r5",
            "1:",
            "shl r0",
            "rcl r1",
            "or r2, r3, r6",
            "bnz 0b",
            "2:",
            "move r4, r0",
            "move r5, r1",
            "pop r6",
            "pop r5",
            "pop r4",
            "rts",
        )
    }
}
