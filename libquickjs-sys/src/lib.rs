//! FFI Bindings for [quickjs](https://bellard.org/quickjs/),
//! a Javascript engine.
//! See the [quickjs](https://crates.io/crates/quickjs) crate for a high-level
//! wrapper.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use super::{JS_NewRuntime, JS_NewContext, JS_EVAL_TYPE_GLOBAL, JSContext, JSValue};

    extern "C" {
        pub fn JS_Eval(
            ctx: *mut JSContext,
            input: *const ::std::os::raw::c_char,
            input_len: usize,
            filename: *const ::std::os::raw::c_char,
            eval_flags: ::std::os::raw::c_int,
        ) -> i64;
    }

    // Small sanity test that starts the runtime and evaluates code.
    #[test]
    fn test_eval() {
        unsafe {
            let rt = JS_NewRuntime();
            let ctx = JS_NewContext(rt);

            let code_str = "function sum(x, y) {return (x + y) * 2;};sum(4,1)\0";
            let code = CStr::from_bytes_with_nul(code_str.as_bytes()).unwrap();
            let script = CStr::from_bytes_with_nul("script\0".as_bytes()).unwrap();

            let value = JS_Eval(
                ctx,
                code.as_ptr(),
                code_str.len() - 1,
                script.as_ptr(),
                JS_EVAL_TYPE_GLOBAL as i32,
            );
            println!("Result: {:?}", value);
        }
    }
}
