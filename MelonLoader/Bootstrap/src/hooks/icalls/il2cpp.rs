use std::{sync::RwLock, ptr::{null_mut, null}};

use lazy_static::lazy_static;
use libc::c_void;
use std::collections::HashMap;

use crate::{
    errors::DynErr, hooks::NativeHook, internal_failure,
};

pub type FnICallAdd = extern "C" fn(u32, *const c_void);
pub type FnICallResolve = extern "C" fn(u32) -> *const c_void;

lazy_static! {
    pub static ref ICALL_ADD_HOOK: RwLock<NativeHook<FnICallAdd>> =
        RwLock::new(NativeHook::new(null_mut(), null_mut()));

    pub static ref ICALL_RESOLVE_HOOK: RwLock<NativeHook<FnICallResolve>> =
        RwLock::new(NativeHook::new(null_mut(), null_mut()));

    static ref ICALL_MAP: RwLock<HashMap<u32, u64>> = 
        RwLock::new(HashMap::new());
}

pub fn detour_add(hash: u32, func: *const c_void) {
    detour_add_inner(hash, func).unwrap_or_else(|e| {
        internal_failure!("il2cpp_add_internal_call detour failed: {e}");
    })
}

pub fn detour_resolve(hash: u32) -> *const c_void {
    detour_resolve_inner(hash).unwrap_or_else(|e| {
        internal_failure!("il2cpp_resolve_icall detour failed: {e}");
    })
}

fn detour_add_inner(hash: u32, func: *const c_void) -> Result<(), DynErr> {
    let trampoline = ICALL_ADD_HOOK.try_read()?;
    trampoline(hash, func);

    let _ = ICALL_MAP.write()?.insert(hash, func as u64);

    Ok(())
}

fn detour_resolve_inner(hash: u32) -> Result<*const c_void, DynErr> {
    let trampoline = ICALL_RESOLVE_HOOK.try_read()?;
    let result = trampoline(hash);

    if result == null() {
        match ICALL_MAP.read()?.get(&hash) {
            Some(ret) => Ok(*ret as *const c_void),
            None => Ok(result)
        }
    }
    else {
        Ok(result)
    }
}
