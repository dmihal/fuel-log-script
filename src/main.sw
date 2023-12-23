script;

use std::tx::tx_script_length;
use std::tx::tx_script_start_pointer;

fn main() {
    let script_length = tx_script_length();
    let script_ptr = tx_script_start_pointer();

    asm(ptr: script_ptr, len: script_length) {
        logd len zero ptr len;
    }
}
