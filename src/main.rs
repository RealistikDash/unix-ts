use std::time::{SystemTime, UNIX_EPOCH};

use clipboard_win::{formats, set_clipboard};

fn get_timestamp() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Incorrectly set system time!")
        .as_secs() as u32
}

fn copy_into_clipboard(value: &String) {
    set_clipboard(formats::Unicode, value).expect("Failed writing to clipboard");
}

fn main() {
    let unix_timestamp = get_timestamp().to_string();
    copy_into_clipboard(&unix_timestamp);
}
