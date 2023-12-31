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
  println!("{}", YAfterTransactionEvent::type_layout());
  println!("{}", YStateVector::type_layout());
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

#[derive(TypeLayout)]
#[repr(C)]
struct YMapInputData {
    keys: *mut *mut c_char,
    values: *mut YInput,
}

#[derive(TypeLayout)]
#[repr(C)]
pub struct YAfterTransactionEvent {
    /// Descriptor of a document state at the moment of creating the transaction.
    pub before_state: YStateVector,
    /// Descriptor of a document state at the moment of committing the transaction.
    pub after_state: YStateVector,
    /// Information about all items deleted within the scope of a transaction.
    pub delete_set: YDeleteSet,
}

#[derive(TypeLayout)]
#[repr(C)]
pub struct YStateVector {
    /// Number of clients. It describes a length of both `client_ids` and `clocks` arrays.
    pub entries_count: u32,
    /// Array of unique client identifiers (length is given in `entries_count` field). Each client
    /// ID has corresponding clock attached, which can be found in `clocks` field under the same
    /// index.
    pub client_ids: *mut u64,
    /// Array of clocks (length is given in `entries_count` field) known for each client. Each clock
    /// has a corresponding client identifier attached, which can be found in `client_ids` field
    /// under the same index.
    pub clocks: *mut u32,
}

#[repr(C)]
pub struct YDeleteSet {
    /// Number of client identifier entries.
    pub entries_count: u32,
    /// Array of unique client identifiers (length is given in `entries_count` field). Each client
    /// ID has corresponding sequence of ranges attached, which can be found in `ranges` field under
    /// the same index.
    pub client_ids: *mut u64,
    /// Array of range sequences (length is given in `entries_count` field). Each sequence has
    /// a corresponding client ID attached, which can be found in `client_ids` field under
    /// the same index.
    pub ranges: *mut u64,
}
