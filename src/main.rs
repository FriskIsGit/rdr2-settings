use std::io::{Write};
use crate::inputs::KeyCode;
use crate::settings::{Setting, SettingType};

#[cfg(target_os = "windows")]
use crate::inputs::windows::{read_key, clear_console};

#[cfg(all(unix))]
use crate::inputs::unix::{read_key, clear_console};
use crate::settings::SettingType::{Level, Multiplier, OnHalfOff, OnOff, Slider};

mod settings;
mod inputs;

// Highest low [test on minimal settings (1024 x 768)] claimed=1537
const MIN_VRAM: usize = 1670;
const RECOMMENDED_VRAM: usize = 6144;
const PADDING: usize = 40;

fn main() {
    println!("Running!");
    // key_testing();
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
    let mut settings = settings::get_settings();
    let capacity = settings_string_capacity(&settings);
    // hacky code since both settings are dependent on each other
    let width_index = settings::get_setting_index_by_tag(&settings, "screenWidthWindowed").unwrap();
    let height_index = settings::get_setting_index_by_tag(&settings, "screenHeightWindowed").unwrap();

    let mut index = 0;
    let mut cycle_settings = true;
    let mut vram_used = MIN_VRAM as f64;
    while cycle_settings {
        let mut format = String::with_capacity(capacity);
        format.push_str(&format!("==== VRAM USAGE {vram_used:.0} / {vram_available_mbs} ====\n"));

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
        clear_console();
        println!("{format}");

        let key = read_key();
        match key {
            KeyCode::Backspace => {
                cycle_settings = false;
            }
            KeyCode::ArrowUp | KeyCode::Char('w') | KeyCode::Char('W') => {
                if index > 0 {
                    index -= 1;
                }
            }
            KeyCode::ArrowDown | KeyCode::Char('s') | KeyCode::Char('S') => {
                if index + 1 < settings.len() {
                    index += 1;
                }
            }
            KeyCode::ArrowLeft | KeyCode::Char('a') | KeyCode::Char('A') => {
                let Slider(width, _, _) = settings[width_index].setting_type else {
                    panic!("Width not found?");
                };
                let Slider(height, _, _) = settings[height_index].setting_type else {
                    panic!("Height not found?");
                };
                match &mut settings[index].setting_type {
                    Level(selected_index, _, vram_levels) => {
                        if *selected_index > 0 {
                            *selected_index -= 1;
                            vram_used -= vram_levels[*selected_index] as f64;
                        }
                    }
                    OnOff(enabled) => {
                        *enabled = !*enabled;
                    }
                    Multiplier(value, _) => {
                        if *value <= 2 {
                            *value = 0;
                        } else {
                            *value /= 2;
                        }
                    }
                    OnHalfOff(value) => {
                        // OFF ON HALF
                        if *value == 0 {
                            *value = 2;
                        } else if *value == 1 {
                            *value = 0;
                        } else if *value == 2 {
                            *value = 1;
                        }
                    }
                    Slider(value, jump, horizontal) => {
                        let jmp = *jump;
                        if *value < jmp * 2 {
                            continue;
                        }
                        *value -= jmp;
                        let pixel_diff = match *horizontal {
                            true => (height * jmp) as f64,
                            false => (width * jmp) as f64
                        };
                        let mb_diff = pixel_diff / settings::PIXELS_PER_1MB_VRAM;
                        vram_used -= mb_diff;
                    }
                }
            }
            KeyCode::ArrowRight | KeyCode::Char('d') | KeyCode::Char('D') => {
                let Slider(width, _, _) = settings[width_index].setting_type else {
                    panic!("Width not found?");
                };
                let Slider(height, _, _) = settings[height_index].setting_type else {
                    panic!("Height not found?");
                };
                match &mut settings[index].setting_type {
                    Level(selected_index, selectable, vram_levels) => {
                        if *selected_index + 1 < selectable.len() {
                            *selected_index += 1;
                            vram_used += vram_levels[*selected_index - 1] as f64;
                        }
                    }
                    OnOff(enabled) => {
                        *enabled = !*enabled;
                    }
                    Multiplier(value, max_factor) => {
                        if *value == 0 {
                            *value = 2;
                        } else if value < max_factor && *value >= 2 {
                            *value *= 2;
                        }
                    }
                    OnHalfOff(value) => {
                        // OFF ON HALF
                        if *value == 0 {
                            *value = 1;
                        } else if *value == 1 {
                            *value = 2;
                        } else if *value == 2 {
                            *value = 0;
                        }
                    }
                    Slider(value, jump, horizontal) => {
                        let jmp = *jump;
                        *value += jmp;
                        let pixel_diff = match *horizontal {
                            true => (height * jmp) as f64,
                            false => (width * jmp) as f64
                        };
                        let mb_diff = pixel_diff / settings::PIXELS_PER_1MB_VRAM;
                        vram_used += mb_diff;
                        //add vram reduction logic
                    }
                }
            }
            KeyCode::Enter => {
                println!("Committing write");
                settings::commit_xml_write(settings);
                break;
            }
            _ => {}
        }
    }
}

fn append_setting_type(format: &mut String, setting_type: &SettingType) {
    match setting_type {
        Level(selected_index, selectable, _) => {
            let option = &selectable[*selected_index];
            format.push_str(&option.nice_name);
        }
        OnOff(enabled) => {
            if *enabled {
                format.push_str("ON");
            } else {
                format.push_str("OFF");
            }
        }
        Multiplier(value, _) => {
            if *value == 0 {
                format.push_str("OFF");
            } else {
                format.push('X');
                format.push_str(&value.to_string());
            }
        }
        OnHalfOff(value) => {
            if *value == 0 {
                format.push_str("OFF");
            } else if *value == 1 {
                format.push_str("ON");
            } else if *value == 2 {
                format.push_str("HALF");
            }
        }
        Slider(value, _, _) => {
            format.push_str(&value.to_string());
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

fn key_testing() {
    let mut looping = true;
    while looping {
        let key = read_key();
        match key {
            KeyCode::Char(chr) => println!("{chr}"),
            KeyCode::Enter => println!("ENTER"),
            KeyCode::ArrowUp => println!("ARROW U"),
            KeyCode::ArrowDown => println!("ARROW D"),
            KeyCode::ArrowRight => println!("ARROW R"),
            KeyCode::ArrowLeft => println!("ARROW L"),
            KeyCode::Backspace => println!("BACKSPACING"),
            KeyCode::Other(id) => println!("{id}"),
            _ => {}
        }
    }
}
