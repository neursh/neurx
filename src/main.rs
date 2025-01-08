#![no_std]
#![no_main]

mod matrix;
mod keycode;

use core::{ panic::PanicInfo, sync::atomic::{ AtomicBool, Ordering } };

use embassy_executor::Spawner;
use embassy_futures::{ join::join, yield_now };
use embassy_rp::{
    bind_interrupts,
    gpio::{ Input, Level, Output, Pull },
    peripherals::USB,
    usb::{ Driver, InterruptHandler },
};
use embassy_usb::{
    class::hid::{ HidReaderWriter, RequestHandler, State },
    Builder,
    Config,
    Handler,
};
use matrix::MatrixLayout;
use usbd_hid::descriptor::{ KeyboardReport, SerializedDescriptor };

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let driver = Driver::new(peripherals.USB, Irqs);

    let mut matrix = MatrixLayout::new(
        [
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
        [
            Input::new(peripherals.PIN_16, Pull::Down),
            Input::new(peripherals.PIN_17, Pull::Down),
            Input::new(peripherals.PIN_18, Pull::Down),
            Input::new(peripherals.PIN_19, Pull::Down),
            Input::new(peripherals.PIN_20, Pull::Down),
            Input::new(peripherals.PIN_21, Pull::Down),
        ]
    );

    let mut config = Config::new(14521, 1819);
    config.manufacturer = Some("Neurs");
    config.product = Some("Neurx");
    config.serial_number = Some("Neurx 1.0");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut request_handler = NeurxRequestHandler {};
    let mut device_handler = DeviceHandler::new();

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf
    );

    builder.handler(&mut device_handler);

    // Create classes on the builder.
    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 1,
        max_packet_size: 64,
    };
    let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut state, config);

    let mut usb = builder.build();
    let runtime = usb.run();

    let (reader, mut writer) = hid.split();

    let usb_output = async {
        loop {
            let report = matrix.scan();
            writer.write(&report).await.ok();

            yield_now().await;
        }
    };

    let host_request = async {
        reader.run(false, &mut request_handler).await;
    };

    join(runtime, join(usb_output, host_request)).await;
}

struct NeurxRequestHandler {}
impl RequestHandler for NeurxRequestHandler {}

struct DeviceHandler {
    configured: AtomicBool,
}
impl DeviceHandler {
    fn new() -> Self {
        DeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for DeviceHandler {
    fn enabled(&mut self, _enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
    }

    fn addressed(&mut self, _addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
    }
}
