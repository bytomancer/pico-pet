/// Finds the best RP2040 PWM register settings to approximate A440 notes
fn main() {
    let freq_table: Vec<(&str, f32)> = vec![
        ("C4", 261.63),
        ("Cs4", 277.18),
        ("D4", 293.66),
        ("Ds4", 311.13),
        ("E4", 329.63),
        ("F4", 349.23),
        ("Fs4", 369.99),
        ("G4", 392.0),
        ("Gs4", 415.3),
        ("A4", 440.0),
        ("As4", 466.16),
        ("B4", 493.88),
        ("C5", 523.25),
        ("Cs5", 554.37),
        ("D5", 587.33),
        ("Ds5", 622.25),
        ("E5", 659.25),
        ("F5", 698.46),
        ("Fs5", 739.99),
        ("G5", 783.99),
        ("Gs5", 830.61),
        ("A5", 880.0),
        ("As5", 932.33),
        ("B5", 987.77),
        ("C6", 1046.5),
        ("Cs6", 1108.73),
        ("D6", 1174.66),
        ("Ds6", 1244.51),
        ("E6", 1318.51),
        ("F6", 1396.91),
        ("Fs6", 1479.98),
        ("G6", 1567.98),
        ("Gs6", 1661.22),
        ("A6", 1760.0),
        ("As6", 1864.66),
        ("B6", 1975.53),
        ("C7", 2093.0),
    ];

    for &(note, freq) in &freq_table {
        let best_tuple = get_closest_setting(freq);
        println!(
            "            Self::{} => AudioRegisterConfig::new{:?},",
            note, best_tuple
        );
    }
}

fn get_closest_setting(target_frequency: f32) -> (u16, u8, u8) {
    let mut closest_frequency = std::f32::INFINITY;
    let mut best_combo = (0, 0, 0);

    for div_int in 0..=255 {
        for div_frac in 0..=15 {
            let top = (125_000_000.0
                / (target_frequency * (div_int as f32 + div_frac as f32 / 16.0)))
                - 1.0;
            if top > 65535.0 {
                continue;
            }
            let frequency = 125_000_000.0 / (top + 1.0) * (div_int as f32 + div_frac as f32 / 16.0);
            if (frequency - target_frequency).abs() < (closest_frequency - target_frequency).abs() {
                closest_frequency = frequency;
                best_combo = (top as u16, div_int, div_frac);
            }
        }
    }
    best_combo
}
