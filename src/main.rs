use std::io::{stdout, Write};
use crossterm::event::{Event, KeyCode, read};
use crossterm::execute;
use crate::settings::{Setting, SettingType};

mod settings;

const UP: char = 'w';
const DOWN: char = 's';
const LEFT: char = 'a';
const RIGHT: char = 'd';
const CARRIAGE_RETURN: char = '\r';
const LINE_BREAK: char = '\n';

const MIN_VRAM: usize = 1537;  // All minimal settings (1024 x 768)
const RECOMMENDED_VRAM: usize = 6144;
const PADDING: usize = 40;

fn main() {
    // println!("Running!");
    let args: Vec<String> = std::env::args().collect();
    let mut vram_available = RECOMMENDED_VRAM;
    if args.len() > 0 {
        // VRAM provided in GB
        if let Ok(gb) = args[0].parse::<usize>() {
            vram_available = gb * 1024;
        }
    }
    start_console(vram_available);
}

fn start_console(vram_available_mbs: usize) {
    let Ok(_) = crossterm::terminal::enable_raw_mode() else {
        eprintln!("Failed to enable raw mode");
        return;
    };

    let mut settings = settings::get_settings();
    let capacity = settings_string_capacity(&settings);
    let mut index = 0;
    let mut cycle_settings = true;
    let mut inputs: Vec<char> = vec![];
    let mut vram_used = MIN_VRAM as f64;
    while cycle_settings {
        let mut format = String::with_capacity(capacity);
        format.push_str(&format!("==== VRAM USAGE {vram_used} / {vram_available_mbs} ====\n"));

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
        let Event::Key(event) = read().unwrap() else {
            continue;
        };

        match event.code {
            KeyCode::Backspace  => {
                cycle_settings = false;
            }
            KeyCode::Up | KeyCode::Char('w') => {
                inputs.push('w');
                if index > 0 {
                    index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('s') => {
                if index + 1 < settings.len() {
                    index += 1;
                }
            }
            KeyCode::Left | KeyCode::Char('a') => {
                let setting = &mut settings[index];
                match &mut setting.setting_type {
                    SettingType::Level(selected_index, _, vram_levels) => {
                        if *selected_index > 0 {
                            *selected_index -= 1;
                            vram_used -= vram_levels[*selected_index] as f64;
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
            KeyCode::Right | KeyCode::Char('d') => {
                let setting = &mut settings[index];
                match &mut setting.setting_type {
                    SettingType::Level(selected_index, selectable, vram_levels) => {
                        if *selected_index + 1 < selectable.len() {
                            *selected_index += 1;
                            vram_used += vram_levels[*selected_index - 1] as f64;
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
            KeyCode::Enter => {
                cycle_settings = false;
                break;
            },
            _ => {}
        }
    }
    println!("{:?}", inputs);
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


