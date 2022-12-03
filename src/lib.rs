// #![feature(abi_thiscall)]

use anyhow::Result;
use log4rs::{append::file::FileAppender, config::Appender, config::Root, Config};
use std::ffi::c_void;

use log::LevelFilter;
use windows::Win32::{
    Foundation::HINSTANCE,
    System::{LibraryLoader::FreeLibraryAndExitThread, SystemServices::DLL_PROCESS_ATTACH},
};

#[macro_use]
extern crate log;

mod gta;

unsafe fn submain<'a>() -> Result<()> {
    let player = gta::find_player_ped(-1);
    info!("Player health: {}", player.health());
    info!("Wanted level: {}", player.wanted_level());

    let weather = gta::weather();
    info!("Weather: {:?}", weather);

    // player.set_wanted_level(0);
    // client.set_weather(Weather::SunnyDesert);

    // loop {
    //     for weather in Weather::iter() {
    //         client.set_weather(weather);
    //         std::thread::sleep(Duration::from_millis(25));
    //     }

    //     if let Some(target) = player.target() {
    //         debug!("target: {:?}", target);
    //         if target.health() > 0f32 {
    //             target.set_health(0f32);
    //             debug!("target health: {:?}", target.health());
    //         }
    //     }
    // }

    Ok(())
}

unsafe fn main(#[allow(non_snake_case)] hModule: HINSTANCE) {
    log4rs::init_config(
        Config::builder()
            .appender(
                Appender::builder().build(
                    "file",
                    Box::new(
                        FileAppender::builder()
                            .build(
                                dirs::desktop_dir()
                                    .unwrap()
                                    .join("gta.log")
                                    .to_str()
                                    .unwrap(),
                            )
                            .unwrap(),
                    ),
                ),
            )
            .build(Root::builder().appender("file").build(LevelFilter::Debug))
            .unwrap(),
    )
    .unwrap();
    debug!("Hello, world!");

    let result = submain();
    if let Err(e) = result {
        error!("{}", e);
    }

    debug!("Goodbye, world!\n");
    FreeLibraryAndExitThread(hModule, 0);
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(
    #[allow(non_snake_case)] hModule: HINSTANCE,
    #[allow(non_snake_case)] dwReason: u32,
    #[allow(non_snake_case)] _lpReserved: *const c_void,
) -> bool {
    if dwReason != DLL_PROCESS_ATTACH {
        return true;
    }
    std::thread::spawn(move || main(hModule));
    true
}
