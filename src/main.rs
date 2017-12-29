#[macro_use]
extern crate gfx;
extern crate gfx_window_sdl;
extern crate sdl2;

use gfx::Device;
pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut builder = video.window("Example", 800, 600);

    let (mut window, mut gl_context, mut device, mut factory, color_view, depth_view) =
        gfx_window_sdl::init::<ColorFormat, DepthFormat>(builder).unwrap();

    let mut running = true;

    while running {
        let mut event_pump = sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => running = false,
                _ => {}
            }
        }
        
        window.gl_swap_window();
        device.cleanup();
    }
}
