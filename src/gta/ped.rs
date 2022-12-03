use std::arch::asm;
use std::ffi::c_void;

#[derive(Debug)]
pub struct CPed {
    address: usize,
}

impl CPed {
    pub fn new(address: usize) -> Self {
        Self { address }
    }

    pub fn health(&self) -> f32 {
        unsafe { *((self.address + 0x540) as *const f32) }
    }

    pub fn set_health(&self, health: f32) {
        unsafe {
            *((self.address + 0x540) as *mut f32) = health;
        }
    }

    pub fn target(&self) -> Option<CPed> {
        let target_address = unsafe { *((self.address + 0x79C) as *const usize) };
        if target_address == 0 {
            None
        } else {
            Some(CPed::new(target_address))
        }
    }

    pub fn create_dead_ped_money(&self) {
        let ptr = 0x4590F0 as *const ();
        let func: fn(this: *const c_void) = unsafe { std::mem::transmute(ptr) };
        func(self.address as *const c_void);
    }

    pub fn clear_weapons(&self) {
        let ptr = 0x5E6320 as *const ();
        let func: fn(this: *const c_void) = unsafe { std::mem::transmute(ptr) };
        func(self.address as *const c_void);
    }
}

pub struct CPlayerPed {
    ped: CPed,
}

impl CPlayerPed {
    pub fn new(address: usize) -> Self {
        Self {
            ped: CPed::new(address),
        }
    }

    pub fn health(&self) -> f32 {
        self.ped.health()
    }

    pub fn set_health(&self, health: f32) {
        self.ped.set_health(health);
    }

    pub fn target(&self) -> Option<CPed> {
        self.ped.target()
    }

    pub fn wanted_level(&self) -> i32 {
        let ptr = 0x41BE60 as *const ();
        let func: fn(*const c_void) -> i32 = unsafe { std::mem::transmute(ptr) };
        func(self.ped.address as *const c_void)
    }

    pub fn set_wanted_level(&self, level: i32) {
        unsafe {
            asm!(
                "push {level}",
                "mov ecx, {address}",
                "call {func}",

                address = in(reg) self.ped.address,
                level = in(reg) level,
                func = in(reg) 0x609F10,
            )
        }
    }
}

pub fn player_ped() -> CPed {
    CPed::new(unsafe { *(0xB6F5F0 as *const usize) })
}

pub fn find_player_ped(id: i32) -> CPlayerPed {
    let ptr = 0x56E210 as *const c_void;
    let func: fn(i32) -> usize = unsafe { std::mem::transmute(ptr) };
    CPlayerPed::new(func(id))
}

pub enum PedType {
    Player1,
    Player2,
    PlayerNetwork,
    PlayerUnused,
    CivMale,
    CivFemale,
    Cop,
    Gang1,
    Gang2,
    Gang3,
    Gang4,
    Gang5,
    Gang6,
    Gang7,
    Gang8,
    Gang9,
    Gang10,
    Dealer,
    Medic,
    Fireman,
    Criminal,
    Bum,
    Prostitute,
    Special,
    Mission1,
    Mission2,
    Mission3,
    Mission4,
    Mission5,
    Mission6,
    Mission7,
    Mission8,
}

pub fn new_civilian_ped(pedType: PedType, modelIndex: u32) -> CPed {
    let mut buf = [0u8; 0x79C];
    let ptr = 0x5DDB70 as *const c_void;
    let func: extern "thiscall" fn(*const c_void, u32, u32) -> usize =
        unsafe { std::mem::transmute(ptr) };

    CPed::new(func(
        buf.as_mut_ptr() as *const c_void,
        pedType as u32,
        modelIndex,
    ))
}
