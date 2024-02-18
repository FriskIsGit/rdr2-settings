use crate::settings::XMLSection::{Graphics, Video};

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
    OnHalfOff(Value), // off=0, on=1, half=2
    Multiplier(Value, MaxFactor),
}

pub enum XMLSection {
    Graphics,
    Video,
    AdvancedGraphics,
}

pub struct Setting {
    pub tag: String,
    pub nice_name: String,
    pub setting_type: SettingType,
    pub section: XMLSection,
}

impl Setting {
    pub fn off_medium_high(section: XMLSection, tag: &str, nice_name: &str, step1: usize, step2: usize) -> Self {
        let off = Selectable::new("Low (OFF)".into(), "kSettingLevel_Low".into()); // ??
        let medium = Selectable::new("Medium".into(), "kSettingLevel_Medium".into());
        let high = Selectable::new("High".into(), "kSettingLevel_High".into());
        let options = vec![off, medium, high];
        let vram = vec![step1, step2];
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::Level(1, options, vram),
            section,
        }
    }
    pub fn off_medium_high_ultra(section: XMLSection, tag: &str, nice_name: &str, step1: usize, step2: usize, step3: usize) -> Self {
        let off = Selectable::new("Low (OFF)".into(), "kSettingLevel_Low".into()); // ??
        let medium = Selectable::new("Medium".into(), "kSettingLevel_Medium".into());
        let high = Selectable::new("High".into(), "kSettingLevel_High".into());
        let ultra = Selectable::new("Ultra".into(), "kSettingLevel_Ultra".into());
        let options = vec![off, medium, high, ultra];
        let vram = vec![step1, step2, step3];
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::Level(1, options, vram),
            section,
        }
    }
    pub fn on_off(section: XMLSection, tag: &str, nice_name: &str) -> Self {
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::OnOff(false),
            section,
        }
    }
    pub fn on_half_off(section: XMLSection, tag: &str, nice_name: &str) -> Self {
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::OnHalfOff(0),
            section,
        }
    }
    pub fn low_medium_high_ultra_no_step(section: XMLSection, tag: &str, nice_name: &str) -> Self {
        Self::low_medium_high_ultra(section, tag, nice_name, 0, 0, 0)
    }
    pub fn low_medium_high_ultra(section: XMLSection, tag: &str, nice_name: &str, step1: usize, step2: usize, step3: usize) -> Self {
        let low = Selectable::new("Low".into(), "kSettingLevel_Low".into());
        let medium = Selectable::new("Medium".into(), "kSettingLevel_Medium".into());
        let high = Selectable::new("High".into(), "kSettingLevel_High".into());
        let ultra = Selectable::new("Ultra".into(), "kSettingLevel_Ultra".into());
        let options = vec![low, medium, high, ultra];
        let vram = vec![step1, step2, step3];
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::Level(2, options, vram),
            section,
        }
    }
    pub fn api_options(section: XMLSection, tag: &str, nice_name: &str) -> Self {
        let vulcan = Selectable::new("Vulcan".into(), "kSettingAPI_Vulcan".into());
        let drx12 = Selectable::new("DirectX12".into(), "kSettingAPI_DX12".into());
        let options = vec![vulcan, drx12];
        let vram = vec![0, 0];
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::Level(0, options, vram),
            section,
        }
    }
    pub fn low_medium_high_no_step(section: XMLSection, tag: &str, nice_name: &str) -> Self {
        Self::low_medium_high(section, tag, nice_name, 0, 0)
    }
    pub fn multiplier(section: XMLSection, tag: &str, nice_name: &str, max_factor: usize) -> Self {
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::Multiplier(0, max_factor),
            section,
        }
    }
    pub fn low_medium_high(section: XMLSection, tag: &str, nice_name: &str, step1: usize, step2: usize) -> Self {
        let low = Selectable::new("Low".into(), "kSettingLevel_Low".into());
        let medium = Selectable::new("Medium".into(), "kSettingLevel_Medium".into());
        let high = Selectable::new("High".into(), "kSettingLevel_High".into());
        let options = vec![low, medium, high];
        let vram = vec![step1, step2];
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::Level(1, options, vram),
            section,
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

pub fn get_settings() -> Vec<Setting> {
    // Settings maintaining in-game order
    let settings = vec![
        Setting::low_medium_high_ultra(Graphics, "textureQuality", "Texture Quality", 164, 185, 726),
        Setting::multiplier(Graphics, "anisotropicFiltering", "Anisotropic Filtering", 16),
        Setting::low_medium_high_ultra_no_step(Graphics, "lightingQuality", "Lighting Quality"),
        Setting::low_medium_high_ultra(Graphics, "ambientLightingQuality", "Ambient Lighting Quality", 1, 9, 0),
        Setting::low_medium_high_ultra(Graphics, "shadowQuality", "Shadow Quality", 72, 11, 333),
        Setting::low_medium_high_ultra(Graphics, "farShadowQuality", "Far Shadow Quality", 0, 1, 1),
        Setting::off_medium_high_ultra(Graphics, "ssao", "Screen Space Ambient Occlusion", 11, 4, 0), // Off is actually Low
        Setting::low_medium_high_ultra(Graphics, "reflectionQuality", "Reflection Quality", 29, 111, 460),
        Setting::low_medium_high_ultra(Graphics, "mirrorQuality", "Mirror Quality", 0, 9, 14),
        Setting::low_medium_high(Graphics, "waterQuality", "Water Quality", 13, 92),
        Setting::low_medium_high_ultra(Graphics, "volumetricsQuality", "Volumetrics Quality", 16, 67, 111),
        Setting::low_medium_high_ultra_no_step(Graphics, "particleQuality", "Particle Quality"),
        Setting::low_medium_high_ultra_no_step(Graphics, "tessellation", "Tessellation Quality"),
        Setting::off_medium_high(Graphics, "taa", "TAA Quality", 6, 0), // Off is actually Low
        Setting::on_off(Graphics, "fxaaEnabled", "FXAA"),
        Setting::multiplier(Graphics, "msaa", "MSAA", 8),

        Setting::on_off(Video, "tripleBuffered", "Triple Buffering"),
        Setting::on_half_off(Video, "vSync", "VSync"),
        Setting::api_options(Video, "API", "Graphical API"),
    ];
    settings
}
