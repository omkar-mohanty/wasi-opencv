use std::mem::MaybeUninit;

pub fn imread(filename:&'static str) {
    let filename = filename.as_ptr();

    let imgptr = MaybeUninit::<u32>::uninit();
    unsafe {
        imgcodecs_raw::wasmedge_imgcodecs_imread(imgptr.as_ptr() as i32, filename as i32);
    }

}

mod imgcodecs_raw {

    #[link(wasm_import_module="wasmedge_imgcodecs_ephemral")]
    extern "C" {

        pub fn wasmedge_imgcodecs_imread(arg1:i32, arg2: i32) -> i32;

    }
}
