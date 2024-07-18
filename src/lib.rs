//! Lootie
//! 
//! ```swift
#![doc = include_str!("../reds/Lootie.reds")]
//! ```

use hook::attach_hook;
use red4ext_rs::{export_plugin, wcstr, ExportNil, Exportable, Plugin, SdkEnv, SemVer, U16CStr};

mod bindings;
mod hook;

struct Lootie;

impl Plugin for Lootie {
    const NAME: &'static U16CStr = wcstr!("lootie");
    const AUTHOR: &'static U16CStr = wcstr!("DerekM07 + Roms1383");
    const VERSION: SemVer = SemVer::new(0, 1, 0);

    fn on_init(env: &SdkEnv) {
        attach_hook(env);
    }

    fn exports() -> impl Exportable {
        ExportNil
    }
}

export_plugin!(Lootie);
