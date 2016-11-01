#![feature(lang_items)]
#![feature(asm)]
#![no_std]

//mod vidio;

//use vidio::TTY;

#[no_mangle]
pub extern fn main() {
    let x = 1;
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
