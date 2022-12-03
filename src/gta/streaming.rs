pub fn request_model(model_index: i32, flags: i32) {
    let ptr = 0x4087E0 as *const ();
    let func: fn(i32, i32) = unsafe { std::mem::transmute(ptr) };
    func(model_index, flags);
}

pub fn load_all_requested_models(val: bool) {
    let ptr = 0x40EA10 as *const ();
    let func: fn(bool) = unsafe { std::mem::transmute(ptr) };
    func(val);
}
