//! Hook for [HasLootableItems](https://nativedb.red4ext.com/gamePuppet#HasLootableItems).

use red4ext_rs::{
    addr_hashes, hooks, log,
    types::{CName, IScriptable, StackFrame, WeakRef},
    PluginOps, RttiSystem, SdkEnv, VoidPtr,
};

use crate::{bindings::ScriptedPuppet, LootIconsExtensionFix};

/// Corresponding hash on:
/// - `2.12a`: `0x141117628`
/// - `2.13`:  `0x1411C26D8`
const OFFSET: u32 = 0x362B23C7;

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

pub(super) fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(OFFSET);
    let addr = unsafe {
        std::mem::transmute::<
            usize,
            unsafe extern "C" fn(*mut IScriptable, *mut StackFrame, VoidPtr, VoidPtr),
        >(addr)
    };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for gamePuppet.HasLootableItems");
}

unsafe extern "C" fn detour(
    i: *mut IScriptable,
    f: *mut StackFrame,
    a3: VoidPtr,
    a4: VoidPtr,
    cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
) {
    // save stack frame args state
    let frame = &mut *f;
    let state = frame.args_state();

    let env = LootIconsExtensionFix::env();

    #[cfg(debug_assertions)]
    log::info!(env, "intercepted HasLootableItems!");

    // read stack frame arg
    let puppet: WeakRef<ScriptedPuppet> = StackFrame::get_arg(frame);

    // retrieve Redscript method
    let func_name = "LootIconsExtensionFix.HasLootableItems";
    let rtti = RttiSystem::get();
    let funcs = rtti.get_global_functions();
    let func = funcs
        .iter()
        .find(|x| x.short_name() == CName::new(func_name))
        .unwrap();

    // call Redscript method
    match func.execute::<_, bool>(None, (puppet,)) {
        Ok(has) => {
            let out = a3 as *mut bool;
            *out = has;

            #[cfg(debug_assertions)]
            log::info!(
                env,
                "wrote LootIconsExtensionFix.HasLootableItems()'s output successfully: {has}"
            );
        }
        // if detouring fails, fallback to vanilla method
        Err(e) => {
            // restore stack frame args
            frame.restore_args(state);

            cb(i, f, a3, a4);

            log::error!(env, "unable to call {func_name}: {e}");
        }
    };
}
