//! LootIconsExtensionFix
//!
//! ```swift
#![doc = include_str!("../reds/LootIconsExtensionFix.reds")]
//! ```

use hook::attach_hook;
use red4ext_rs::{
    export_plugin_symbols, wcstr, ExportNil, Exportable, Plugin, SdkEnv, SemVer, U16CStr,
};

mod bindings;
mod hook;

struct LootIconsExtensionFix;

impl Plugin for LootIconsExtensionFix {
    const NAME: &'static U16CStr = wcstr!("LootIconsExtensionFix");
    const AUTHOR: &'static U16CStr = wcstr!("Demon9ne + Roms1383");
    const VERSION: SemVer = SemVer::new(0, 1, 0);

    fn on_init(env: &SdkEnv) {
        attach_hook(env);
    }

    fn exports() -> impl Exportable {
        ExportNil
    }
}

export_plugin_symbols!(LootIconsExtensionFix);
