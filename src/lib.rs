#![feature(lang_items)]
#![feature(asm)]
#![no_std]

/// Create a dummy unwind function for any non-standard refs to it in 3rd party libs
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
        loop {}
}

//mod vidio;

//use vidio::TTY;

#[no_mangle]
pub extern fn main() {
    let msg = b"RUSTXU";
    let color_byte = 0x1f; // white foreground, blue background

    let mut msg_colored = [color_byte; 12];
    for (i, char_byte) in msg.into_iter().enumerate() {
        msg_colored[i*2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = msg_colored };

    //let tty = TTY::
    //wtty(b'H');
    //wtty(b'E');
    //wtty(b'L');
    //wtty(b'L');
    //wtty(b'O');
    //wtty(b' ');
    //wtty(b'W');
    //wtty(b'O');
    //wtty(b'R');
    //wtty(b'L');
    //wtty(b'D');
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
