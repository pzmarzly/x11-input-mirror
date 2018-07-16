use std::process::Command;

pub fn reset_keys() {
    let key_up = |name| Command::new("xdotool").arg("keyup").arg(name).status().is_ok();
    key_up("Control_L");
    key_up("Control_R");
    key_up("Shift_L");
    key_up("Shift_R");
    key_up("Alt_L");
    key_up("Alt_R");
    key_up("Super_L");
    key_up("Super_R");
    key_up("ISO_Level3_Shift"); // AltGr
    Command::new("numlockx").arg("on").status().is_ok();
}