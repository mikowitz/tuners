pub enum PlaybackMode {
    Chord,
    Interval,
}

pub trait Play {
    fn play(&self, _mode: PlaybackMode) {}
}
