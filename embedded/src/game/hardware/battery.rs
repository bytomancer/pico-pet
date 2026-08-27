const NUM_READINGS: usize = 64;
const READINGS_DIV_SHIFT: u32 = 6;
pub struct BatteryState {
    readings: [u16; NUM_READINGS],
    last_read_index: usize,
}

impl Default for BatteryState {
    fn default() -> Self {
        Self {
            readings: [0u16; NUM_READINGS],
            last_read_index: 0,
        }
    }
}

impl BatteryState {
    fn push_reading(&mut self, reading: u16) {
        if self.last_read_index >= NUM_READINGS {
            self.last_read_index = 0;
        }
        self.readings[self.last_read_index] = reading;
        self.last_read_index += 1;
    }
    pub fn update_readings(&mut self) {
        let hw = crate::game::globals::get_hardware();
        let reading = hw.get_vsense();
        self.push_reading(reading);
    }

    pub fn get_average(&self) -> Option<u16> {
        let mut sum = 0u32;
        for reading in self.readings {
            if reading == 0 {
                return None;
            }
            sum += reading as u32
        }
        let sum = sum >> READINGS_DIV_SHIFT;
        Some(sum as u16)
    }
}
