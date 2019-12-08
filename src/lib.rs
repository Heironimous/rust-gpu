extern crate gpu_sys;

#[cfg(test)]
mod tests {
    use super::gpu_sys;
    
    
    // creates a black 800x800 window and waits a few seconds.
    #[test]
    fn init_test() {

        unsafe {
            let target = gpu_sys::GPU_Init(800, 800, gpu_sys::GPU_DEFAULT_INIT_FLAGS);
            let clearer = gpu_sys::GPU_MakeRect(0.0f32, 0.0f32, 800f32, 800f32);
            let color = gpu_sys::SDL_Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0xFF
            };

            

            gpu_sys::GPU_Clear(target);

            gpu_sys::GPU_RectangleFilled2(target, clearer, color);

            gpu_sys::GPU_Flip(target);

            let dur = std::time::Duration::from_secs(5 as u64);
            std::thread::sleep(dur);

            gpu_sys::GPU_FreeTarget(target);
            gpu_sys::GPU_Quit();
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
