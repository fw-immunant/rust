//@ run-pass
#![feature(const_destruct)]
use std::marker::Destruct;

struct PanicOnDrop;

impl std::mem::Drop for PanicOnDrop {
    fn drop(&mut self) {
        panic!("should not be dropped due to drop_fields logic of containing type")
    }
}

struct TwoFields {
    a: PanicOnDrop,
    b: PanicOnDrop,
}

impl<T> Destruct for TwoFields {
    unsafe fn drop_fields(mut to_drop: *mut Self) {
        // Simply leak the fields.
        // Mention each field to suppress "unused field in drop_fields" warning.
        let _ = self.a;
        let _ = self.b;
    }
}

fn main() {
    let _ = TwoFields { a: PanicOnDrop, b: PanicOnDrop };
}
