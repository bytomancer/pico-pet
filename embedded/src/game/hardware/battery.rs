const NUM_READINGS: usize = 64;
const READINGS_DIV_SHIFT: u32 = 6;
pub struct BatteryState {
    readings: [u16; NUM_READINGS],
    last_read_index: usize,

    tracked_level: BatteryLevel,
}

impl Default for BatteryState {
    fn default() -> Self {
        Self {
            readings: [0u16; NUM_READINGS],
            last_read_index: 0,

            tracked_level: BatteryLevel::ApxPct0,
        }
    }
}

impl BatteryState {
    pub fn get_level(&self) -> BatteryLevel {
        self.tracked_level
    }
    fn push_reading(&mut self, reading: u16) {
        if self.last_read_index >= NUM_READINGS {
            self.last_read_index = 0;
        }
        self.readings[self.last_read_index] = reading;
        self.last_read_index += 1;
    }
    fn update_tracked_level(&mut self) {
        let batt_pct = self.get_percent();
        self.tracked_level = self.tracked_level.test_and_move(batt_pct);
    }
    pub fn update_readings(&mut self) {
        let hw = crate::game::globals::get_hardware();
        let reading = hw.get_vsense();
        self.push_reading(reading as u16);
        self.update_tracked_level();
    }

    const VSENSE_AT_100: u16 = 39_000;
    const VSENSE_AT_0: u16 = 26_000;
    pub fn get_percent(&mut self) -> u8 {
        let avg = self.get_average().unwrap_or(0);
        Self::ilerp(
            Self::VSENSE_AT_0 as u32,
            Self::VSENSE_AT_100 as u32,
            avg as u32,
        )
        .unwrap_or(255)
    }

    /// Generates a number from 0 to 100, based on the % of value between min and max
    fn ilerp(min: u32, max: u32, value: u32) -> Option<u8> {
        if min > max {
            return None;
        } else if value < min {
            return Some(0);
        } else if value > max {
            return Some(100);
        }

        let numerator = value - min;
        let denominator = max - min;
        let pct = (numerator * 100 + denominator / 2) / denominator;

        Some(pct as u8)
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

// const LEVEL_MOVE_THRESHOLD: u8 = 5;
#[derive(Clone, Copy)]
pub enum BatteryLevel {
    ApxPct100,
    ApxPct80,
    ApxPct60,
    ApxPct40,
    ApxPct20,
    ApxPct0,
}
impl BatteryLevel {
    pub fn into_int(self) -> u8 {
        match self {
            BatteryLevel::ApxPct100 => 5,
            BatteryLevel::ApxPct80 => 4,
            BatteryLevel::ApxPct60 => 3,
            BatteryLevel::ApxPct40 => 2,
            BatteryLevel::ApxPct20 => 1,
            BatteryLevel::ApxPct0 => 0,
        }
    }
    fn move_up(self) -> Self {
        match self {
            BatteryLevel::ApxPct100 => BatteryLevel::ApxPct100,
            BatteryLevel::ApxPct80 => BatteryLevel::ApxPct100,
            BatteryLevel::ApxPct60 => BatteryLevel::ApxPct80,
            BatteryLevel::ApxPct40 => BatteryLevel::ApxPct60,
            BatteryLevel::ApxPct20 => BatteryLevel::ApxPct40,
            BatteryLevel::ApxPct0 => BatteryLevel::ApxPct20,
        }
    }
    fn move_down(self) -> Self {
        match self {
            BatteryLevel::ApxPct100 => BatteryLevel::ApxPct80,
            BatteryLevel::ApxPct80 => BatteryLevel::ApxPct60,
            BatteryLevel::ApxPct60 => BatteryLevel::ApxPct40,
            BatteryLevel::ApxPct40 => BatteryLevel::ApxPct20,
            BatteryLevel::ApxPct20 => BatteryLevel::ApxPct0,
            BatteryLevel::ApxPct0 => BatteryLevel::ApxPct0,
        }
    }
    pub fn test_and_move(self, batt_pct: u8) -> Self {
        let range = self.movement_range();
        if let Some(move_up) = range.move_up {
            if batt_pct > move_up {
                return self.move_up();
            }
        }
        if let Some(move_down) = range.move_down {
            if batt_pct < move_down {
                return self.move_down();
            }
        }

        self
    }
    // These battery levels are weird, because the visuals don't define it well...
    // Apx100 --(85%)-> Apx80 --(65%)-> Apx60 --(45%)-> Apx40 --(25%)-> Apx20 --( 5%)-> Apx0
    // Apx100 <-(90%)-- Apx80 <-(70%)-- Apx60 <-(50%)-- Apx40 <-(30%)-- Apx20 <-(10%)-- Apx0
    fn movement_range(&self) -> BatteryLevelRange {
        match self {
            Self::ApxPct100 => BatteryLevelRange {
                move_up: None,
                move_down: Some(85),
            },
            Self::ApxPct80 => BatteryLevelRange {
                move_up: Some(90),
                move_down: Some(65),
            },
            Self::ApxPct60 => BatteryLevelRange {
                move_up: Some(70),
                move_down: Some(45),
            },
            Self::ApxPct40 => BatteryLevelRange {
                move_up: Some(50),
                move_down: Some(25),
            },
            Self::ApxPct20 => BatteryLevelRange {
                move_up: Some(30),
                move_down: Some(5),
            },
            Self::ApxPct0 => BatteryLevelRange {
                move_up: Some(10),
                move_down: None,
            },
        }
    }
}
struct BatteryLevelRange {
    move_up: Option<u8>,
    move_down: Option<u8>,
}
