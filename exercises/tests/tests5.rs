/// # Safety
///
/// The `address` must contain a valid and properly aligned reference to a `u32` value.
///
/// This function takes an `address` as a `usize`. The `address` should point to a valid and
/// properly aligned `u32` value. It is your responsibility to ensure that the `address` is
/// indeed a valid reference to a `u32`. The function modifies the value at the given `address`
/// without performing any checks, so it is crucial that the `address` is valid.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: The caller must ensure that `address` points to a valid `u32`.
    // There are no further checks in this function.
    let ptr = address as *mut u32;
    *ptr = 0xAABBCCDD;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
