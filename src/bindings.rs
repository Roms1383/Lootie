//! Types generated from [red4ext-rs-bindings](https://github.com/jac3km4/red4ext-rs-bindings).

use red4ext_rs::{
    types::{CName, IScriptable, ItemId, RedArray, Ref, ScriptClass, Scripted, WeakRef},
    NativeRepr,
};

#[repr(C)]
pub struct ScriptedPuppet {
    pub _padding0: [u8; 0xF0],
    pub ai_controller: Ref<IScriptable>,              // 0xF0
    pub move_policies: Ref<IScriptable>,              // 0x100
    pub ai_state_handler_component: Ref<IScriptable>, // 0x110
    pub hit_reaction_component: Ref<IScriptable>,     // 0x120
    pub signal_handler_component: Ref<IScriptable>,   // 0x130
    pub reaction_component: Ref<IScriptable>,         // 0x140
    pub dismemberment_component: Ref<IScriptable>,    // 0x150
    pub hit_represantation: Ref<IScriptable>,         // 0x160
    pub interaction_component: Ref<IScriptable>,      // 0x170
    pub slot_component: Ref<IScriptable>,             // 0x180
    pub senses_component: Ref<IScriptable>,           // 0x190
    pub visible_object_component: Ref<IScriptable>,   // 0x1A0
    pub visible_object_position_updated: bool,        // 0x1B0
    pub sensor_object_component: Ref<IScriptable>,    // 0x1B8
    pub target_tracker_component: Ref<IScriptable>,   // 0x1C8
    pub targeting_components_array: RedArray<Ref<IScriptable>>, // 0x1D8
    pub states_component: Ref<IScriptable>,           // 0x1E8
    pub fx_resource_mapper: Ref<IScriptable>,         // 0x1F8
    pub linked_status_effect: LinkedStatusEffect,     // 0x208
    pub resource_library_component: Ref<IScriptable>, // 0x230
    pub crowd_member_component: Ref<IScriptable>,     // 0x240
    pub inventory_component: WeakRef<IScriptable>,    // 0x250
    pub object_selection_component: Ref<IScriptable>, // 0x260
    pub transform_history_component: Ref<IScriptable>, // 0x270
    pub animation_controller_component: Ref<IScriptable>, // 0x280
    pub bump_component: Ref<IScriptable>,             // 0x290
    pub is_crowd: bool,                               // 0x2A0
    pub incapacitated_on_attach: bool,                // 0x2A1
    pub is_iconic: bool,                              // 0x2A2
    pub combat_hud_manager: Ref<IScriptable>,         // 0x2A8
    pub expose_position: bool,                        // 0x2B8
    pub puppet_state_blackboard: Ref<IScriptable>,    // 0x2C0
    pub custom_blackboard: Ref<IScriptable>,          // 0x2D0
    pub security_area_callback_id: u32,               // 0x2E0
    pub custom_ai_components: RedArray<Ref<IScriptable>>, // 0x2E8
    pub listeners: RedArray<Ref<IScriptable>>,        // 0x2F8
    pub security_support_listener: Ref<IScriptable>,  // 0x308
    pub should_be_revealed_storage: Ref<IScriptable>, // 0x318
    pub input_processed: bool,                        // 0x328
    pub should_spawn_blood_puddle: bool,              // 0x329
    pub blood_puddle_spawned: bool,                   // 0x32A
    pub skip_death_animation: bool,                   // 0x32B
    pub hit_history: Ref<IScriptable>,                // 0x330
    pub current_workspot_tags: RedArray<CName>,       // 0x340
    pub loot_quality: GamedataQuality,                // 0x350
    pub has_quest_items: bool,                        // 0x354
    pub active_quality_range_interaction: CName,      // 0x358
    pub dropped_weapons: bool,                        // 0x360
    pub weakspot_component: Ref<IScriptable>,         // 0x368
    pub breach_controller_component: Ref<IScriptable>, // 0x378
    pub highlight_data: Ref<IScriptable>,             // 0x388
    pub current_tags_stack: u32,                      // 0x398
    pub killer: WeakRef<IScriptable>,                 // 0x3A0
    pub object_actions_callback_ctrl: Ref<IScriptable>, // 0x3B0
    pub is_active_cached: CachedBoolValue,            // 0x3C0
    pub is_cyberpsycho: bool,                         // 0x3C3
    pub is_civilian: bool,                            // 0x3C4
    pub is_police: bool,                              // 0x3C5
    pub is_ganger: bool,                              // 0x3C6
    pub currently_uploading_action: WeakRef<IScriptable>, // 0x3C8
    pub gameplay_role_component: WeakRef<IScriptable>, // 0x3D8
    pub active_quickhack_action_history: RedArray<Ref<IScriptable>>, // 0x3E8
    pub completed_quickhack_history: RedArray<Ref<IScriptable>>, // 0x3F8
    pub is_finsher_sound_played: bool,                // 0x408
    pub attempted_shards: RedArray<ItemId>,           // 0x410
}

unsafe impl ScriptClass for ScriptedPuppet {
    const CLASS_NAME: &'static str = "ScriptedPuppet";
    type Kind = Scripted;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GamedataQuality {
    Common = 0,
    CommonPlus = 1,
    Epic = 2,
    EpicPlus = 3,
    Iconic = 4,
    Legendary = 5,
    LegendaryPlus = 6,
    LegendaryPlusPlus = 7,
    Random = 8,
    Rare = 9,
    RarePlus = 10,
    Uncommon = 11,
    UncommonPlus = 12,
    Count = 13,
    Invalid = 14,
}

unsafe impl NativeRepr for GamedataQuality {
    const NAME: &'static str = "gamedataQuality";
}

#[repr(C, align(8))]
pub struct LinkedStatusEffect {
    pub _padding0: [u8; 0x28],
}

unsafe impl NativeRepr for LinkedStatusEffect {
    const NAME: &'static str = "LinkedStatusEffect";
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct CachedBoolValue {
    pub _padding0: [u8; 0x3],
}

unsafe impl NativeRepr for CachedBoolValue {
    const NAME: &'static str = "AIUtilsCachedBoolValue";
}
