use type_layout::TypeLayout;
use std::ffi::{c_char, c_void, CStr, CString};
use std::mem::{forget, ManuallyDrop, MaybeUninit};

// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.
fn main() {
  // Statements here are executed when the compiled binary is called.

  println!("{}", YInput::type_layout());
}

#[derive(TypeLayout)]
#[repr(C)]
pub struct YInput {
    /// Tag describing, which `value` type is being stored by this input cell. Can be one of:
    ///
    /// - [Y_JSON_BOOL] for boolean flags.
    /// - [Y_JSON_NUM] for 64-bit floating point numbers.
    /// - [Y_JSON_INT] for 64-bit signed integers.
    /// - [Y_JSON_STR] for null-terminated UTF-8 encoded strings.
    /// - [Y_JSON_BUF] for embedded binary data.
    /// - [Y_JSON_ARR] for arrays of JSON-like values.
    /// - [Y_JSON_MAP] for JSON-like objects build from key-value pairs.
    /// - [Y_JSON_NULL] for JSON-like null values.
    /// - [Y_JSON_UNDEF] for JSON-like undefined values.
    /// - [Y_ARRAY] for cells which contents should be used to initialize a `YArray` shared type.
    /// - [Y_MAP] for cells which contents should be used to initialize a `YMap` shared type.
    /// - [Y_DOC] for cells which contents should be used to nest a `YDoc` sub-document.
    pub tag: i8,

    /// Length of the contents stored by current `YInput` cell.
    ///
    /// For [Y_JSON_NULL] and [Y_JSON_UNDEF] its equal to `0`.
    ///
    /// For [Y_JSON_ARR], [Y_JSON_MAP], [Y_ARRAY] and [Y_MAP] it describes a number of passed
    /// elements.
    ///
    /// For other types it's always equal to `1`.
    pub len: u32,

    /// Union struct which contains a content corresponding to a provided `tag` field.
    value: YInputContent,
}

pub struct Doc { 
}

#[repr(C)]
union YInputContent {
    flag: u8,
    num: f64,
    integer: i64,
    str: *mut c_char,
    buf: *mut c_char,
    values: *mut YInput,
    map: ManuallyDrop<YMapInputData>,
    doc: *mut Doc,
}

#[repr(C)]
struct YMapInputData {
    keys: *mut *mut c_char,
    values: *mut YInput,
}