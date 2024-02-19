
#[derive(Debug)]
pub enum KeyCode {
    Ascii(u8),
    ArrowUp,
    ArrowDown,
    ArrowRight,
    ArrowLeft,
    Other,
}

#[cfg(target_os = "windows")]
pub(crate) mod windows {
    use super::KeyCode;

    #[allow(non_camel_case_types)]
    type void = std::ffi::c_void;

    const STD_INPUT_HANDLE:  u32 = -10i32 as u32;
    const ENABLE_LINE_INPUT: u32 = 0x0002;
    const ENABLE_ECHO_INPUT: u32 = 0x0004;

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Coord {
        x: i16,
        y: i16,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct EventKey {
        key_down:          i32,
        repeat_count:      u16,
        virtual_keycode:   u16,
        virtual_scancode:  u16,
        // character_data:    u16,
        character_data:    u8,
        control_key_state: u32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct EventMouse {
        mouse_position:     Coord,
        button_state:       u32,
        control_key_state : u32,
        event_flags:        u32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    union Event {
        focus: i32,
        key:   EventKey,
        menu:  u32,
        mouse: EventMouse,
        size:  Coord,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct InputRecord {
        event_type: u16,
        event: Event,
    }

    impl Default for InputRecord {
        fn default() -> Self {
            Self {
                event_type: 0,
                event: Event { focus: 0 }
            }
        }
    }

    extern "system" {
        fn GetStdHandle(std_handle_code: u32) -> *const void;
        fn GetConsoleMode(console_handle: *const void, console_mode: *mut u32) -> i32;
        fn SetConsoleMode(console_handle: *const void, console_mode: u32) -> i32;
        fn ReadConsoleInputA(console_handle: *const void, buffer: *mut InputRecord, buffer_length: i32, entries_read: *mut u32) -> i32;
        fn FlushConsoleInputBuffer(console_handle: *const void) -> i32;
    }

    pub fn read_key() -> KeyCode {
        unsafe {
            let handle = GetStdHandle(STD_INPUT_HANDLE);
            if handle == std::ptr::null() {
                // STDIN console handle not found
                return KeyCode::Other;
            }

            FlushConsoleInputBuffer(handle);

            let mut prev_mode = 0;
            GetConsoleMode(handle, &mut prev_mode as *mut u32);

            let new_mode = prev_mode & !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT);
            SetConsoleMode(handle, new_mode);

            let mut entries_read = 0u32;
            let mut input = InputRecord::default();
            loop {
                let result = ReadConsoleInputA(handle, &mut input as *mut InputRecord, 1, &mut entries_read as *mut u32);

                // Reading the console input failed.
                if result == 0 || entries_read != 1 {
                    SetConsoleMode(handle, prev_mode);
                    return KeyCode::Other
                }

                if input.event_type != 1 {
                    continue;
                }

                let key = input.event.key;
                if key.key_down == 0 {
                    continue;
                }

                break;
            }

            SetConsoleMode(handle, prev_mode);

            let key = input.event.key;

            if key.character_data != 0 {
                return KeyCode::Ascii(key.character_data);
            }

            // TODO: Special keys are not handled.

            match key.virtual_keycode {
                0x25 => return KeyCode::ArrowLeft,
                0x26 => return KeyCode::ArrowUp,
                0x27 => return KeyCode::ArrowRight,
                0x28 => return KeyCode::ArrowDown,
                _    => return KeyCode::Other,
            }
        }
    }
}



#[cfg(all(unix))]
pub(crate) mod unix {
    use super::KeyCode;

    const STDIN_FILENO: i32 = 0;
    const TCSANOW:      i32 = 0;
    const ICANON:       i32 = 2;
    const ECHO:         i32 = 10;

    #[repr(C)]
    #[derive(Default, Copy, Clone)]
    struct Termios {
        input_mode:          i32,       // tcflag_t c_iflag
        output_mode:         i32,       // tcflag_t c_oflag
        control_mode:        i32,       // tcflag_t c_cflag
        local_mode:          i32,       // tcflag_t c_lflag
        line_discipline:     i32,       // cc_t     c_line
        control_characters: [i32; 32],  // cc_t     c_cc[32]
        input_speed:         i32,       // speed_t  c_ispeed
        output_speed:        i32,       // speed_t  c_ospeed
    }

    extern "C" {
        fn tcgetattr(file_descriptor: i32, termios: *mut Termios) -> i32;
        fn tcsetattr(file_descriptor: i32, optional_actions: i32, termios: *const Termios) -> i32;
        fn read(file_descriptor: i32, buffer: *mut std::ffi::c_void, buffer_size: i32) -> i32;
    }

    pub fn read_key() -> KeyCode {
        unsafe {
            let mut old_settings = Termios::default();
            tcgetattr(STDIN_FILENO, &mut old_settings as *mut Termios);

            let mut new_settings = old_settings;
            new_settings.local_mode &= !(ICANON | ECHO);

            tcsetattr(STDIN_FILENO, TCSANOW, &new_settings as *const Termios);

            let mut data: u32 = 0;
            let _ = read(0, &mut data as *mut u32 as *mut std::ffi::c_void, 4);

            tcsetattr(STDIN_FILENO, TCSANOW, &old_settings as *const Termios);

            match data {
                65..=90 | 97..=122 => return KeyCode::Ascii(data as u8),
                4479771 => return KeyCode::ArrowLeft,
                4283163 => return KeyCode::ArrowUp,
                4414235 => return KeyCode::ArrowRight,
                4348699 => return KeyCode::ArrowDown,
                _ => return KeyCode::Other,
            }
        }
    }
}
