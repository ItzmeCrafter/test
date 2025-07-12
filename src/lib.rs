// src/lib.rs
mod hooks;

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn JNI_OnLoad(_vm: jni::JavaVM, _reserved: *mut std::os::raw::c_void) -> jni::sys::jint {
    init_hooks();
    jni::sys::JNI_VERSION_1_6
}

fn init_hooks() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Info)
    );
    
    log::info!("Minecraft Godmode loaded!");
    
    // Get address of Player::hurt function
    let hurt_addr = get_symbol_address("_ZN6Player4hurtEf");  // Mangled symbol
    
    unsafe {
        hooks::on_player_hurt::hook_address(hurt_addr as *mut u8);
    }
}

// Helper to find game symbols
fn get_symbol_address(symbol: &str) -> usize {
    // Use dlsym or pattern scanning here
    // Example for Android:
    let handle = unsafe { libc::dlopen(b"libminecraftpe.so\0".as_ptr() as *const _, libc::RTLD_LAZY) };
    unsafe { libc::dlsym(handle, symbol.as_ptr() as *const _) as usize }
}
