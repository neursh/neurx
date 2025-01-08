
/**
 * USB HID Keyboard scan codes
 * https://gist.github.com/MightyPork/6da26e382a7ad91b5496ee55fdc73db2
 */

#[allow(non_snake_case)]
pub struct Keycodes {
    /**
     * Modifier masks - used for the first byte in the HID report.
     * NOTE: The second byte in the report is reserved, 0x00
     */
    pub KEY_MOD_LCTRL: u8,
    pub KEY_MOD_LSHIFT: u8,
    pub KEY_MOD_LALT: u8,
    pub KEY_MOD_LMETA: u8,
    pub KEY_MOD_RCTRL: u8,
    pub KEY_MOD_RSHIFT: u8,
    pub KEY_MOD_RALT: u8,
    pub KEY_MOD_RMETA: u8,

    /**
     * Scan codes - last N slots in the HID report (usually 6).
     * 0x00 if no key pressed.
     *
     * If more than N keys are pressed, the HID reports
     * KEY_ERR_OVF in all slots to indicate this condition.
     */

    pub KEY_NONE: u8, // No key pressed
    pub KEY_ERR_OVF: u8, //  Keyboard Error Roll Over - used for all slots if too many keys are pressed ("Phantom key"),
    // 0x02 //  Keyboard POST Fail
    // 0x03 //  Keyboard Error Undefined
    pub KEY_A: u8, // Keyboard a and A
    pub KEY_B: u8, // Keyboard b and B
    pub KEY_C: u8, // Keyboard c and C
    pub KEY_D: u8, // Keyboard d and D
    pub KEY_E: u8, // Keyboard e and E
    pub KEY_F: u8, // Keyboard f and F
    pub KEY_G: u8, // Keyboard g and G
    pub KEY_H: u8, // Keyboard h and H
    pub KEY_I: u8, // Keyboard i and I
    pub KEY_J: u8, // Keyboard j and J
    pub KEY_K: u8, // Keyboard k and K
    pub KEY_L: u8, // Keyboard l and L
    pub KEY_M: u8, // Keyboard m and M
    pub KEY_N: u8, // Keyboard n and N
    pub KEY_O: u8, // Keyboard o and O
    pub KEY_P: u8, // Keyboard p and P
    pub KEY_Q: u8, // Keyboard q and Q
    pub KEY_R: u8, // Keyboard r and R
    pub KEY_S: u8, // Keyboard s and S
    pub KEY_T: u8, // Keyboard t and T
    pub KEY_U: u8, // Keyboard u and U
    pub KEY_V: u8, // Keyboard v and V
    pub KEY_W: u8, // Keyboard w and W
    pub KEY_X: u8, // Keyboard x and X
    pub KEY_Y: u8, // Keyboard y and Y
    pub KEY_Z: u8, // Keyboard z and Z

    pub KEY_1: u8, // Keyboard 1 and !
    pub KEY_2: u8, // Keyboard 2 and @
    pub KEY_3: u8, // Keyboard 3 and #
    pub KEY_4: u8, // Keyboard 4 and $
    pub KEY_5: u8, // Keyboard 5 and %
    pub KEY_6: u8, // Keyboard 6 and ^
    pub KEY_7: u8, // Keyboard 7 and &
    pub KEY_8: u8, // Keyboard 8 and *
    pub KEY_9: u8, // Keyboard 9 and (
    pub KEY_0: u8, // Keyboard 0 and )

    pub KEY_ENTER: u8, // Keyboard Return (ENTER)
    pub KEY_ESC: u8, // Keyboard ESCAPE
    pub KEY_BACKSPACE: u8, // Keyboard DELETE (Backspace)
    pub KEY_TAB: u8, // Keyboard Tab
    pub KEY_SPACE: u8, // Keyboard Spacebar
    pub KEY_MINUS: u8, // Keyboard - and _
    pub KEY_EQUAL: u8, // Keyboard = and +
    pub KEY_LEFTBRACE: u8, // Keyboard [ and {
    pub KEY_RIGHTBRACE: u8, // Keyboard ] and }
    pub KEY_BACKSLASH: u8, // Keyboard \ and |
    pub KEY_HASHTILDE: u8, // Keyboard Non-US # and ~
    pub KEY_SEMICOLON: u8, // Keyboard ; and :
    pub KEY_APOSTROPHE: u8, // Keyboard ' and "
    pub KEY_GRAVE: u8, // Keyboard ` and ~
    pub KEY_COMMA: u8, // Keyboard , and <
    pub KEY_DOT: u8, // Keyboard . and >
    pub KEY_SLASH: u8, // Keyboard / and ?
    pub KEY_CAPSLOCK: u8, // Keyboard Caps Lock

    pub KEY_F1: u8, // Keyboard F1
    pub KEY_F2: u8, // Keyboard F2
    pub KEY_F3: u8, // Keyboard F3
    pub KEY_F4: u8, // Keyboard F4
    pub KEY_F5: u8, // Keyboard F5
    pub KEY_F6: u8, // Keyboard F6
    pub KEY_F7: u8, // Keyboard F7
    pub KEY_F8: u8, // Keyboard F8
    pub KEY_F9: u8, // Keyboard F9
    pub KEY_F10: u8, // Keyboard F10
    pub KEY_F11: u8, // Keyboard F11
    pub KEY_F12: u8, // Keyboard F12

    pub KEY_SYSRQ: u8, // Keyboard Print Screen
    pub KEY_SCROLLLOCK: u8, // Keyboard Scroll Lock
    pub KEY_PAUSE: u8, // Keyboard Pause
    pub KEY_INSERT: u8, // Keyboard Insert
    pub KEY_HOME: u8, // Keyboard Home
    pub KEY_PAGEUP: u8, // Keyboard Page Up
    pub KEY_DELETE: u8, // Keyboard Delete Forward
    pub KEY_END: u8, // Keyboard End
    pub KEY_PAGEDOWN: u8, // Keyboard Page Down
    pub KEY_RIGHT: u8, // Keyboard Right Arrow
    pub KEY_LEFT: u8, // Keyboard Left Arrow
    pub KEY_DOWN: u8, // Keyboard Down Arrow
    pub KEY_UP: u8, // Keyboard Up Arrow

    pub KEY_NUMLOCK: u8, // Keyboard Num Lock and Clear
    pub KEY_KPSLASH: u8, // Keypad /
    pub KEY_KPASTERISK: u8, // Keypad *
    pub KEY_KPMINUS: u8, // Keypad -
    pub KEY_KPPLUS: u8, // Keypad +
    pub KEY_KPENTER: u8, // Keypad ENTER
    pub KEY_KP1: u8, // Keypad 1 and End
    pub KEY_KP2: u8, // Keypad 2 and Down Arrow
    pub KEY_KP3: u8, // Keypad 3 and PageDn
    pub KEY_KP4: u8, // Keypad 4 and Left Arrow
    pub KEY_KP5: u8, // Keypad 5
    pub KEY_KP6: u8, // Keypad 6 and Right Arrow
    pub KEY_KP7: u8, // Keypad 7 and Home
    pub KEY_KP8: u8, // Keypad 8 and Up Arrow
    pub KEY_KP9: u8, // Keypad 9 and Page Up
    pub KEY_KP0: u8, // Keypad 0 and Insert
    pub KEY_KPDOT: u8, // Keypad . and Delete

    pub KEY_102ND: u8, // Keyboard Non-US \ and |
    pub KEY_COMPOSE: u8, // Keyboard Application
    pub KEY_POWER: u8, // Keyboard Power
    pub KEY_KPEQUAL: u8, // Keypad =

    pub KEY_F13: u8, // Keyboard F13
    pub KEY_F14: u8, // Keyboard F14
    pub KEY_F15: u8, // Keyboard F15
    pub KEY_F16: u8, // Keyboard F16
    pub KEY_F17: u8, // Keyboard F17
    pub KEY_F18: u8, // Keyboard F18
    pub KEY_F19: u8, // Keyboard F19
    pub KEY_F20: u8, // Keyboard F20
    pub KEY_F21: u8, // Keyboard F21
    pub KEY_F22: u8, // Keyboard F22
    pub KEY_F23: u8, // Keyboard F23
    pub KEY_F24: u8, // Keyboard F24

    pub KEY_OPEN: u8, // Keyboard Execute
    pub KEY_HELP: u8, // Keyboard Help
    pub KEY_PROPS: u8, // Keyboard Menu
    pub KEY_FRONT: u8, // Keyboard Select
    pub KEY_STOP: u8, // Keyboard Stop
    pub KEY_AGAIN: u8, // Keyboard Again
    pub KEY_UNDO: u8, // Keyboard Undo
    pub KEY_CUT: u8, // Keyboard Cut
    pub KEY_COPY: u8, // Keyboard Copy
    pub KEY_PASTE: u8, // Keyboard Paste
    pub KEY_FIND: u8, // Keyboard Find
    pub KEY_MUTE: u8, // Keyboard Mute
    pub KEY_VOLUMEUP: u8, // Keyboard Volume Up
    pub KEY_VOLUMEDOWN: u8, // Keyboard Volume Down
    // 0x82  Keyboard Locking Caps Lock
    // 0x83  Keyboard Locking Num Lock
    // 0x84  Keyboard Locking Scroll Lock
    pub KEY_KPCOMMA: u8, // Keypad Comma
    // 0x86  Keypad Equal Sign
    pub KEY_RO: u8, // Keyboard International1
    pub KEY_KATAKANAHIRAGANA: u8, // Keyboard International2
    pub KEY_YEN: u8, // Keyboard International3
    pub KEY_HENKAN: u8, // Keyboard International4
    pub KEY_MUHENKAN: u8, // Keyboard International5
    pub KEY_KPJPCOMMA: u8, // Keyboard International6
    // 0x8d  Keyboard International7
    // 0x8e  Keyboard International8
    // 0x8f  Keyboard International9
    pub KEY_HANGEUL: u8, // Keyboard LANG1
    pub KEY_HANJA: u8, // Keyboard LANG2
    pub KEY_KATAKANA: u8, // Keyboard LANG3
    pub KEY_HIRAGANA: u8, // Keyboard LANG4
    pub KEY_ZENKAKUHANKAKU: u8, // Keyboard LANG5
    // 0x95  Keyboard LANG6
    // 0x96  Keyboard LANG7
    // 0x97  Keyboard LANG8
    // 0x98  Keyboard LANG9
    // 0x99  Keyboard Alternate Erase
    // 0x9a  Keyboard SysReq/Attention
    // 0x9b  Keyboard Cancel
    // 0x9c  Keyboard Clear
    // 0x9d  Keyboard Prior
    // 0x9e  Keyboard Return
    // 0x9f  Keyboard Separator
    // 0xa0  Keyboard Out
    // 0xa1  Keyboard Oper
    // 0xa2  Keyboard Clear/Again
    // 0xa3  Keyboard CrSel/Props
    // 0xa4  Keyboard ExSel

    // 0xb0  Keypad 00
    // 0xb1  Keypad 000
    // 0xb2  Thousands Separator
    // 0xb3  Decimal Separator
    // 0xb4  Currency Unit
    // 0xb5  Currency Sub-unit
    pub KEY_KPLEFTPAREN: u8, // Keypad (
    pub KEY_KPRIGHTPAREN: u8, // Keypad )
    // 0xb8  Keypad {
    // 0xb9  Keypad }
    // 0xba  Keypad Tab
    // 0xbb  Keypad Backspace
    // 0xbc  Keypad A
    // 0xbd  Keypad B
    // 0xbe  Keypad C
    // 0xbf  Keypad D
    // 0xc0  Keypad E
    // 0xc1  Keypad F
    // 0xc2  Keypad XOR
    // 0xc3  Keypad ^
    // 0xc4  Keypad %
    // 0xc5  Keypad <
    // 0xc6  Keypad >
    // 0xc7  Keypad &
    // 0xc8  Keypad &&
    // 0xc9  Keypad |
    // 0xca  Keypad ||
    // 0xcb  Keypad :
    // 0xcc  Keypad #
    // 0xcd  Keypad Space
    // 0xce  Keypad @
    // 0xcf  Keypad !
    // 0xd0  Keypad Memory Store
    // 0xd1  Keypad Memory Recall
    // 0xd2  Keypad Memory Clear
    // 0xd3  Keypad Memory Add
    // 0xd4  Keypad Memory Subtract
    // 0xd5  Keypad Memory Multiply
    // 0xd6  Keypad Memory Divide
    // 0xd7  Keypad +/-
    // 0xd8  Keypad Clear
    // 0xd9  Keypad Clear Entry
    // 0xda  Keypad Binary
    // 0xdb  Keypad Octal
    // 0xdc  Keypad Decimal
    // 0xdd  Keypad Hexadecimal

    pub KEY_LEFTCTRL: u8, // Keyboard Left Control
    pub KEY_LEFTSHIFT: u8, // Keyboard Left Shift
    pub KEY_LEFTALT: u8, // Keyboard Left Alt
    pub KEY_LEFTMETA: u8, // Keyboard Left GUI
    pub KEY_RIGHTCTRL: u8, // Keyboard Right Control
    pub KEY_RIGHTSHIFT: u8, // Keyboard Right Shift
    pub KEY_RIGHTALT: u8, // Keyboard Right Alt
    pub KEY_RIGHTMETA: u8, // Keyboard Right GUI

    pub KEY_MEDIA_PLAYPAUSE: u8,
    pub KEY_MEDIA_STOPCD: u8,
    pub KEY_MEDIA_PREVIOUSSONG: u8,
    pub KEY_MEDIA_NEXTSONG: u8,
    pub KEY_MEDIA_EJECTCD: u8,
    pub KEY_MEDIA_VOLUMEUP: u8,
    pub KEY_MEDIA_VOLUMEDOWN: u8,
    pub KEY_MEDIA_MUTE: u8,
    pub KEY_MEDIA_WWW: u8,
    pub KEY_MEDIA_BACK: u8,
    pub KEY_MEDIA_FORWARD: u8,
    pub KEY_MEDIA_STOP: u8,
    pub KEY_MEDIA_FIND: u8,
    pub KEY_MEDIA_SCROLLUP: u8,
    pub KEY_MEDIA_SCROLLDOWN: u8,
    pub KEY_MEDIA_EDIT: u8,
    pub KEY_MEDIA_SLEEP: u8,
    pub KEY_MEDIA_COFFEE: u8,
    pub KEY_MEDIA_REFRESH: u8,
    pub KEY_MEDIA_CALC: u8,

    /**
     * Neurx programmable key codes.
     */
    pub KEY_P1: u8,
    pub KEY_P2: u8,
    pub KEY_P3: u8,
    pub KEY_P4: u8,
}

impl Default for Keycodes {
    fn default() -> Self {
        Self {
            KEY_MOD_LCTRL: 0x01,
            KEY_MOD_LSHIFT: 0x02,
            KEY_MOD_LALT: 0x04,
            KEY_MOD_LMETA: 0x08,
            KEY_MOD_RCTRL: 0x10,
            KEY_MOD_RSHIFT: 0x20,
            KEY_MOD_RALT: 0x40,
            KEY_MOD_RMETA: 0x80,

            KEY_NONE: 0x00,
            KEY_ERR_OVF: 0x01,

            KEY_A: 0x04,
            KEY_B: 0x05,
            KEY_C: 0x06,
            KEY_D: 0x07,
            KEY_E: 0x08,
            KEY_F: 0x09,
            KEY_G: 0x0a,
            KEY_H: 0x0b,
            KEY_I: 0x0c,
            KEY_J: 0x0d,
            KEY_K: 0x0e,
            KEY_L: 0x0f,
            KEY_M: 0x10,
            KEY_N: 0x11,
            KEY_O: 0x12,
            KEY_P: 0x13,
            KEY_Q: 0x14,
            KEY_R: 0x15,
            KEY_S: 0x16,
            KEY_T: 0x17,
            KEY_U: 0x18,
            KEY_V: 0x19,
            KEY_W: 0x1a,
            KEY_X: 0x1b,
            KEY_Y: 0x1c,
            KEY_Z: 0x1d,

            KEY_1: 0x1e,
            KEY_2: 0x1f,
            KEY_3: 0x20,
            KEY_4: 0x21,
            KEY_5: 0x22,
            KEY_6: 0x23,
            KEY_7: 0x24,
            KEY_8: 0x25,
            KEY_9: 0x26,
            KEY_0: 0x27,

            KEY_ENTER: 0x28,
            KEY_ESC: 0x29,
            KEY_BACKSPACE: 0x2a,
            KEY_TAB: 0x2b,
            KEY_SPACE: 0x2c,
            KEY_MINUS: 0x2d,
            KEY_EQUAL: 0x2e,
            KEY_LEFTBRACE: 0x2f,
            KEY_RIGHTBRACE: 0x30,
            KEY_BACKSLASH: 0x31,
            KEY_HASHTILDE: 0x32,
            KEY_SEMICOLON: 0x33,
            KEY_APOSTROPHE: 0x34,
            KEY_GRAVE: 0x35,
            KEY_COMMA: 0x36,
            KEY_DOT: 0x37,
            KEY_SLASH: 0x38,
            KEY_CAPSLOCK: 0x39,

            KEY_F1: 0x3a,
            KEY_F2: 0x3b,
            KEY_F3: 0x3c,
            KEY_F4: 0x3d,
            KEY_F5: 0x3e,
            KEY_F6: 0x3f,
            KEY_F7: 0x40,
            KEY_F8: 0x41,
            KEY_F9: 0x42,
            KEY_F10: 0x43,
            KEY_F11: 0x44,
            KEY_F12: 0x45,

            KEY_SYSRQ: 0x46,
            KEY_SCROLLLOCK: 0x47,
            KEY_PAUSE: 0x48,
            KEY_INSERT: 0x49,
            KEY_HOME: 0x4a,
            KEY_PAGEUP: 0x4b,
            KEY_DELETE: 0x4c,
            KEY_END: 0x4d,
            KEY_PAGEDOWN: 0x4e,
            KEY_RIGHT: 0x4f,
            KEY_LEFT: 0x50,
            KEY_DOWN: 0x51,
            KEY_UP: 0x52,

            KEY_NUMLOCK: 0x53,
            KEY_KPSLASH: 0x54,
            KEY_KPASTERISK: 0x55,
            KEY_KPMINUS: 0x56,
            KEY_KPPLUS: 0x57,
            KEY_KPENTER: 0x58,
            KEY_KP1: 0x59,
            KEY_KP2: 0x5a,
            KEY_KP3: 0x5b,
            KEY_KP4: 0x5c,
            KEY_KP5: 0x5d,
            KEY_KP6: 0x5e,
            KEY_KP7: 0x5f,
            KEY_KP8: 0x60,
            KEY_KP9: 0x61,
            KEY_KP0: 0x62,
            KEY_KPDOT: 0x63,

            KEY_102ND: 0x64,
            KEY_COMPOSE: 0x65,
            KEY_POWER: 0x66,
            KEY_KPEQUAL: 0x67,

            KEY_F13: 0x68,
            KEY_F14: 0x69,
            KEY_F15: 0x6a,
            KEY_F16: 0x6b,
            KEY_F17: 0x6c,
            KEY_F18: 0x6d,
            KEY_F19: 0x6e,
            KEY_F20: 0x6f,
            KEY_F21: 0x70,
            KEY_F22: 0x71,
            KEY_F23: 0x72,
            KEY_F24: 0x73,

            KEY_OPEN: 0x74,
            KEY_HELP: 0x75,
            KEY_PROPS: 0x76,
            KEY_FRONT: 0x77,
            KEY_STOP: 0x78,
            KEY_AGAIN: 0x79,
            KEY_UNDO: 0x7a,
            KEY_CUT: 0x7b,
            KEY_COPY: 0x7c,
            KEY_PASTE: 0x7d,
            KEY_FIND: 0x7e,
            KEY_MUTE: 0x7f,
            KEY_VOLUMEUP: 0x80,
            KEY_VOLUMEDOWN: 0x81,
            KEY_KPCOMMA: 0x85,
            KEY_RO: 0x87,
            KEY_KATAKANAHIRAGANA: 0x88,
            KEY_YEN: 0x89,
            KEY_HENKAN: 0x8a,
            KEY_MUHENKAN: 0x8b,
            KEY_KPJPCOMMA: 0x8c,
            KEY_HANGEUL: 0x90,
            KEY_HANJA: 0x91,
            KEY_KATAKANA: 0x92,
            KEY_HIRAGANA: 0x93,
            KEY_ZENKAKUHANKAKU: 0x94,
            KEY_KPLEFTPAREN: 0xb6,
            KEY_KPRIGHTPAREN: 0xb7,
            KEY_LEFTCTRL: 0xe0,
            KEY_LEFTSHIFT: 0xe1,
            KEY_LEFTALT: 0xe2,
            KEY_LEFTMETA: 0xe3,
            KEY_RIGHTCTRL: 0xe4,
            KEY_RIGHTSHIFT: 0xe5,
            KEY_RIGHTALT: 0xe6,
            KEY_RIGHTMETA: 0xe7,

            KEY_MEDIA_PLAYPAUSE: 0xe8,
            KEY_MEDIA_STOPCD: 0xe9,
            KEY_MEDIA_PREVIOUSSONG: 0xea,
            KEY_MEDIA_NEXTSONG: 0xeb,
            KEY_MEDIA_EJECTCD: 0xec,
            KEY_MEDIA_VOLUMEUP: 0xed,
            KEY_MEDIA_VOLUMEDOWN: 0xee,
            KEY_MEDIA_MUTE: 0xef,
            KEY_MEDIA_WWW: 0xf0,
            KEY_MEDIA_BACK: 0xf1,
            KEY_MEDIA_FORWARD: 0xf2,
            KEY_MEDIA_STOP: 0xf3,
            KEY_MEDIA_FIND: 0xf4,
            KEY_MEDIA_SCROLLUP: 0xf5,
            KEY_MEDIA_SCROLLDOWN: 0xf6,
            KEY_MEDIA_EDIT: 0xf7,
            KEY_MEDIA_SLEEP: 0xf8,
            KEY_MEDIA_COFFEE: 0xf9,
            KEY_MEDIA_REFRESH: 0xfa,
            KEY_MEDIA_CALC: 0xfb,

            KEY_P1: 0xfc,
            KEY_P2: 0xfd,
            KEY_P3: 0xfe,
            KEY_P4: 0xff,
        }
    }
}
