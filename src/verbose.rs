// ©Arthurdw
pub struct Verbose {
    pub intensity: VerboseIntensity,
    pub intensity_value: u64,
}

impl Verbose {
    pub fn from_intensity_value(intensity_value: u64) -> Verbose {
        Verbose {
            intensity: match intensity_value {
                1 => VerboseIntensity::DEBUG,
                2 => VerboseIntensity::CRITICAL,
                3 => VerboseIntensity::ERROR,
                4 => VerboseIntensity::WARNING,
                5 => VerboseIntensity::INFO,
                0 | _ => VerboseIntensity::CRITICAL,
            },
            intensity_value: if intensity_value == 0 {
                2
            } else {
                intensity_value
            },
        }
    }
}

pub enum VerboseIntensity {
    INFO,
    WARNING,
    ERROR,
    CRITICAL,
    DEBUG,
}
