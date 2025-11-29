//! # Roman Numeral Conversion Engine
//! 
//! The Titan-Machine Citadel's Core Engine
//! 
//! This library provides safe, high-performance Roman numeral conversion
//! with FFI exports for C# interop.

mod converter;
mod validator;
mod algorithms;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// FFI-safe result code
#[repr(C)]
pub struct ConversionResult {
    pub success: bool,
    pub value: i32,
    pub error_code: i32,
}

/// Error codes for FFI
pub mod errors {
    pub const SUCCESS: i32 = 0;
    pub const INVALID_INPUT: i32 = 1;
    pub const OUT_OF_RANGE: i32 = 2;
    pub const NULL_POINTER: i32 = 3;
}

/// Convert Roman numeral string to integer
/// 
/// # Safety
/// `roman_str` must be a valid null-terminated C string
#[no_mangle]
pub unsafe extern "C" fn roman_to_int(roman_str: *const c_char) -> ConversionResult {
    // Validate pointer
    if roman_str.is_null() {
        return ConversionResult {
            success: false,
            value: 0,
            error_code: errors::NULL_POINTER,
        };
    }

    // Convert C string to Rust string
    let c_str = match CStr::from_ptr(roman_str).to_str() {
        Ok(s) => s,
        Err(_) => {
            return ConversionResult {
                success: false,
                value: 0,
                error_code: errors::INVALID_INPUT,
            };
        }
    };

    // Validate Roman numeral
    if !validator::is_valid_roman(c_str) {
        return ConversionResult {
            success: false,
            value: 0,
            error_code: errors::INVALID_INPUT,
        };
    }

    // Perform conversion
    match converter::roman_to_integer(c_str) {
        Ok(value) => ConversionResult {
            success: true,
            value,
            error_code: errors::SUCCESS,
        },
        Err(_) => ConversionResult {
            success: false,
            value: 0,
            error_code: errors::INVALID_INPUT,
        },
    }
}

/// Convert integer to Roman numeral string
/// 
/// # Safety
/// Caller must free the returned string using `free_roman_string`
#[no_mangle]
pub unsafe extern "C" fn int_to_roman(value: i32) -> *mut c_char {
    // Validate range (Roman numerals: 1-3999)
    if value < 1 || value > 3999 {
        return std::ptr::null_mut();
    }

    match converter::integer_to_roman(value) {
        Ok(roman) => {
            match CString::new(roman) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a string allocated by `int_to_roman`
/// 
/// # Safety
/// `s` must be a pointer returned by `int_to_roman` and must only be freed once
#[no_mangle]
pub unsafe extern "C" fn free_roman_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int_basic() {
        let test_cases = vec![
            ("III", 3),
            ("IV", 4),
            ("IX", 9),
            ("LVIII", 58),
            ("MCMXCIV", 1994),
        ];

        for (roman, expected) in test_cases {
            let c_string = CString::new(roman).unwrap();
            let result = unsafe { roman_to_int(c_string.as_ptr()) };
            assert!(result.success);
            assert_eq!(result.value, expected);
        }
    }
}
