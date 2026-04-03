#![allow(dead_code)]

use slint::platform::software_renderer::SoftwareRenderer;
use slint::Window;

/// Рендерит Slint компонент в Wayland SHM буфер
#[allow(unused_variables)]
pub fn render_frame(
    renderer: &SoftwareRenderer,
    window: &Window,
    surface_buffer: &mut [u8],
    width: u32,
    height: u32,
) {
    // Simplified render - just clear buffer
    for pixel in surface_buffer.chunks_exact_mut(4) {
        pixel[0] = 0x2d; // B
        pixel[1] = 0x2d; // G
        pixel[2] = 0x2d; // R
        pixel[3] = 0xff; // A
    }
}
