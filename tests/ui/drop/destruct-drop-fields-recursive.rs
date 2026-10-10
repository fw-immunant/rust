//@ run-pass
#![feature(const_destruct)]
use std::marker::Destruct;
use std::atomic::{AtomicUsize, Ordering};

static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);

enum LinkedList<T> {
    Cons(T, Box<LinkedList<T>>),
    Nil,
}

// Drop the list iteratively, avoiding stack overflow.
impl<T> Destruct for LinkedList<T> {
    unsafe fn drop_fields(mut to_drop: *mut Self) {
        let mut temp: Self = LinkedList::Nil;
        std::mem::swap(*to_drop, &mut temp);
        while let LinkedList::Cons(elem, rest) = temp {
            // Box is destroyed here.
            temp = *rest;
        }
        // `temp` is `Nil`, no additional destruction to do.
        std::mem::forget(temp);
    }
}

// Demonstrates that the elements are not leaked.
struct LoudDrop<T: std::fmt::Debug>(T);

impl<T: std::fmt::Debug> Drop for LoudDrop<T> {
    fn drop(&mut self) {
        DROP_COUNTER.fetch_add(1, Ordering::Relaxed);
    }
}

fn main() {
    let mut list = LinkedList::Nil;
    for i in 0..999999 {
        list = LinkedList::Cons(LoudDrop(i), Box::new(list));
    }
    assert!(DROP_COUNTER.load(Ordering::Relaxed) == 1000000)
}
