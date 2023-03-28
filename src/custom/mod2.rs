use smash::lib::{L2CValue,lua_const::*};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::app::lua_bind::*;
use smashline::*;
use smash::app::BattleObjectModuleAccessor;
use smash::app::FighterEntryID;
use skyline::nn::ro::LookupSymbol;
use core::f32::consts::PI;
use smash::app;

pub static mut FIGHTER_MANAGER: usize = 0;

pub unsafe fn fighter_info(fighter_boma: &mut BattleObjectModuleAccessor) -> *mut smash::app::FighterInformation {
    let entry_id = WorkModule::get_int(fighter_boma,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    return smash::app::lua_bind::FighterManager::get_fighter_information(fighter_manager,FighterEntryID(entry_id));
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAIBase)]

pub unsafe fn ai_main(fighter: &mut L2CFighterAIBase) -> L2CAgent {
    wait_random(fighter, 99999.0, 99999.0);
    original!()(fighter)
}


pub fn install() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
    }
}