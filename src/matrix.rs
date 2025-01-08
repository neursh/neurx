use embassy_rp::gpio::{ Input, Output };

use crate::keycode::Keycodes;

pub struct MatrixLayout<'a> {
    columns: [Output<'a>; 16],
    rows: [Input<'a>; 6],
    mapping: KeycodeMapping,
    modifiers_list: [u8; 8],
}

impl<'a> MatrixLayout<'a> {
    pub fn new(columns: [Output<'a>; 16], rows: [Input<'a>; 6]) -> Self {
        MatrixLayout {
            columns,
            rows,
            mapping: KeycodeMapping::default(),
            modifiers_list: [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80],
        }
    }

    pub fn scan(&mut self) -> [u8; 8] {
        let mut available_space = 6;
        let mut report: [u8; 8] = [0u8; 8];

        for column_data in self.columns.iter_mut().enumerate() {
            column_data.1.set_high();

            for row_data in self.rows.iter().enumerate() {
                if row_data.1.is_high() {
                    let key = self.mapping.get_key(row_data.0, column_data.0);

                    if self.modifiers_list.contains(&key) {
                        report[0] += key;
                        continue;
                    }

                    if available_space == 1 {
                        report[7] = 0x01;
                        break;
                    }

                    report[8 - available_space] = key;
                    available_space -= 1;
                }
            }

            column_data.1.set_low();

            if available_space == 1 {
                break;
            }
        }

        report
    }
}

struct KeycodeMapping {
    scan_mapping: [[u8; 16]; 6],
}

impl Default for KeycodeMapping {
    fn default() -> Self {
        let code = Keycodes::default();
        KeycodeMapping {
            scan_mapping: [
                [
                    code.KEY_ESC,
                    code.KEY_F1,
                    code.KEY_F2,
                    code.KEY_F3,
                    code.KEY_F4,
                    code.KEY_F5,
                    code.KEY_F6,
                    code.KEY_F7,
                    code.KEY_F8,
                    code.KEY_F9,
                    code.KEY_F10,
                    code.KEY_F11,
                    code.KEY_F12,
                    code.KEY_P1,
                    code.KEY_P2,
                    code.KEY_SYSRQ,
                ],
                [
                    code.KEY_GRAVE,
                    code.KEY_1,
                    code.KEY_2,
                    code.KEY_3,
                    code.KEY_4,
                    code.KEY_5,
                    code.KEY_6,
                    code.KEY_7,
                    code.KEY_8,
                    code.KEY_9,
                    code.KEY_0,
                    code.KEY_MINUS,
                    code.KEY_EQUAL,
                    code.KEY_NONE, // Column 13 skips row 1.
                    code.KEY_BACKSPACE,
                    code.KEY_DELETE,
                ],
                [
                    code.KEY_TAB,
                    code.KEY_NONE, // Column 1 skips row 2.
                    code.KEY_Q,
                    code.KEY_W,
                    code.KEY_E,
                    code.KEY_R,
                    code.KEY_T,
                    code.KEY_Y,
                    code.KEY_U,
                    code.KEY_I,
                    code.KEY_O,
                    code.KEY_P,
                    code.KEY_LEFTBRACE,
                    code.KEY_RIGHTBRACE,
                    code.KEY_BACKSLASH,
                    code.KEY_INSERT,
                ],
                [
                    code.KEY_CAPSLOCK,
                    code.KEY_NONE, // Column 1 skips row 3.
                    code.KEY_A,
                    code.KEY_S,
                    code.KEY_D,
                    code.KEY_F,
                    code.KEY_G,
                    code.KEY_H,
                    code.KEY_J,
                    code.KEY_K,
                    code.KEY_L,
                    code.KEY_SEMICOLON,
                    code.KEY_APOSTROPHE,
                    code.KEY_ENTER,
                    code.KEY_NONE, // Column 14 skips row 3.
                    code.KEY_P3,
                ],
                [
                    code.KEY_NONE, // Column 0 skips row 4.
                    code.KEY_MOD_LSHIFT,
                    code.KEY_Z,
                    code.KEY_X,
                    code.KEY_C,
                    code.KEY_V,
                    code.KEY_B,
                    code.KEY_N,
                    code.KEY_M,
                    code.KEY_COMMA,
                    code.KEY_DOT,
                    code.KEY_SLASH,
                    code.KEY_NONE, // Column 12 skips row 4.
                    code.KEY_MOD_RSHIFT,
                    code.KEY_UP,
                    code.KEY_P4,
                ],
                [
                    code.KEY_MOD_LCTRL,
                    code.KEY_MOD_LMETA,
                    code.KEY_NONE, // Column 2 skips row 5.
                    code.KEY_MOD_LALT,
                    code.KEY_NONE, // Column 4 skips row 5.
                    code.KEY_NONE, // Column 5 skips row 5.
                    code.KEY_SPACE,
                    code.KEY_NONE, // Column 7 skips row 5.
                    code.KEY_NONE, // Column 8 skips row 5.
                    code.KEY_NONE, // Column 9 skips row 5.
                    code.KEY_MOD_RMETA,
                    code.KEY_MOD_RCTRL,
                    code.KEY_NONE, // Column 12 skips row 5.
                    code.KEY_LEFT,
                    code.KEY_DOWN,
                    code.KEY_RIGHT,
                ],
            ],
        }
    }
}

impl KeycodeMapping {
    fn get_key(&self, row: usize, column: usize) -> u8 {
        self.scan_mapping[row][column]
    }
}
