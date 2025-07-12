// src/hooks.rs
use bhook::hook_fn;
use log::info;  // For logging

hook_fn! {
    fn on_player_hurt(amount: i32) = {
        info!("Blocked {} damage!", amount);
        unsafe { call_original(0) } // Force 0 damage
    }
}
