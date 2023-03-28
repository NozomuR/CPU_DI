#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
use skyline::nro::{self, NroInfo};
use smash::params::add_hook;

mod custom;

pub fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.module.isLoaded {
        return;
    }

    if info.name == "common" {
        skyline::install_hooks!(
            custom::dmg_fly_main
        );
    }
}

#[skyline::main(name = "dmg_fly")]
pub fn main() {
    nro::add_hook(nro_hook).unwrap();
    custom::install()
}