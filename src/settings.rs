/**
Attribute()
 */

type VRamLevels = Vec<usize>;
type SelectedIndex = usize;
type Value = usize;
type MaxFactor = usize;
type Enabled = bool;

pub enum SettingType {
    Level(SelectedIndex, Vec<Selectable>, VRamLevels),
    OnOff(Enabled),
    Multiplier(Value, MaxFactor),
}

pub struct Setting {
    pub tag: String,
    pub nice_name: String,
    pub setting_type: SettingType,
}

impl Setting {
    pub fn off_medium_high(tag: &str, nice_name: &str, step1: usize, step2: usize) -> Self {
        let vram = vec![step1, step2];
        let off = Selectable::new("Off(Low)".into(), "kSettingLevel_Low".into()); // ??
        let medium = Selectable::new("Medium".into(), "kSettingLevel_Medium".into());
        let high = Selectable::new("High".into(), "kSettingLevel_High".into());
        let options = vec![off, medium, high];
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::Level(1, options, vram),
        }
    }
    pub fn off_on(tag: &str, nice_name: &str) -> Self {
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::OnOff(false),
        }
    }
    pub fn low_medium_high_ultra_no_step(tag: &str, nice_name: &str) -> Self {
        Self::low_medium_high_ultra(tag, nice_name, 0, 0, 0)
    }
    pub fn low_medium_high_ultra(tag: &str, nice_name: &str, step1: usize, step2: usize, step3: usize) -> Self {
        let vram = vec![step1, step2, step3];
        let low = Selectable::new("Low".into(), "kSettingLevel_Low".into());
        let medium = Selectable::new("Medium".into(), "kSettingLevel_Medium".into());
        let high = Selectable::new("High".into(), "kSettingLevel_High".into());
        let ultra = Selectable::new("Ultra".into(), "kSettingLevel_Ultra".into());
        let options = vec![low, medium, high, ultra];
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::Level(2, options, vram),
        }
    }
    pub fn low_medium_high_no_step(tag: &str, nice_name: &str) -> Self {
        Self::low_medium_high(tag, nice_name, 0, 0)
    }
    pub fn multiplier(tag: &str, nice_name: &str, max_factor: usize) -> Self {
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::Multiplier(0, max_factor),
        }
    }
    pub fn low_medium_high(tag: &str, nice_name: &str, step1: usize, step2: usize) -> Self {
        let vram = vec![step1, step2];
        let low = Selectable::new("Low".into(), "kSettingLevel_Low".into());
        let medium = Selectable::new("Medium".into(), "kSettingLevel_Medium".into());
        let high = Selectable::new("High".into(), "kSettingLevel_High".into());
        let options = vec![low, medium, high];
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::Level(1, options, vram),
        }
    }
}

pub struct Selectable {
    pub nice_name: String,
    pub config_name: String,
}

impl Selectable {
    pub fn new(nice_name: String, config_name: String) -> Self {
        Self { nice_name, config_name }
    }
}

pub fn get_graphics_settings() -> Vec<Setting> {
    let settings = vec![
        Setting::low_medium_high_ultra("textureQuality", "Texture Quality", 164, 185, 726),
        Setting::low_medium_high_ultra_no_step("lightingQuality", "Lighting Quality"),
        Setting::low_medium_high_ultra("ambientLightingQuality", "Ambient Lighting Quality(Global)", 2, 9, 0),
        Setting::low_medium_high_ultra("shadowQuality", "Shadow Quality", 72, 11, 333),
        Setting::low_medium_high_ultra("farShadowQuality", "Far Shadow Quality", 0, 1, 1),
        Setting::low_medium_high_ultra("reflectionQuality", "Reflection Quality", 29, 111, 460),
        Setting::low_medium_high_ultra("mirrorQuality", "Mirror Quality", 0, 9, 14),
        Setting::low_medium_high_ultra("volumetricsQuality", "Volumetrics Quality", 16, 67, 111),
        Setting::low_medium_high_ultra_no_step("particleQuality", "Particle Quality", ),
        Setting::low_medium_high_ultra_no_step("tessellation", "Tessellation Quality"),
        Setting::off_medium_high("taa", "TAA Quality", 6, 0), // Off is actually Low
        Setting::low_medium_high("waterQuality", "Water Quality", 13, 92),
        Setting::off_on("fxaaEnabled", "FXAA"),
        Setting::multiplier("msaa>", "MSAA", 8),
    ];
    settings
}

pub fn get_advanced_graphics_settings() -> Vec<Setting> {
    let settings = vec![];
    settings
}

pub fn get_video_settings() -> Vec<Setting> {
    let settings = vec![];
    settings
}