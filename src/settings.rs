use std::fs::File;
use std::io::Write;
use xmlwriter::{Options, XmlWriter};
use crate::settings::SettingType::{Level, Multiplier, OnHalfOff, OnOff, Slider};
use crate::settings::XMLSection::{AdvancedGraphics, Graphics, Video};

/**
Attribute()
 */

type VRamLevels = Vec<usize>;
type SelectedIndex = usize;
type Value = usize;
type MaxFactor = usize;
type Enabled = bool;
type Jump = usize;
type IsHorizontal = bool;

pub enum SettingType {
    Level(SelectedIndex, Vec<Selectable>, VRamLevels),
    OnOff(Enabled),
    OnHalfOff(Value), // off=0, on=1, half=2
    Multiplier(Value, MaxFactor),
    Slider(Value, Jump, bool),
}

#[derive(PartialEq)]
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

pub const PIXELS_PER_1MB_VRAM: f64 = 5155.0;

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
            setting_type: SettingType::Level(0, options, vram),
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
            setting_type: SettingType::Level(0, options, vram),
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
            setting_type: SettingType::Level(0, options, vram),
            section,
        }
    }
    pub fn screen_pixels(section: XMLSection, tag: &str, nice_name: &str, curr_val: usize, jump: usize, is_horizontal: bool) -> Self {
        Self {
            nice_name: nice_name.into(),
            tag: tag.into(),
            setting_type: SettingType::Slider(curr_val, jump, is_horizontal),
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
            setting_type: SettingType::Level(0, options, vram),
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
        Setting::screen_pixels(Video, "screenWidthWindowed", "Width Pixels", 1024, 8, true),
        Setting::screen_pixels(Video, "screenHeightWindowed", "Height Pixels", 768, 4, false),
        Setting::api_options(AdvancedGraphics, "API", "Graphical API"),
        Setting::on_off(AdvancedGraphics, "motionBlur", "Motion Blur"),
        Setting::on_off(AdvancedGraphics, "waterReflectionSSR", "Water Reflection SSR"),
        Setting::low_medium_high(AdvancedGraphics, "waterRefractionQuality", "Water Refraction Quality", 2, 5),
        Setting::low_medium_high(AdvancedGraphics, "waterReflectionQuality", "Water Reflection Quality", 3, 15),
        Setting::low_medium_high_ultra(AdvancedGraphics, "particleLightingQuality", "Particle Lighting Quality", 18, 0, 0),
        Setting::off_medium_high_ultra(AdvancedGraphics, "shadowSoftShadows", "Soft Shadows", 0, 0, 0),
    ];
    settings
}

pub fn get_setting_index_by_tag(settings: &[Setting], tag: &str) -> Option<usize> {
    for (i, setting) in settings.iter().enumerate() {
        if setting.tag == tag {
            return Some(i);
        }
    }
    return None;
}

pub fn commit_xml_write(settings: Vec<Setting>) {
    let opt = Options {
        use_single_quote: false, // RDR2 has double quote
        ..Options::default()
    };
    let mut xml = XmlWriter::new(opt);
    xml.write_declaration();

    xml.start_element("rage__fwuiSystemSettingsCollection");

    xml.start_element("version");
    xml.write_attribute("value", "37");
    xml.end_element();

    xml.start_element("configSource");
    xml.write_text("kSettingsConfig_Auto");
    xml.end_element();

    xml.start_element("graphics");
    write_default_graphics(&mut xml);
    for setting in settings.iter() {
        if setting.section != Graphics {
            continue;
        }
        xml.start_element(&setting.tag);
        match &setting.setting_type {
            Level(index, selectables, _) => {
                xml.write_text(&selectables[*index].config_name);
            }
            OnOff(on) => {
                let boolean = if *on { "true" } else { "false" };
                xml.write_attribute("value", boolean);
            }
            Multiplier(value, _) => {
                xml.write_attribute("value", &value.to_string());
            }
            Slider(_, _, _) => {}
            OnHalfOff(_) => {}
        }
        xml.end_element();
    }
    xml.end_element();

    xml.start_element("video");
    write_default_video(&mut xml);
    for setting in settings.iter() {
        if setting.section != Video {
            continue;
        }
        xml.start_element(&setting.tag);
        match &setting.setting_type {
            Level(index, selectables, _) => xml.write_text(&selectables[*index].config_name),
            OnOff(on) => {
                let boolean = if *on { "true" } else { "false" };
                xml.write_attribute("value", boolean);
            }
            Multiplier(value, _) => xml.write_attribute("value", &value),
            OnHalfOff(vsync) => xml.write_attribute("value", vsync),
            Slider(pixels, _, _) => xml.write_attribute("value", pixels)
        }
        xml.end_element();
    }
    xml.end_element();

    xml.start_element("advancedGraphics");
    write_default_advanced_graphics(&mut xml);
    for setting in settings.iter() {
        if setting.section != AdvancedGraphics {
            continue;
        }
        xml.start_element(&setting.tag);
        match &setting.setting_type {
            Level(index, selectables, _) => xml.write_text(&selectables[*index].config_name),
            OnOff(on) => {
                let boolean = if *on { "true" } else { "false" };
                xml.write_attribute("value", boolean);
            }
            OnHalfOff(_) => {}
            Multiplier(value, _) => {}
            Slider(pixels, _, _) => xml.write_attribute("value", pixels),
        }
        xml.end_element();
    }
    xml.end_element();

    xml.start_element("videoCardDescription");
    xml.write_text("GPU CARD NAME");
    xml.end_element();

    let content = xml.end_document();
    let mut file = File::create("system.xml").expect("Couldn't create file");
    file.write_all(content.as_bytes()).expect("Failed to write to file");
}

fn write_default_graphics(xml: &mut XmlWriter) {
    write_element("dlssIndex", "0", xml);
    write_element("dlssQuality", "5", xml);
    write_element("graphicsQualityPreset", "0.5", xml);
    write_element("hdr", "true", xml);
    write_element("hdrIntensity", "100", xml);
    write_element("hdrPeakBrightness", "1000", xml);
    write_element("hdrFilmicMode", "true", xml);
    write_element("gamma", "15", xml);
    write_element("hdrSettingsMigrated", "true", xml);
}

fn write_default_video(xml: &mut XmlWriter) {

    write_element("adapterIndex", "0", xml); // output adapter
    write_element("outputIndex", "0", xml); // output monitor

    write_element("resolutionIndexWindowed", "0", xml);
    write_element("resolutionIndex", "1", xml);

    write_element("screenWidth", "1240", xml);
    write_element("screenHeight", "720", xml);

    write_element("refreshRateIndex", "0", xml);
    write_element("refreshRateNumerator", "60", xml);
    write_element("refreshRateDenominator", "1", xml);

    write_element("windowed", "2", xml);
    write_element("pauseOnFocusLoss", "false", xml); // make selectable
    write_element("constrainMousePointer", "false", xml); // make selectable
}

fn write_default_advanced_graphics(xml: &mut XmlWriter) {
    write_element("locked", "false", xml);
    write_element("asyncComputeEnabled", "false", xml);
    write_element("transferQueuesEnabled", "true", xml);
    write_element("motionBlurLimit", "16.0", xml);
    write_element("waterSimulationQuality", "3", xml); // make selectable
    write_text_element("waterLightingQuality", "kSettingLevel_Ultra", xml); //make selectable
    write_text_element("furDisplayQuality", "kSettingLevel_Medium", xml); // make selectable
    write_element("maxTexUpgradesPerFrame", "5", xml);
    // check
    write_text_element("shadowGrassShadows", "kSettingLevel_High", xml);
    write_element("shadowParticleShadows", "true", xml);
    write_element("shadowLongShadows", "true", xml);
    write_element("directionalShadowsAlpha", "false", xml);
    write_element("worldHeightShadowQuality", "1.0", xml);
    write_element("directionalScreenSpaceShadowQuality", "1.0", xml);
    write_element("ambientMaskVolumesHighPrecision", "true", xml);
    write_text_element("scatteringVolumeQuality", "kSettingLevel_High", xml);
    write_text_element("volumetricsRaymarchQuality", "kSettingLevel_High", xml);
    write_text_element("volumetricsLightingQuality", "kSettingLevel_High", xml);
    write_element("volumetricsRaymarchResolutionUnclamped", "true", xml);
    write_text_element("terrainShadowQuality", "kSettingLevel_Ultra", xml);
    write_element("damageModelsDisabled", "false", xml); //interesting?
    write_text_element("decalQuality", "kSettingLevel_High", xml);
    write_element("ssaoFullScreenEnabled", "false", xml);
    write_element("ssaoType", "0", xml);
    write_element("ssdoSampleCount", "4", xml);
    write_element("ssdoUseDualRadii", "false", xml);
    write_text_element("ssdoResolution", "kSettingLevel_Low", xml);
    write_element("ssdoTAABlendEnabled", "true", xml);
    write_element("ssroSampleCount", "2", xml);
    write_element("snowGlints", "true", xml);
    write_text_element("POMQuality", "kSettingLevel_Ultra", xml);
    write_element("probeRelightEveryFrame", "false", xml);
    write_text_element("scalingMode", "kSettingScale_Mode1o1", xml);
    write_element("reflectionMSAA", "0", xml);
    write_element("lodScale", "1.0", xml);
    write_element("grassLod", "3.0", xml);
    write_element("pedLodBias", "0", xml);
    write_element("vehicleLodBias", "0", xml);
    write_element("sharpenIntensity", "1", xml);
    write_element("sharpenIntensity", "1", xml);
    write_text_element("treeQuality", "kSettingLevel_Ultra", xml);
    write_text_element("deepsurfaceQuality", "kSettingLevel_High", xml);
    write_element("treeTessellationEnabled", "false", xml);
}

fn write_element(name: &str, val: &str, xml: &mut XmlWriter) {
    xml.start_element(name);
    xml.write_attribute("value", val);
    xml.end_element();
}

fn write_text_element(name: &str, text: &str, xml: &mut XmlWriter) {
    xml.start_element(name);
    xml.write_text(text);
    xml.end_element();
}