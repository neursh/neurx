use embassy_rp::{ gpio::{ Input, Level, Output, Pull }, Peripherals };

use crate::keycode::Keycodes;

struct MatrixLayout<'a> {
    columns: [Output<'a>; 16],
    rows: [Input<'a>; 6],
}

impl<'a> MatrixLayout<'a> {
    fn new(peripherals: Peripherals) -> Self {
        MatrixLayout {
            columns: [
                Output::new(peripherals.PIN_15, Level::Low),
                Output::new(peripherals.PIN_14, Level::Low),
                Output::new(peripherals.PIN_13, Level::Low),
                Output::new(peripherals.PIN_12, Level::Low),
                Output::new(peripherals.PIN_11, Level::Low),
                Output::new(peripherals.PIN_10, Level::Low),
                Output::new(peripherals.PIN_9, Level::Low),
                Output::new(peripherals.PIN_8, Level::Low),
                Output::new(peripherals.PIN_7, Level::Low),
                Output::new(peripherals.PIN_6, Level::Low),
                Output::new(peripherals.PIN_5, Level::Low),
                Output::new(peripherals.PIN_4, Level::Low),
                Output::new(peripherals.PIN_3, Level::Low),
                Output::new(peripherals.PIN_2, Level::Low),
                Output::new(peripherals.PIN_1, Level::Low),
                Output::new(peripherals.PIN_0, Level::Low),
            ],
            rows: [
                Input::new(peripherals.PIN_16, Pull::Down),
                Input::new(peripherals.PIN_17, Pull::Down),
                Input::new(peripherals.PIN_18, Pull::Down),
                Input::new(peripherals.PIN_19, Pull::Down),
                Input::new(peripherals.PIN_20, Pull::Down),
                Input::new(peripherals.PIN_21, Pull::Down),
            ],
        }
    }

    fn scan(mut self) {
        let mut report: [u8; 8] = [0u8; 8];

        for column_data in self.columns.iter_mut().enumerate() {
            column_data.1.set_high();

            for row_data in self.rows.iter().enumerate() {
                if row_data.1.is_high() {
                }
            }

            column_data.1.set_low();
        }
    }
}

struct KeyscanService {
    scan: [[u8; 16]; 5],
}
