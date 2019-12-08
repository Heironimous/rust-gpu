#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    // creates a black 800x800 window and waits a few seconds.
    #[test]
    fn init_test() {

        unsafe {
            let target = GPU_Init(800, 800, GPU_DEFAULT_INIT_FLAGS);
            let clearer = GPU_MakeRect(0.0f32, 0.0f32, 800f32, 800f32);
            let color = SDL_Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0xFF
            };

            

            GPU_Clear(target);

            GPU_RectangleFilled2(target, clearer, color);

            GPU_Flip(target);

            let dur = std::time::Duration::from_secs(5 as u64);
            std::thread::sleep(dur);

            GPU_FreeTarget(target);
            GPU_Quit();
        }
    }
}
