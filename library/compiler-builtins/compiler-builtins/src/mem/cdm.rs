use core::arch::asm;
use core::ffi::c_int;

// Smaller and simpler implementations for CDM-16.

#[inline(always)]
pub unsafe fn copy_forward(mut dest: *mut u8, mut src: *const u8, mut count: usize) {
    asm!(
        "tst {cnt}",
        "bz 1f",
        "0:",
        "ldb {src}, {tmp}",
        "stb {dst}, {tmp}",
        "inc {src}",
        "inc {dst}",
        "dec {cnt}",
        "bnz 0b",
        "1:",
        dst = inout(reg) dest,
        src = inout(reg) src,
        cnt = inout(reg) count,
        tmp = out(reg) _,
        options(nostack),
    );
}

#[inline(always)]
pub unsafe fn copy_backward(mut dest: *mut u8, mut src: *const u8, mut count: usize) {
    asm!(
        "tst {cnt}",
        "bz 1f",
        "add {dst}, {cnt}, {dst}",
        "add {src}, {cnt}, {src}",
        "dec {dst}",
        "dec {src}",
        "0:",
        "ldb {src}, {tmp}",
        "stb {dst}, {tmp}",
        "dec {dst}",
        "dec {src}",
        "dec {cnt}",
        "bnz 0b",
        "1:",
        dst = inout(reg) dest,
        src = inout(reg) src,
        cnt = inout(reg) count,
        tmp = out(reg) _,
        options(nostack),
    );
}

#[inline(always)]
pub unsafe fn set_bytes(mut s: *mut u8, c: u8, mut n: usize) {
    asm!(
        "tst {cnt}",
        "bz 1f",
        "0:",
        "stb {dst}, {val}",
        "inc {dst}",
        "dec {cnt}",
        "bnz 0b",
        "1:",
        dst = inout(reg) s,
        val = in(reg) c,
        cnt = inout(reg) n,
        options(nostack),
    );
}

#[inline(always)]
pub unsafe fn compare_bytes(mut s1: *const u8, mut s2: *const u8, mut n: usize) -> c_int {
    let result : c_int;
    asm!(
        "ldi {val2}, 0",
        "tst {cnt}",
        "bz 1f",
        "0:",
        "ldb {ptr1}, {val1}",
        "ldb {ptr2}, {val2}",
        "sub {val1}, {val2}, {val2}",
        "bnz 1f",
        "inc {ptr1}",
        "inc {ptr2}",
        "dec {cnt}",
        "bnz 0b",
        "1:",
        ptr1 = inout(reg) s1,
        ptr2 = inout(reg) s2,
        cnt = inout(reg) n,
        val1 = out(reg) _,
        val2 = out(reg) result,
        options(readonly, nostack),
    );
    result
}

#[inline(always)]
pub unsafe fn c_string_length(mut s: *const core::ffi::c_char) -> usize {
    let len : usize;
    asm!(
        "move {ptr}, {start}",
        "br 1f",
        "0:",
        "inc {ptr}",
        "1:",
        "ldb {ptr}, {tmp}",
        "tst {tmp}",
        "bnz 0b",
        "sub {ptr}, {start}, {res}",
        ptr = inout(reg) s,
        start = out(reg) _,
        tmp = out(reg) _,
        res = lateout(reg) len,
        options(readonly, nostack),
    );
    len
}
