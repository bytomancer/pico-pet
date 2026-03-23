use cortex_m::delay::Delay;

use cortex_m::prelude::_embedded_hal_blocking_i2c_Read;
use cortex_m::prelude::_embedded_hal_blocking_i2c_Write;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::prelude::RgbColor;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;
use fugit::RateExtU32;

use rp2040_hal::clocks::init_clocks_and_plls;
use rp2040_hal::clocks::Clock;
use rp2040_hal::gpio::Pins;
use rp2040_hal::pac;
use rp2040_hal::pac::PPB;
use rp2040_hal::pac::PSM;
use rp2040_hal::pio::PIOExt;
use rp2040_hal::sio::SioFifo;
use rp2040_hal::watchdog::Watchdog;
use rp2040_hal::Adc;
use rp2040_hal::Sio;
use rp2040_hal::{self};

use st7735_lcd::Orientation;
use st7735_lcd::ST7735;

use crate::game::hardware::rtc::real_time::RealTime;
use crate::game::nvm::settings::SettingType;

use super::audio::AudioFrequency;
use super::rtc;
use crate::game::hardware::rtc::RealDate;
use crate::game::hardware::rtc::RealDateTime;

pub const LCD_WIDTH: usize = 128;
pub const LCD_HEIGHT: usize = 128;

const RTC_ADDRESS: u8 = 0x68;

// For the module from Amazon
// const NVM_ADDRESS: u8 = 0x57;
// For my custom module with flipped bits, whoops
// const NVM_ADDRESS: u8 = 0b_0_1010_000;

pub const BRIGHTNESS_LUT: [u16; 16] = [
    306, 438, 626, 895, 1281, 1831, 2619, 3746, 5357, 7660, 10955, 15667, 22406, 32043, 45825,
    65535,
];
pub const VOLUME_LUT: [u16; 6] = [0, 32768 / 256, 32768 / 128, 32768 / 64, 32768 / 32, 32768];

type DisplaySdi = rp2040_hal::Spi<rp2040_hal::spi::Enabled, pac::SPI1, 8>;
type DisplayDc = rp2040_hal::gpio::Pin<
    rp2040_hal::gpio::bank0::Gpio8,
    rp2040_hal::gpio::Output<rp2040_hal::gpio::PushPull>,
>;
type DisplayRst = rp2040_hal::gpio::Pin<
    rp2040_hal::gpio::bank0::Gpio12,
    rp2040_hal::gpio::Output<rp2040_hal::gpio::PushPull>,
>;

pub type Lcd = ST7735<DisplaySdi, DisplayDc, DisplayRst>;

type LcdBlPinChannel = rp2040_hal::pwm::Channel<
    rp2040_hal::pwm::Pwm6,
    rp2040_hal::pwm::FreeRunning,
    rp2040_hal::pwm::B,
>;
type BuzzerPinChannel = rp2040_hal::pwm::Channel<
    rp2040_hal::pwm::Pwm2,
    rp2040_hal::pwm::FreeRunning,
    rp2040_hal::pwm::A,
>;
type BuzzerPwmSlice = rp2040_hal::pwm::Slice<rp2040_hal::pwm::Pwm2, rp2040_hal::pwm::FreeRunning>;

type Key0Pin = rp2040_hal::gpio::Pin<
    rp2040_hal::gpio::bank0::Gpio15,
    rp2040_hal::gpio::Input<rp2040_hal::gpio::PullUp>,
>;
type Key1Pin = rp2040_hal::gpio::Pin<
    rp2040_hal::gpio::bank0::Gpio17,
    rp2040_hal::gpio::Input<rp2040_hal::gpio::PullUp>,
>;
type Key2Pin = rp2040_hal::gpio::Pin<
    rp2040_hal::gpio::bank0::Gpio2,
    rp2040_hal::gpio::Input<rp2040_hal::gpio::PullUp>,
>;
type Key3Pin = rp2040_hal::gpio::Pin<
    rp2040_hal::gpio::bank0::Gpio3,
    rp2040_hal::gpio::Input<rp2040_hal::gpio::PullUp>,
>;
type Key5Pin = rp2040_hal::gpio::Pin<
    rp2040_hal::gpio::bank0::Gpio5,
    rp2040_hal::gpio::Input<rp2040_hal::gpio::PullUp>,
>;

type Adc0Pin = rp2040_hal::gpio::Pin<
    rp2040_hal::gpio::bank0::Gpio26,
    rp2040_hal::gpio::Input<rp2040_hal::gpio::Floating>,
>;

// type VsenseEnablePin = rp2040_hal::gpio::Pin<
//     rp2040_hal::gpio::bank0::Gpio22,
//     rp2040_hal::gpio::Output<rp2040_hal::gpio::PushPull>,
// >;

type VibePin = rp2040_hal::gpio::Pin<
    rp2040_hal::gpio::bank0::Gpio6,
    rp2040_hal::gpio::Output<rp2040_hal::gpio::PushPull>,
>;

type I2CBus = rp2040_hal::I2C<
    pac::I2C0,
    (
        rp2040_hal::gpio::Pin<
            rp2040_hal::gpio::bank0::Gpio0,
            rp2040_hal::gpio::Function<rp2040_hal::gpio::I2C>,
        >,
        rp2040_hal::gpio::Pin<
            rp2040_hal::gpio::bank0::Gpio1,
            rp2040_hal::gpio::Function<rp2040_hal::gpio::I2C>,
        >,
    ),
>;

#[allow(unused)]
pub struct HardwareComponents {
    pub display: Lcd,
    pub sys_freq: u32,
    pub backlight_channel_ptr: *mut LcdBlPinChannel,
    pub buzzer_channel_ptr: *mut BuzzerPinChannel,
    pub buzzer_pwm_slice_ptr: *mut BuzzerPwmSlice,
    pub delay: Delay,
    pub key0: Key0Pin,
    pub key1: Key1Pin,
    pub key2: Key2Pin,
    pub key3: Key3Pin,
    pub second_clock: Key5Pin,
    pub vibe: VibePin,
    pub psm_ptr: *mut PSM,
    pub ppb_ptr: *mut PPB,
    pub fifo_ptr: *mut SioFifo,
    pub i2c_bus: I2CBus,
    pub adc: Adc,
    pub vsense_pin: Adc0Pin,
    // pub vsense_enable_pin: VsenseEnablePin,
    pub nvm_addr: u8,
}
impl HardwareComponents {
    pub fn new() -> Self {
        unsafe {
            let mut pac = pac::Peripherals::take().unwrap();
            let core = pac::CorePeripherals::take().unwrap();

            let mut watchdog = Watchdog::new(pac.WATCHDOG);

            let clocks = init_clocks_and_plls(
                12_000_000,
                pac.XOSC,
                pac.CLOCKS,
                pac.PLL_SYS,
                pac.PLL_USB,
                &mut pac.RESETS,
                &mut watchdog,
            )
            .ok()
            .unwrap();

            let mut sio = Sio::new(pac.SIO);
            let pins = Pins::new(
                pac.IO_BANK0,
                pac.PADS_BANK0,
                sio.gpio_bank0,
                &mut pac.RESETS,
            );

            // Init PWMs
            let mut pwm_slices: rp2040_hal::pwm::Slices =
                rp2040_hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

            // Configure LCD PWM slice
            let pwm6 = &mut pwm_slices.pwm6;
            pwm6.set_ph_correct();
            pwm6.set_top(65535);
            pwm6.set_div_int(1);
            pwm6.set_div_frac(0);
            pwm6.enable();

            // Configure buzzer PWM slice
            let buzzer_pwm_slice_ptr: *mut BuzzerPwmSlice =
                &mut pwm_slices.pwm2 as *mut BuzzerPwmSlice;

            // Output channel B on PWM6 to GPIO 13
            let backlight_channel_ptr = &mut pwm6.channel_b as *mut LcdBlPinChannel;
            // disable backlight ASAP to hide boot artifacts
            (*backlight_channel_ptr).output_to(pins.gpio13);
            (*backlight_channel_ptr).enable();
            (*backlight_channel_ptr).set_duty(0);

            let buzzer_channel_ptr =
                &mut (*buzzer_pwm_slice_ptr).channel_a as *mut BuzzerPinChannel;
            (*buzzer_channel_ptr).output_to(pins.gpio4);
            (*buzzer_channel_ptr).enable();
            (*buzzer_channel_ptr).set_duty(0);

            (*buzzer_pwm_slice_ptr).set_ph_correct();
            (*buzzer_pwm_slice_ptr).set_div_int(0);
            (*buzzer_pwm_slice_ptr).set_div_frac(0);
            (*buzzer_pwm_slice_ptr).set_top(0);
            (*buzzer_pwm_slice_ptr).enable();

            let key0 = pins.gpio15.into_pull_up_input();
            let key1 = pins.gpio17.into_pull_up_input();
            let key2 = pins.gpio2.into_pull_up_input();
            let key3 = pins.gpio3.into_pull_up_input();

            let second_clock = pins.gpio5.into_pull_up_input();

            let mut vsense_enable_pin = pins.gpio22.into_push_pull_output();
            vsense_enable_pin.set_low().unwrap();

            let adc: Adc = Adc::new(pac.ADC, &mut pac.RESETS);
            let vsense_pin = pins.gpio26.into_floating_input();

            let mut vibe = pins.gpio6.into_push_pull_output();
            vibe.set_low().unwrap();

            let sys_freq = clocks.system_clock.freq().to_Hz();
            let mut delay = Delay::new(core.SYST, sys_freq);

            let (mut _pio, _sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

            let lcd_dc = pins.gpio8.into_push_pull_output();
            let mut _lcd_cs = pins.gpio9.into_mode::<rp2040_hal::gpio::FunctionSpi>();
            let mut _lcd_clk = pins.gpio10.into_mode::<rp2040_hal::gpio::FunctionSpi>();
            let mut _lcd_mosi = pins.gpio11.into_mode::<rp2040_hal::gpio::FunctionSpi>();
            let lcd_rst = pins
                .gpio12
                .into_push_pull_output_in_state(rp2040_hal::gpio::PinState::High);

            let spi = rp2040_hal::Spi::<_, _, 8>::new(pac.SPI1);

            // This frequency dictates our rate of communication to the display.
            // There is a strong correlation between frequency and FPS.
            // Should we choose to move from 128x128 to 128x160, these numbers all change.
            // Additionally, there are "thresholds" where the FPS will not change. So 24-30 MHz all produce 31-32 FPS.
            // Readings are reported per second, and due to the 1hz clock & FPS count desynchronizing, that produces 2 FPS readings.
            // Essentially we have a floating point FPS somewhere between the 2 integers reported.
            // There may need to be power consumption investigations for the different frequencies as well.
            //
            // 128x128 using write_pixels_buffered():
            // 43-44 FPS    63MHz+
            // 35-36 FPS    32MHz - 62MHz
            // 31-32 FPS    21MHz - 31MHz
            // 27-28 FPS    16MHz - 20MHz
            // 25-26 FPS    13MHz - 15MHz
            // 22-23 FPS    11MHz - 12MHz
            // 20-21 FPS    9MHz - 10MHz
            // 19-20 FPS    8MHz
            // 17-18 FPS    7MHz
            // 15-16 FPS    6MHz
            // 13-14 FPS    5MHz
            // 11-12 FPS    4MHz
            // 9-10 FPS    3MHz
            // 6-7 FPS    2MHz
            // 3-4 FPS    1MHz
            //
            // 21MHz = 33.7mA
            // 31MHz = 33.6mA
            // 6MHz = 30.5mA
            // So FPS has _some_ effect,
            // but MHz likely has no effect

            let spi = spi.init(
                &mut pac.RESETS,
                clocks.peripheral_clock.freq(),
                24.MHz(),
                &embedded_hal::spi::MODE_0,
            );

            let mut display = ST7735::new(
                spi,
                lcd_dc,
                lcd_rst,
                false,
                false,
                LCD_WIDTH as u32,
                LCD_HEIGHT as u32,
            );

            display.init(&mut delay).unwrap();
            display.set_orientation(&Orientation::Portrait).unwrap();

            display.set_offset(2, 1);

            let psm_ptr: *mut PSM = &mut pac.PSM as *mut PSM;
            let ppb_ptr: *mut PPB = &mut pac.PPB as *mut PPB;
            let fifo_ptr: *mut SioFifo = &mut sio.fifo as *mut SioFifo;

            display
                .clear(Rgb565::BLACK)
                .expect("Could not initialize display");

            let sda_pin = pins.gpio0.into_mode::<rp2040_hal::gpio::FunctionI2C>();
            let scl_pin = pins.gpio1.into_mode::<rp2040_hal::gpio::FunctionI2C>();

            let mut i2c_bus: I2CBus = rp2040_hal::I2C::i2c0(
                pac.I2C0,
                sda_pin,
                scl_pin,
                400.kHz(),
                &mut pac.RESETS,
                &clocks.system_clock,
            );

            let probe_addr = [0u8, 0u8];
            let mut nvm_addr = 0x00;
            for possible_addr in 0x50..=0x57 {
                match i2c_bus.write(possible_addr, &probe_addr) {
                    Ok(_) => {
                        nvm_addr = possible_addr;
                        break;
                    }
                    Err(_) => {}
                };
            }
            if nvm_addr == 0x00 {
                // This message will not display as the hardware component is not initialized yet
                panic!("No NVM detected.");
            }

            let mut s = Self {
                display,
                sys_freq,
                backlight_channel_ptr,
                buzzer_channel_ptr,
                buzzer_pwm_slice_ptr,
                delay,
                key0,
                key1,
                key2,
                key3,
                second_clock,
                vibe,
                psm_ptr,
                ppb_ptr,
                fifo_ptr,
                i2c_bus,
                adc,
                vsense_pin,
                // vsense_enable_pin,
                nvm_addr,
            };

            s.init_wfi();

            // enable 1hz clock
            s.write_sqw_pin_mode(0x00);

            s
        }
    }

    pub fn get_vsense(&mut self) -> u16 {
        // self.vsense_enable_pin.set_high().unwrap();
        let r = <Adc as embedded_hal::prelude::_embedded_hal_adc_OneShot<
            Adc,
            u16,
            rp2040_hal::gpio::Pin<
                rp2040_hal::gpio::bank0::Gpio26,
                rp2040_hal::gpio::Input<rp2040_hal::gpio::Floating>,
            >,
        >>::read(&mut self.adc, &mut self.vsense_pin);
        // self.vsense_enable_pin.set_low().unwrap();
        match r {
            Ok(val) => val,
            Err(_) => 0,
        }
    }

    pub fn key0_pressed(&self) -> bool {
        self.key0.is_low().unwrap()
    }

    pub fn key1_pressed(&self) -> bool {
        self.key1.is_low().unwrap()
    }

    pub fn key2_pressed(&self) -> bool {
        self.key2.is_low().unwrap()
    }

    pub fn key3_pressed(&self) -> bool {
        self.key3.is_low().unwrap()
    }

    pub fn clock_high(&self) -> bool {
        self.second_clock.is_low().unwrap()
    }

    pub fn set_backlight_from_lut(&mut self) {
        let nvm = crate::game::globals::get_nvm();
        let brightness = nvm.settings.get_setting(SettingType::Brightness);
        let effective_brightness = BRIGHTNESS_LUT[brightness.get_value() as usize];
        self.set_backlight_raw(effective_brightness);
    }

    pub fn set_backlight_raw(&mut self, brightness_value: u16) {
        unsafe {
            (*self.backlight_channel_ptr).set_duty(brightness_value);
            (*self.backlight_channel_ptr).enable();
        }
    }

    pub fn start_tone(&mut self, tone: &AudioFrequency) {
        let nvm = crate::game::globals::get_nvm();
        let volume = nvm.settings.get_setting(SettingType::Volume);
        let tone = if volume.get_value() == 0 {
            &AudioFrequency::None
        } else {
            tone
        };
        let tone_settings = tone.get_registers();
        let effective_volume = if tone == &AudioFrequency::None {
            0
        } else {
            VOLUME_LUT[volume.get_value() as usize]
        };
        unsafe {
            (*self.buzzer_pwm_slice_ptr).set_top(tone_settings.get_top());
            (*self.buzzer_pwm_slice_ptr).set_div_int(tone_settings.get_div_int());
            (*self.buzzer_pwm_slice_ptr).set_div_frac(tone_settings.get_div_frac());
            (*self.buzzer_channel_ptr).set_duty(effective_volume);
            (*self.buzzer_channel_ptr).enable();
        }
    }

    pub fn end_tone(&mut self) {
        self.start_tone(&AudioFrequency::None);
    }

    pub fn start_vibrating(&mut self) {
        let nvm = crate::game::globals::get_nvm();
        let enabled = nvm.settings.get_setting(SettingType::Vibration);
        if enabled.get_value() == 1 {
            self.vibe.set_high().unwrap()
        } else {
            self.vibe.set_low().unwrap()
        }
    }

    pub fn stop_vibrating(&mut self) {
        self.vibe.set_low().unwrap()
    }

    fn write_sqw_pin_mode(&mut self, mode: u8) -> () {
        let mut buffer = [0u8; 1];
        self.i2c_bus.write(RTC_ADDRESS, &[0x0E]).unwrap();
        self.i2c_bus.read(RTC_ADDRESS, &mut buffer).unwrap();
        let mut ctrl = buffer[0];

        ctrl &= !0x04; // turn off INTCON
        ctrl &= !0x18; // set freq bits to 0

        ctrl |= mode;

        self.i2c_bus.write(RTC_ADDRESS, &[0x0E, ctrl]).unwrap();
    }

    pub fn get_time(&mut self) -> RealTime {
        let date_time = self.get_date_time();
        date_time.time
    }

    pub fn get_date(&mut self) -> RealDate {
        let date_time = self.get_date_time();
        date_time.date
    }

    pub fn get_date_time(&mut self) -> RealDateTime {
        let mut buffer = [0u8; 7];
        self.i2c_bus.write(RTC_ADDRESS, &[0x00]).unwrap();
        self.i2c_bus.read(RTC_ADDRESS, &mut buffer).unwrap();

        let sec = rtc::bcd_to_dec(buffer[0]);
        let min = rtc::bcd_to_dec(buffer[1]);
        let hr = rtc::bcd_to_dec(buffer[2]);

        let real_time = RealTime { sec, min, hr };

        let dow = buffer[3];
        let dom = rtc::bcd_to_dec(buffer[4]);
        let mon = rtc::bcd_to_dec(buffer[5] & 0b0001_1111);
        let century = (buffer[5] & 0b1000_0000) >> 7;
        let year = rtc::bcd_to_dec(buffer[6]) + (century * 100);

        let real_date = RealDate {
            day_of_week: dow,
            day_of_month: dom,
            month: mon,
            year_since_2k: year,
        };

        RealDateTime {
            time: real_time,
            date: real_date,
        }
    }

    pub fn set_time(&mut self, new_time: &RealTime) {
        let sec_bcd = rtc::dec_to_bcd(new_time.sec);
        let min_bcd = rtc::dec_to_bcd(new_time.min);
        let hr_bcd = rtc::dec_to_bcd(new_time.hr);

        let data = [0x00, sec_bcd, min_bcd, hr_bcd];

        self.i2c_bus.write(RTC_ADDRESS, &data).unwrap();
    }

    pub fn set_date(&mut self, new_date: &RealDate) {
        let dow = new_date.day_of_week;
        let dom_bcd = rtc::dec_to_bcd(new_date.day_of_month);
        let mon_bcd = rtc::dec_to_bcd(new_date.month);
        let century = (new_date.year_since_2k / 100) as u8;
        let year = new_date.year_since_2k % 100;
        let year_bcd = rtc::dec_to_bcd(year);
        let data = [
            0x03,
            dow,
            dom_bcd,
            mon_bcd,
            year_bcd,
            (century << 7) | year_bcd,
        ];

        self.i2c_bus.write(RTC_ADDRESS, &data).unwrap();
    }

    fn page_to_address(page: u16) -> [u8; 2] {
        [
            ((page & 0xFF00) >> 5) as u8, // MSB
            ((page & 0xFF) << 3) as u8,   // LSB
        ]
    }

    pub fn get_nvm_page(&mut self, page: u16) -> [u8; 8] {
        let mut buffer = [0u8; 8];

        let address = Self::page_to_address(page);

        self.i2c_bus.write(self.nvm_addr, &address).unwrap();
        self.i2c_bus.read(self.nvm_addr, &mut buffer).unwrap();

        buffer
    }

    pub fn write_nvm_page(&mut self, page: u16, data: &[u8; 8]) {
        let address = Self::page_to_address(page);

        // We need to send address and data together in one write operation.
        // Therefore, creating a buffer that holds both address and data.
        let mut buffer = [0u8; 10];
        buffer[0] = address[0];
        buffer[1] = address[1];
        buffer[2..].copy_from_slice(data);

        self.i2c_bus.write(self.nvm_addr, &buffer).unwrap();

        // wait for the EEPROM to complete its write
        self.delay.delay_ms(5);
    }

    pub fn init_wfi(&mut self) {
        self.key3
            .set_interrupt_enabled(rp2040_hal::gpio::Interrupt::EdgeLow, true);
        self.second_clock
            .set_interrupt_enabled(rp2040_hal::gpio::Interrupt::EdgeHigh, true);

        unsafe {
            pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0);
        }
    }

    pub fn wfi(&self) {
        cortex_m::asm::wfi();
    }
}
