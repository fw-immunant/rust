//@ run-pass
#![feature(const_destruct)]
use std::marker::Destruct;
use std::sync::Mutex;

static EVENT_LOG: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

struct Foo;
impl Drop for Foo {
    fn drop(&mut self) {
        EVENT_LOG.lock().unwrap().push("Foo")
    }
}
struct Bar;
impl Drop for Bar {
    fn drop(&mut self) {
        EVENT_LOG.lock().unwrap().push("Bar")
    }
}

// Normally, fields would be dropped in declaration order, with `bar` dropped before `foo`.
struct HoldsBarFoo {
    bar: Bar,
    foo: Foo,
}

// Drop `foo` before `bar`.
impl Destruct for HoldsBarFoo {
    unsafe fn drop_fields(to_drop: *mut Self) {
        // SAFETY: Drops `.foo` and `.bar`, in that order, each via a single call to `Destruct::drop_in_place`.
        // Neither field is used after the call to `drop_in_place`, so use-after-drop is impossible.
        unsafe {
            std::ptr::drop_in_place(&raw mut (*to_drop).foo);
            std::ptr::drop_in_place(&raw mut (*to_drop).bar);
        }
    }
}

fn main(){
    std::mem::drop(HoldsBarFoo { bar: Bar, foo: Foo });
    assert_eq!(*EVENT_LOG.lock().unwrap(), ["Foo", "Bar"]);
}
