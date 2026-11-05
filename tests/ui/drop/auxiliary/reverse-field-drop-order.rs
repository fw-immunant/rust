use std::sync::Mutex;

pub static EVENT_LOG: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

pub struct LoggedDrop(&'static str);
impl Drop for LoggedDrop {
    fn drop(&mut self) {
        EVENT_LOG.lock().unwrap().push(self.0)
    }
}

#[reverse_drop_order]
pub struct ReverseStruct {
    field1: LoggedDrop,
    field2: LoggedDrop,
    field3: LoggedDrop,
}
