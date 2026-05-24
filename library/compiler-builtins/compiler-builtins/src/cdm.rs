//! CDM-specific 16-bit and 32-bit mul and div implementations

use core::intrinsics;

#[unsafe(naked)]
unsafe extern "custom" fn udiv_mod_16_impl() {
    //! Computes unsigned quotient and remainder of r0 / r1
    //! Places the quotient in r0 and the remainder in r2
    core::arch::naked_asm!(
        "cmp r1, 0",
        "bnz 0f",
        "zero",
        "0:",

        "ldi r2, 0",
        "ldi r3, 16",

        "1:",
        "shl r0",
        "rcl r2",
        "cmp r2, r1",
        "blo 2f",
        "sub r2, r1, r2",
        "inc r0",
        "2:",
        "dec r3",
        "bnz 1b",

        "rts",
    )
}

#[unsafe(naked)]
unsafe extern "custom" fn udiv_mod_32_impl() {
    //! Computes unsigned quotient and remainder of r0:r1 / r2:r3
    //! Places the quotient in r0:r1 and the remainder in r2:r3
    core::arch::naked_asm!(
        "cmp r2, 0",
        "bnz 0f",
        "cmp r3, 0",
        "bnz 0f",
        "zero",
        "0:",

        "push r4",
        "push r5",
        "push r6",
        "ldi r4, 0",
        "ldi r5, 0",
        "ldi r6, 32",

        "1:",
        "shl r0",
        "rcl r1",
        "rcl r4",
        "rcl r5",
        "cmp r5, r3",
        "blo 2f",
        "bhi 3f",
        "cmp r4, r2",
        "blo 2f",
        "3:",
        "sub r4, r2, r4",
        "subc r5, r3, r5",
        "inc r0",
        "2:",
        "dec r6",
        "bnz 1b",

        "move r4, r2",
        "move r5, r3",
        "pop r6",
        "pop r5",
        "pop r4",
        "rts",
    )
}


intrinsics! {
    #[unsafe(naked)]
    pub unsafe extern "custom" fn __adddi3() {
        //! Adds two 64-bit integers
        //! CDM-specific
        core::arch::naked_asm!(
            "push fp",
            "ldsp fp",
            "addsp -2",
            "ssw r5, -2",
            "lsw r5, 12",
            "add r0, r5, r0",
            "lsw r5, 14",
            "addc r1, r5, r1",
            "lsw r5, 16",
            "addc r2, r5, r2",
            "lsw r5, 18",
            "addc r3, r5, r3",
            "lsw r5, -2",
            "stsp fp",
            "pop fp",
            "rts",
        );
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __subdi3() {
        //! Subtracts the second 64-bit
        //! integer from the first one
        //! CDM-specific
        core::arch::naked_asm!(
            "push fp",
            "ldsp fp",
            "addsp -2",
            "ssw r5, -2",
            "lsw r5, 12",
            "sub r0, r5, r0",
            "lsw r5, 14",
            "subc r1, r5, r1",
            "lsw r5, 16",
            "subc r2, r5, r2",
            "lsw r5, 18",
            "subc r3, r5, r3",
            "lsw r5, -2",
            "stsp fp",
            "pop fp",
            "rts",
        );
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __negdi2() {
        //! Computes 2's complement of a 64-bit integer
        //! CDM-specific
        core::arch::naked_asm!(
            "neg r0",
            "bcs 0f",
            "not r1",
            "not r2",
            "not r3",
            "rts",
            "0:",
            "neg r1",
            "bcs 1f",
            "not r2",
            "not r3",
            "rts",
            "1:",
            "neg r2",
            "bcs 2f",
            "not r3",
            "rts",
            "2:",
            "neg r3",
            "rts",
        );
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __mulhi3() {
        //! Computes the product of r0 and r1
        //! Places the result in r0
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
    pub unsafe extern "custom" fn __mulsi3() {
        //! Computes the product of r0:r1 and r2:r3
        //! Places the result in r0:r1
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

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __udivhi3() {
        //! Computes unsigned r0 / r1
        //! Places the result in r0
        core::arch::naked_asm!(
            "jsr {}",
            "rts",
            sym crate::cdm::udiv_mod_16_impl,
        )
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __umodhi3() {
        //! Computes unsigned r0 % r1
        //! Places the result in r0
        core::arch::naked_asm!(
            "jsr {}",
            "move r2, r0",
            "rts",
            sym crate::cdm::udiv_mod_16_impl,
        )
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __udivmodhi4() {
        //! Computes unsigned r0 / r1 and r0 % r1
        //! Places the quotient in r0 and
        //! writes the remainder to memory pointed to by r2
        //!
        //! ```
        //! extern "C" fn __udivmodhi4(a: u16, b: u16, rem: &mut u16) -> u16;
        //! ```
        core::arch::naked_asm!(
            "push r4",
            "move r2, r4",
            "jsr {}",
            "stw r4, r2",
            "pop r4",
            "rts",
            sym crate::cdm::udiv_mod_16_impl,
        )
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __divhi3() {
        //! Computes signed r0 / r1
        //! Places the quotient in r0
        core::arch::naked_asm!(
            "push r4",
            "ldi r4, 0",

            "cmp r0, 0",
            "bpl 0f",
            "neg r0",
            "dec r4",
            "0:",

            "cmp r1, 0",
            "bpl 0f",
            "neg r1",
            "inc r4",
            "0:",

            "jsr {}",
            "cmp r4, 0",
            "bz 1f",
            "neg r0",
            "1:",
            "pop r4",
            "rts",
            sym crate::cdm::udiv_mod_16_impl,
        )
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __modhi3() {
        //! Computes signed r0 % r1
        //! Places the remainder in r0
        core::arch::naked_asm!(
            "push r4",
            "ldi r4, 0",

            "cmp r0, 0",
            "bpl 0f",
            "neg r0",
            "dec r4",
            "0:",

            "cmp r1, 0",
            "bpl 0f",
            "neg r1",
            "0:",

            "jsr {}",
            "move r2, r0",
            "cmp r4, 0",
            "bz 1f",
            "neg r0",
            "1:",
            "pop r4",
            "rts",
            sym crate::cdm::udiv_mod_16_impl,
        )
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __divmodhi4() {
        //! Computes signed r0 / r1 and r0 % r1
        //! Places the quotient in r0 and
        //! writes the remainder to memory pointed to by r2
        //!
        //! ```
        //! extern "C" fn __divmodhi4(a: i16, b: i16, rem: &mut i16) -> i16;
        //! ```
        core::arch::naked_asm!(
            "push r4",
            "push r5",
            "push r6",
            "ldi r4, 0",
            "move r2, r6",

            "cmp r0, 0",
            "bpl 0f",
            "neg r0",
            "dec r4",
            "0:",
            "move r4, r5",

            "cmp r1, 0",
            "bpl 0f",
            "neg r1",
            "inc r4",
            "0:",

            "jsr {}",
            "cmp r4, 0",
            "bz 1f",
            "neg r0",
            "1:",

            "cmp r5, 0",
            "bz 1f",
            "neg r2",
            "1:",

            "stw r6, r2",

            "pop r6",
            "pop r5",
            "pop r4",
            "rts",
            sym crate::cdm::udiv_mod_16_impl,
        )
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __udivsi3() {
        //! Computes unsigned r0:r1 / r2:r3
        //! Places the quotient in r0:r1
        core::arch::naked_asm!(
            "jsr {}",
            "rts",
            sym crate::cdm::udiv_mod_32_impl,
        )
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __umodsi3() {
        //! Computes unsigned r0:r1 % r2:r3
        //! Places the remainder in r0:r1
        core::arch::naked_asm!(
            "jsr {}",
            "move r2, r0",
            "move r3, r1",
            "rts",
            sym crate::cdm::udiv_mod_32_impl,
        )
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __udivmodsi4() {
        //! Computes unsigned r0:r1 / r2:r3 and r0:r1 % r2:r3
        //! Places the quotient in r0:r1 and
        //! writes the remainder to memory pointed to by pointer
        //! on the stack at offset 12 from the frame pointer
        //!
        //! ```
        //! extern "C" fn __udivmodsi4(a: u32, b: u32, rem: &mut u32) -> u32;
        //! ```
        core::arch::naked_asm!(
            "jsr {}",
            "push r4",
            "lsw r4, 12",
            "stw r4, r2",
            "add r4, 2",
            "stw r4, r3",
            "pop r4",
            "rts",
            sym crate::cdm::udiv_mod_32_impl,
        )
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __divsi3() {
        //! Computes signed r0:r1 / r2:r3
        //! Places the quotient in r0:r1
        core::arch::naked_asm!(
            "push r4",
            "ldi r4, 0",

            "cmp r1, 0",
            "bpl 0f",
            "not r1",
            "neg r0",
            "bcc 2f",
            "inc r1",
            "2:",
            "dec r4",
            "0:",

            "cmp r3, 0",
            "bpl 0f",
            "not r3",
            "neg r2",
            "bcc 2f",
            "inc r3",
            "2:",
            "inc r4",
            "0:",

            "jsr {}",
            "cmp r4, 0",
            "bz 1f",
            "not r1",
            "neg r0",
            "bcc 1f",
            "inc r1",
            "1:",

            "pop r4",
            "rts",
            sym crate::cdm::udiv_mod_32_impl,
        )
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __modsi3() {
        //! Computes signed r0:r1 % r2:r3
        //! Places the remainder in r0:r1
        core::arch::naked_asm!(
            "push r4",
            "ldi r4, 0",

            "cmp r1, 0",
            "bpl 0f",
            "not r1",
            "neg r0",
            "bcc 2f",
            "inc r1",
            "2:",
            "dec r4",
            "0:",

            "cmp r3, 0",
            "bpl 0f",
            "not r3",
            "neg r2",
            "bcc 0f",
            "inc r3",
            "0:",

            "jsr {}",
            "move r2, r0",
            "move r3, r1",
            "cmp r4, 0",
            "bz 1f",
            "not r1",
            "neg r0",
            "bcc 1f",
            "inc r1",
            "1:",

            "pop r4",
            "rts",
            sym crate::cdm::udiv_mod_32_impl,
        )
    }

    #[unsafe(naked)]
    pub unsafe extern "custom" fn __divmodsi4() {
        //! Computes signed r0:r1 / r2:r3 and r0:r1 % r2:r3
        //! Places the quotient in r0:r1 and
        //! writes the remainder to memory pointed to by pointer
        //! on the stack at offset 12 from the frame pointer
        //!
        //! ```
        //! extern "C" fn __divmodsi4(a: i32, b: i32, rem: &mut i32) -> i32;
        //! ```
        core::arch::naked_asm!(
            "push r4",
            "push r5",
            "ldi r4, 0",

            "cmp r1, 0",
            "bpl 0f",
            "not r1",
            "neg r0",
            "bcc 2f",
            "inc r1",
            "2:",
            "dec r4",
            "0:",
            "move r4, r5",

            "cmp r3, 0",
            "bpl 0f",
            "not r3",
            "neg r2",
            "bcc 2f",
            "inc r3",
            "2:",
            "inc r4",
            "0:",

            "jsr {}",
            "cmp r4, 0",
            "bz 1f",
            "not r1",
            "neg r0",
            "bcc 1f",
            "inc r1",
            "1:",

            "cmp r5, 0",
            "bz 1f",
            "not r3",
            "neg r2",
            "bcc 1f",
            "inc r3",
            "1:",

            "lsw r4, 12",
            "stw r4, r2",
            "add r4, 2",
            "stw r4, r3",

            "pop r5",
            "pop r4",
            "rts",
            sym crate::cdm::udiv_mod_32_impl,
        )
    }
}
