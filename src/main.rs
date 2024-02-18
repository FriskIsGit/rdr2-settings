use getch_rs::{Getch, Key};
use crate::settings::{Setting, SettingType};

mod settings;
mod console;

const UP: char = 'w';
const DOWN: char = 's';
const LEFT: char = 'a';
const RIGHT: char = 'd';
const CARRIAGE_RETURN: char = '\r';
const LINE_BREAK: char = '\n';

const BASE_VRAM: usize = 1024;
const ALL_VRAM: usize = 2048;
const PADDING: usize = 40;

fn main() {
    println!("Running!");
    start_console();
}

fn start_console() {
    let mut settings = settings::get_settings();
    let capacity = settings_string_capacity(&settings);
    let g = Getch::new();
    let mut index = 0;
    let mut cycle_settings = true;

    let mut vram_used = BASE_VRAM;
    while cycle_settings {
        let mut format = String::with_capacity(capacity);
        format.push_str(&format!("==== VRAM USAGE {vram_used} / {ALL_VRAM} ====\n"));

        for (i, setting) in settings.iter().enumerate() {
            if i == index {
                format.push_str(" > ");
            } else {
                format.push_str("   ");
            }
            let nice_name_length = setting.nice_name.len();
            format.push_str(&setting.nice_name);
            pad_with_spaces(&mut format, PADDING - nice_name_length);
            append_setting_type(&mut format, &setting.setting_type);
        }
        println!("{format}");
        let Ok(key) = g.getch() else {
            continue;
        };
        console::clear_screen();
        match key {
            // BACKSPACE IS NOT DETECTED as BACKSPACE but as DELETE
            Key::Delete | Key::Backspace => {
                cycle_settings = false;
            }
            // Arrow keys are not detected
            Key::Char(chr) => {
                match chr {
                    UP => {
                        if index > 0 {
                            index -= 1;
                        }
                    }
                    DOWN => {
                        if index + 1 < settings.len() {
                            index += 1;
                        }
                    }
                    LEFT => {
                        let setting = &mut settings[index];
                        match &mut setting.setting_type {
                            SettingType::Level(selected_index, _, vram_levels) => {
                                if *selected_index > 0 {
                                    *selected_index -= 1;
                                    vram_used -= vram_levels[*selected_index];
                                }
                            }
                            SettingType::OnOff(enabled) => {
                                *enabled = !*enabled;
                            }
                            SettingType::Multiplier(value, _) => {
                                if *value <= 2 {
                                    *value = 0;
                                } else {
                                    *value /= 2;
                                }
                            }
                            SettingType::OnHalfOff(value) => {
                                // OFF ON HALF
                                if *value == 0 {
                                    *value = 2;
                                } else if *value == 1 {
                                    *value = 0;
                                } else if *value == 2 {
                                    *value = 1;
                                }
                            }
                        }
                    }
                    RIGHT => {
                        let setting = &mut settings[index];
                        match &mut setting.setting_type {
                            SettingType::Level(selected_index, selectable, vram_levels) => {
                                if *selected_index + 1 < selectable.len() {
                                    *selected_index += 1;
                                    vram_used += vram_levels[*selected_index - 1];
                                }
                            }
                            SettingType::OnOff(enabled) => {
                                *enabled = !*enabled;
                            }
                            SettingType::Multiplier(value, max_factor) => {
                                if *value == 0 {
                                    *value = 2;
                                } else if value < max_factor && *value >= 2 {
                                    *value *= 2;
                                }
                            }
                            SettingType::OnHalfOff(value) => {
                                // OFF ON HALF
                                if *value == 0 {
                                    *value = 1;
                                } else if *value == 1 {
                                    *value = 2;
                                } else if *value == 2 {
                                    *value = 0;
                                }
                            }
                        }
                    }
                    CARRIAGE_RETURN | LINE_BREAK => {}
                    _ => {
                        // println!("ANOTHER CHAR: {chr}")
                    }
                }
            }
            _ => {}
        }
    }
}

fn append_setting_type(format: &mut String, setting_type: &SettingType) {
    match setting_type {
        SettingType::Level(selected_index, selectable, vram_levels) => {
            let option = &selectable[*selected_index];
            format.push_str(&option.nice_name);
        }
        SettingType::OnOff(enabled) => {
            if *enabled {
                format.push_str("ON");
            } else {
                format.push_str("OFF");
            }
        }
        SettingType::Multiplier(value, _) => {
            if *value == 0 {
                format.push_str("OFF");
            } else {
                format.push('X');
                format.push_str(&value.to_string());
            }
        }
        SettingType::OnHalfOff(value) => {
            if *value == 0 {
                format.push_str("OFF");
            } else if *value == 1 {
                format.push_str("ON");
            } else if *value == 2 {
                format.push_str("HALF");
            }
        }
    }

    format.push('\n');
}

fn pad_with_spaces(str: &mut String, spaces: usize) {
    for _ in 0..spaces {
        str.push(' ');
    }
}

fn settings_string_capacity(settings: &[Setting]) -> usize {
    let mut capacity = 0;
    for setting in settings {
        capacity += setting.nice_name.len();
    }
    capacity
}


