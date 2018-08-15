use std::panic;

fn pretty_panic(panic_info: &panic::PanicInfo) {
    if let Some(info) = panic_info.message() {
        print!("{}", info);
    } else if let Some(info) = panic_info.payload().downcast_ref::<&'static str>() {
        print!("{}", info);
    }
    if let Some(loc) = panic_info.location() {
        println!(" @ {}:{}", loc.file(), loc.line());
    } else {
        println!();
    }
}

pub fn set() {
    panic::set_hook(Box::new(pretty_panic));
}
