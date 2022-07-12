use libwifi::frame::components::MacAddress;

mod parsing_tests;
mod serializing_tests;

pub const TEST_MAC_1: MacAddress = MacAddress([0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD]);
pub const TEST_MAC_2: MacAddress = MacAddress([0xBE, 0xEF, 0xBE, 0xEF, 0xBE, 0xEF]);
pub const TEST_MAC_3: MacAddress = MacAddress([0xC0, 0xDE, 0xC0, 0xDE, 0xC0, 0xDE]);
pub const TEST_MAC_4: MacAddress = MacAddress([0xCA, 0xFE, 0xCA, 0xFE, 0xCA, 0xFE]);

fn compare_byte_slice(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        false
    } else {
        a.iter().zip(b.iter()).all(|(a, b)| *a == *b)
    }
}
