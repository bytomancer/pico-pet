use super::audio_track::AudioTrack;

/// Unique identifier and BEAT loader for all audio tracks in the game.
pub enum AudioId {
    BallGame,

    FerrisCry,

    PomodoroFinished,
    BreakFinished,
    Countdown321,
    CountdownGo,

    ButtonBeep,
}
impl AudioId {
    pub fn get_track(&self) -> AudioTrack {
        match self {
            AudioId::BallGame => {
                AudioTrack::new(include_bytes!("../../../assets/sound_raw/ball_game.beat"))
            }
            AudioId::FerrisCry => {
                AudioTrack::new(include_bytes!("../../../assets/sound_raw/ferris_cry.beat"))
            }

            AudioId::PomodoroFinished => AudioTrack::new(include_bytes!(
                "../../../assets/sound_raw/pomodoro_finish.beat"
            )),
            AudioId::BreakFinished => AudioTrack::new(include_bytes!(
                "../../../assets/sound_raw/break_finish.beat"
            )),
            AudioId::Countdown321 => AudioTrack::new(include_bytes!(
                "../../../assets/sound_raw/countdown_321.beat"
            )),
            AudioId::CountdownGo => AudioTrack::new(include_bytes!(
                "../../../assets/sound_raw/countdown_go.beat"
            )),

            AudioId::ButtonBeep => {
                AudioTrack::new(include_bytes!("../../../assets/sound_raw/button_beep.beat"))
            }
        }
    }
}
