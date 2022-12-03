use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Weather {
    ExtraSunnyLA,
    SunnyLA,
    ExtraSunnySmogLA,
    SunnySmogLA,
    CloudyLA,
    SunnySF,
    ExtraSunnySF,
    CloudySF,
    RainySF,
    FoggySF,
    SunnyVegas,
    ExtraSunnyVegas,
    CloudyVegas,
    ExtraSunnyCountrySide,
    SunnyCountrySide,
    CloudyCountrySide,
    RainyCountrySide,
    ExtraSunnyDesert,
    SunnyDesert,
    SandstormDesert,
    Underwater,
    ExtraColours1,
    ExtraColours2,
}

impl From<i16> for Weather {
    fn from(value: i16) -> Self {
        match value {
            0 => Weather::ExtraSunnyLA,
            1 => Weather::SunnyLA,
            2 => Weather::ExtraSunnySmogLA,
            3 => Weather::SunnySmogLA,
            4 => Weather::CloudyLA,
            5 => Weather::SunnySF,
            6 => Weather::ExtraSunnySF,
            7 => Weather::CloudySF,
            8 => Weather::RainySF,
            9 => Weather::FoggySF,
            10 => Weather::SunnyVegas,
            11 => Weather::ExtraSunnyVegas,
            12 => Weather::CloudyVegas,
            13 => Weather::ExtraSunnyCountrySide,
            14 => Weather::SunnyCountrySide,
            15 => Weather::CloudyCountrySide,
            16 => Weather::RainyCountrySide,
            17 => Weather::ExtraSunnyDesert,
            18 => Weather::SunnyDesert,
            19 => Weather::SandstormDesert,
            20 => Weather::Underwater,
            21 => Weather::ExtraColours1,
            22 => Weather::ExtraColours2,
            _ => panic!("Unknown weather value: {}", value),
        }
    }
}

pub fn weather() -> Weather {
    unsafe { *(0xC81320 as *const i16) }.into()
}

pub fn set_weather(weather: Weather) {
    unsafe {
        *(0xC81320 as *mut i16) = weather as i16;
    }
}
