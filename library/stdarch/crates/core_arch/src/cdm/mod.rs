// CDM intrinsics

unsafe extern "unadjusted" {
    #[link_name = "llvm.cdm.halt"]
    fn _halt() -> !;

    #[link_name = "llvm.cdm.wait"]
    fn _wait();

    #[link_name = "llvm.cdm.di"]
    fn _di();

    #[link_name = "llvm.cdm.ei"]
    fn _ei();

    #[link_name = "llvm.cdm.int"]
    fn _int(vec: u16);

    #[link_name = "llvm.cdm.reset"]
    fn _reset(vec: u16) -> !;

    #[link_name = "llvm.cdm.ldps"]
    fn _ldps() -> u16;

    #[link_name = "llvm.cdm.stps"]
    fn _stps(ps: u16);
}

/// Represents the value of the processor state register.
#[stable(feature = "cdm_intrinsics", since = "1.90")]
pub type ProcState = u16;

/// The interrupt flag of the processor state register.
#[stable(feature = "cdm_intrinsics", since = "1.90")]
pub const PS_INT: u16 = 0x8000;


/// Generates the `halt` instruction
///
/// The `halt` instruction transitions the processor into the `HALTED` state and stops the clock.
#[stable(feature = "cdm_intrinsics", since = "1.90")]
#[inline]
pub unsafe fn halt() -> ! {
    _halt();
}

/// Generates the `wait` instruction
///
/// The `wait` instruction transitions the processor into the `WAITING` state. The clock is stopped
/// until an interrupt request is received.
#[stable(feature = "cdm_intrinsics", since = "1.90")]
#[inline]
pub unsafe fn wait() {
    _wait()
}

/// Generates the `int` instruction
///
/// The `int` instruction triggers a software interrupt with the specified number
/// in the range [0; 511].
#[stable(feature = "cdm_intrinsics", since = "1.90")]
#[inline]
pub unsafe fn int<const V: u16>() {
    static_assert!(V < 512);
    _int(V)
}

/// Generates the `reset` instruction
///
/// The `reset` instruction fetches the interrupt vector with the specifed number
/// in the range [0; 511].
#[stable(feature = "cdm_intrinsics", since = "1.90")]
#[inline]
pub unsafe fn reset<const V: u16>() -> ! {
    static_assert!(V < 512);
    _reset(V)
}

/// Generates the `di` instruction
///
/// The `di` instruction sets the interrupt bit in the status register to 0.
#[stable(feature = "cdm_intrinsics", since = "1.90")]
#[inline]
pub unsafe fn di() {
    _di()
}

/// Generates the `ei` instruction
///
/// The `ei` instruction sets the interrupt bit in the status register to 1.
#[stable(feature = "cdm_intrinsics", since = "1.90")]
#[inline]
pub unsafe fn ei() {
    _ei()
}

/// Generates the `ldps` instruction
///
/// The `ldps` instruction gets the current value of the status register.
#[stable(feature = "cdm_intrinsics", since = "1.90")]
#[inline]
pub unsafe fn ldps() -> ProcState {
    _ldps()
}

/// Generates the `stps` instruction
///
/// The `stps` instruction sets the value of the status register.
#[stable(feature = "cdm_intrinsics", since = "1.90")]
#[inline]
pub unsafe fn stps(ps: ProcState) {
    _stps(ps)
}
