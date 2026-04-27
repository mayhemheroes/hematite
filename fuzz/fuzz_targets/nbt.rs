#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    _ = hematite::minecraft::nbt::Nbt::from_reader(data);
});
