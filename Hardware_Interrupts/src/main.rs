#![no_std]
#![no_main]
mod vga_buffer;
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        "\n 12322222 {} {} 2",
        42,
        1.337
    )
    .unwrap();


    loop {}
}
