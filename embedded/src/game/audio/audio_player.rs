use crate::game::hardware::audio::AudioFrequency as Freq;

use super::audio_library::AudioId;
use super::audio_track::AudioTrack;

pub enum RepeatMode {
    Off,
    On,
}
impl Into<bool> for RepeatMode {
    fn into(self) -> bool {
        match self {
            RepeatMode::Off => false,
            RepeatMode::On => true,
        }
    }
}

pub enum AutoPlayMode {
    Off,
    On,
}
impl Into<bool> for AutoPlayMode {
    fn into(self) -> bool {
        match self {
            AutoPlayMode::Off => false,
            AutoPlayMode::On => true,
        }
    }
}

/// An object that represents audio playback.
pub struct AudioPlayer {
    repeat: bool,
    tracker: Option<usize>,
    current_freq: Freq,
    audio_track: AudioTrack,
}
impl AudioPlayer {
    pub fn new(audio_id: AudioId, repeat: RepeatMode, autoplay: AutoPlayMode) -> Self {
        Self {
            repeat: repeat.into(),
            tracker: match autoplay {
                AutoPlayMode::On => Some(0),
                AutoPlayMode::Off => None,
            },
            current_freq: Freq::None,
            audio_track: audio_id.get_track(),
        }
    }

    pub fn play(&mut self) {
        self.tracker = Some(0);
    }

    // TODO (Release): remove if unused
    #[allow(dead_code)]
    pub fn stop(&mut self) {
        self.tracker = None;
    }

    pub fn is_playing(&self) -> bool {
        self.tracker.is_some()
    }

    pub fn tick(&mut self) {
        if let Some(tracker) = self.tracker {
            // if (tracker) / (self.audio_track.playback_rate as usize)
            //     >= self.audio_track.source.len()
            // {
            //     if self.repeat {
            //         self.tracker = Some(0);
            //     } else {
            //         self.tracker = None;
            //         return;
            //     }
            // }
            let note = Freq::from_byte(
                self.audio_track.source[tracker / self.audio_track.playback_rate as usize],
            );

            let hardware = crate::game::globals::get_hardware();
            if note != self.current_freq {
                self.current_freq = note;
                hardware.start_tone(&note);
            }

            if (tracker + 1) / (self.audio_track.playback_rate as usize)
                < self.audio_track.source.len()
            {
                self.tracker = Some(tracker + 1);
            } else if self.repeat {
                self.tracker = Some(0);
            } else {
                self.tracker = None;
                hardware.end_tone();
            }
        }
    }
}
