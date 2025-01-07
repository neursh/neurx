use embassy_rp::{ gpio::{ Input, Pull }, Peripherals };

struct MatrixLayout<'a> {
    columns: [Input<'a>; 16],
    rows: [Input<'a>; 5],
}

impl<'a> MatrixLayout<'a> {
    fn new(peripherals: Peripherals) -> Self {
        MatrixLayout {
            columns: [
                Input::new(peripherals.PIN_15, Pull::Down),
                Input::new(peripherals.PIN_14, Pull::Down),
                Input::new(peripherals.PIN_13, Pull::Down),
                Input::new(peripherals.PIN_12, Pull::Down),
                Input::new(peripherals.PIN_11, Pull::Down),
                Input::new(peripherals.PIN_10, Pull::Down),
                Input::new(peripherals.PIN_9, Pull::Down),
                Input::new(peripherals.PIN_8, Pull::Down),
                Input::new(peripherals.PIN_7, Pull::Down),
                Input::new(peripherals.PIN_6, Pull::Down),
                Input::new(peripherals.PIN_5, Pull::Down),
                Input::new(peripherals.PIN_4, Pull::Down),
                Input::new(peripherals.PIN_3, Pull::Down),
                Input::new(peripherals.PIN_2, Pull::Down),
                Input::new(peripherals.PIN_1, Pull::Down),
                Input::new(peripherals.PIN_0, Pull::Down),
            ],
            rows: [
                Input::new(peripherals.PIN_16, Pull::Down),
                Input::new(peripherals.PIN_17, Pull::Down),
                Input::new(peripherals.PIN_18, Pull::Down),
                Input::new(peripherals.PIN_19, Pull::Down),
                Input::new(peripherals.PIN_20, Pull::Down),
            ],
        }
    }
}
