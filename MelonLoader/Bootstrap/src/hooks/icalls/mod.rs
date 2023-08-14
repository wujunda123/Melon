mod il2cpp;

use crate::{
    debug,
    errors::DynErr,
    runtime
};
use std::ffi::c_void;
use super::NativeHook;

pub fn hook() -> Result<(), DynErr> {
    debug!("Attaching hook to il2cpp_add_internal_call il2cpp_resolve_icall")?;

    let runtime = runtime!()?;

    let func = runtime.get_export_ptr("il2cpp_add_internal_call")?;
    let detour = il2cpp::detour_add as usize;
    let mut add_hook = il2cpp::ICALL_ADD_HOOK.try_write()?;
    *add_hook = NativeHook::new(func, detour as *mut c_void);
    add_hook.hook()?;

    let func = runtime.get_export_ptr("il2cpp_resolve_icall")?;
    let detour = il2cpp::detour_resolve as usize;
    let mut resolve_hook = il2cpp::ICALL_RESOLVE_HOOK.try_write()?;
    *resolve_hook = NativeHook::new(func, detour as *mut c_void);
    resolve_hook.hook()?;

    Ok(())
}
