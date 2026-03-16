use core::arch::asm;
use core::ffi::c_int;

// Smaller and simpler implementations for CDM-16.

#[inline(always)]
pub unsafe fn copy_forward(dest: *mut u8, src: *const u8, count: usize) {
    asm!(
        "tst r2",
        "ble 1f",
        "0:",
        "ldb r1, r3",
        "stb r0, r3",
        "inc r0",
        "inc r1",
        "dec r2",
        "bgt 0b",
        "1:",
        in("r0") dest,
        in("r1") src,
        in("r2") count,
        clobber_abi("C"),
        options(nostack),
    );
}

#[inline(always)]
pub unsafe fn copy_backward(dest: *mut u8, src: *const u8, count: usize) {
    asm!(
        "tst r2",
        "ble 1f",
        "add r0, r2, r0",
        "add r1, r2, r1",
        "dec r0",
        "dec r1",
        "0:",
        "ldb r1, r3",
        "stb r0, r3",
        "dec r0",
        "dec r1",
        "dec r2",
        "bgt 0b",
        "1:",
        in("r0") dest,
        in("r1") src,
        in("r2") count,
        clobber_abi("C"),
        options(nostack),
    );
}

#[inline(always)]
pub unsafe fn set_bytes(s: *mut u8, c: u8, n: usize) {
    asm!(
        "tst r2",
        "ble 1f",
        "0:",
        "stb r0, r1",
        "inc r0",
        "dec r2",
        "bgt 0b",
        "1:",
        in("r0") s,
        in("r1") c,
        in("r2") n,
        clobber_abi("C"),
        options(nostack),
    );
}

#[inline(always)]
pub unsafe fn compare_bytes(s1: *const u8, s2: *const u8, n: usize) -> c_int {
    let result : c_int;
    asm!(
        "ldi r4, 0",
        "tst r2",
        "ble 1f",
        "0:",
        "ldb r0, r3",
        "ldb r1, r4",
        "sub r3, r4, r4",
        "bnz 1f",
        "inc r0",
        "inc r1",
        "dec r2",
        "bgt 0b",
        "1:",
        in("r0") s1,
        in("r1") s2,
        in("r2") n,
        out("r4") result,
        clobber_abi("C"),
        options(readonly, nostack),
    );
    result
}

#[inline(always)]
pub unsafe fn c_string_length(mut s: *const core::ffi::c_char) -> usize {
    let len : usize;
    asm!(
        "move r0, r1",
        "br 1f",
        "0:",
        "inc r0",
        "1:",
        "ldb r0, r2",
        "tst r2",
        "bnz 0b",
        "sub r0, r1, r1",
        in("r0") s,
        out("r1") len,
        clobber_abi("C"),
        options(readonly, nostack),
    );
    len
}
