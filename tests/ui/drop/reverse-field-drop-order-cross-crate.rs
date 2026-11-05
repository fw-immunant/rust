//! Test that the #[reverse_drop_order] attribute applies across crates

//@ run-pass
//@ aux-build: reverse-field-drop-order.rs
extern crate reverse_field_drop_order;

use reverse_field_drop_order::{ReverseStruct, LoggedDrop, EVENT_LOG};
fn main() {
    {
        let s = ReverseStruct {
            field1: LoggedDrop("one"),
            field2: LoggedDrop("two"),
            field3: LoggedDrop("three"),
        };
        // Drop a field before dropping s so that we locally generate a drop ladder (relying on
        // cross-crate metadata to see the attribute) rather than calling into an exported dtor.
        drop(s.2);
    }

    assert_eq!(EVENT_LOG.lock().unwrap(), ["two", "three", "one"]);
}
