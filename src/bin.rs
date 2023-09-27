use tuners::play::{Play, PlaybackMode::*};
use tuners::ratio::Ratio;

fn main() {
    let r = Ratio::new(7, 6);
    r.play(Interval);
    r.play(Chord);
}
