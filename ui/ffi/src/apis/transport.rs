use std::sync::Arc;
use pinboard::NonEmptyPinboard;
use mizer_clock::ClockSnapshot;
use crate::types::{drop_pointer, FFIFromPointer};

pub struct Transport {
    pub clock_ref: Arc<NonEmptyPinboard<ClockSnapshot>>,
}

#[repr(C)]
pub struct Timecode {
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub frames: u64,
}

impl From<mizer_clock::Timecode> for Timecode {
    fn from(tc: mizer_clock::Timecode) -> Self {
        Self {
            hours: tc.hours,
            minutes: tc.minutes,
            seconds: tc.seconds,
            frames: tc.frames,
        }
    }
}

#[no_mangle]
pub extern fn read_timecode(ptr: *const Transport) -> Timecode {
    let ffi = Arc::from_pointer(ptr);

    let snapshot = ffi.clock_ref.read();

    std::mem::forget(ffi);

    snapshot.time.into()
}

#[no_mangle]
pub extern fn drop_transport_pointer(ptr: *const Transport) {
    drop_pointer(ptr);
}
