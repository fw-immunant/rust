//! Test for the #[reverse_drop_order] attribute

//@ run-pass

use std::sync::Mutex;

static EVENT_LOG: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

struct LoggedDrop(&'static str);
impl Drop for LoggedDrop {
    fn drop(&mut self) {
        EVENT_LOG.lock().unwrap().push(self.0)
    }
}

#[reverse_drop_order]
struct ReverseStruct {
    _one: LoggedDrop,
    _two: LoggedDrop,
    _three: LoggedDrop,
}

fn rs_init() -> ReverseStruct {
    ReverseStruct {
        _one: LoggedDrop("one"),
        _two: LoggedDrop("two"),
        _three: LoggedDrop("three"),
    }
}

struct NormalStruct {
    _one: LoggedDrop,
    _two: LoggedDrop,
    _three: LoggedDrop,
}

#[reverse_drop_order]
enum ReverseEnum {
    Unit,
    Tuple(LoggedDrop, LoggedDrop, LoggedDrop),
    Struct {
        field1: LoggedDrop,
        field2: LoggedDrop,
        field3: LoggedDrop,
    },
}

fn re_struct_init() -> ReverseEnum {
    ReverseEnum::Struct {
        field1: LoggedDrop("one"),
        field2: LoggedDrop("two"),
        field3: LoggedDrop("three"),
    }
}

pub fn main() {
    // Normal drop order
    let s = NormalStruct {
        _one: LoggedDrop("one"),
        _two: LoggedDrop("two"),
        _three: LoggedDrop("three"),
    };
    drop(s);
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["one", "two", "three"]);

    // Reverse drop order, fully initialized
    let s = rs_init();
    drop(s);
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["three", "two", "one"]);

    // Reverse drop order, with first field moved out
    {
        let s = rs_init();
        drop(s._one);
    }
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["one", "three", "two"]);

    // Reverse drop order, with second field moved out
    {
        let s = rs_init();
        drop(s._two);
    }
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["two", "three", "one"]);

    // Reverse drop order, with third field moved out
    {
        let s = rs_init();
        drop(s._three);
    }
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["three", "two", "one"]);

    // Reverse drop order, unit variant
    {
        let e = ReverseEnum::Unit;
    }
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, Vec::<&str>::new());

    // Reverse drop order, tuple variant
    {
        let e = ReverseEnum::Tuple(LoggedDrop("one"), LoggedDrop("two"), LoggedDrop("three"));
    }
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["three", "two", "one"]);

    // Reverse drop order, tuple variant, first field moved out
    {
        let e = ReverseEnum::Tuple(LoggedDrop("one"), LoggedDrop("two"), LoggedDrop("three"));
        match e {
            ReverseEnum::Tuple(x, _, _) => drop(x),
            _ => {},
        }
    }
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["one", "three", "two"]);

    // Reverse drop order, tuple variant, second field moved out
    {
        let e = ReverseEnum::Tuple(LoggedDrop("one"), LoggedDrop("two"), LoggedDrop("three"));
        match e {
            ReverseEnum::Tuple(_, x, _) => drop(x),
            _ => {},
        }
    }
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["two", "three", "one"]);

    // Reverse drop order, tuple variant, third field moved out
    {
        let e = ReverseEnum::Tuple(LoggedDrop("one"), LoggedDrop("two"), LoggedDrop("three"));
        match e {
            ReverseEnum::Tuple(_, _, x) => drop(x),
            _ => {},
        }
    }
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["three", "two", "one"]);

    // Reverse drop order, struct variant
    {
        let e = re_struct_init();
    }
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["three", "two", "one"]);

    // Reverse drop order, struct variant, first field moved out
    {
        let e = re_struct_init();
        match e {
            ReverseEnum::Struct { field1, .. } => drop(field1),
            _ => {},
        }
    }
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["one", "three", "two"]);

    // Reverse drop order, struct variant, second field moved out
    {
        let e = re_struct_init();
        match e {
            ReverseEnum::Struct { field2, .. } => drop(field2),
            _ => {},
        }
    }
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["two", "three", "one"]);

    // Reverse drop order, struct variant, second field moved out
    {
        let e = re_struct_init();
        match e {
            ReverseEnum::Struct { field3, .. } => drop(field3),
            _ => {},
        }
    }
    let log = std::mem::take(&mut *EVENT_LOG.lock().unwrap());
    dbg!(&log);
    assert_eq!(log, ["three", "two", "one"]);
}
