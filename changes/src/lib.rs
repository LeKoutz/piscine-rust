#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
	pub fn new(alias: &str) -> Self {
        let new = Light{alias: alias.to_string(), brightness: 0};
        return new;
	}
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for light in lights {
        if light.alias == alias {
            light.brightness = value
        }
    }
}
