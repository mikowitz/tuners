use crate::math::{greatest_prime_factor, normalize_pair, reduce, two};
use crate::play::{Play, PlaybackMode};
use num::{cast, one, PrimInt};
use std::ops::{Div, Mul, Neg};
use std::time::Duration;

use rodio::{
    source::{SineWave, Source},
    OutputStream, Sink,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ratio<T: PrimInt> {
    numer: T,
    denom: T,
}

impl<T: PrimInt> Ratio<T> {
    #[must_use]
    pub fn new(numer: T, denom: T) -> Self {
        let (numer, denom) = normalize_pair(numer, denom);
        let (numer, denom) = reduce(numer, denom);
        Self { numer, denom }
    }

    #[must_use]
    pub fn complement(&self) -> Self {
        Self::new(two(), one()) / *self
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn pow(&self, exp: i32) -> Self {
        match exp {
            0 => Self::new(one(), one()),
            e if e < 0 => self.complement().pow(-e),
            _ => Self::new(self.numer.pow(exp as u32), self.denom.pow(exp as u32)),
        }
    }

    pub fn limit(&self) -> T {
        greatest_prime_factor(self.numer).max(greatest_prime_factor(self.denom))
    }
}

impl<T: PrimInt> Play for Ratio<T> {
    fn play(&self, mode: PlaybackMode) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let r: f32 = self.into();

        let root = SineWave::new(220.)
            .take_duration(Duration::from_secs_f32(1.))
            .amplify(0.2);

        let above = SineWave::new(220. * r)
            .take_duration(Duration::from_secs_f32(1.))
            .amplify(0.2);

        match mode {
            PlaybackMode::Chord => {
                let sink1 = Sink::try_new(&stream_handle).unwrap();
                let sink2 = Sink::try_new(&stream_handle).unwrap();
                sink1.append(root);
                sink2.append(above);
                sink1.sleep_until_end();
                sink2.sleep_until_end();
            }
            PlaybackMode::Interval => {
                let sink = Sink::try_new(&stream_handle).unwrap();
                sink.append(root);
                sink.append(above);
                sink.sleep_until_end();
            }
        }
    }
}

impl<T: PrimInt> From<&Ratio<T>> for f32 {
    fn from(value: &Ratio<T>) -> Self {
        let n: f32 = cast(value.numer).unwrap();
        let d: f32 = cast(value.denom).unwrap();
        n / d
    }
}

impl<T: PrimInt> Mul<Ratio<T>> for Ratio<T> {
    type Output = Ratio<T>;

    fn mul(self, rhs: Ratio<T>) -> Self::Output {
        Ratio::new(self.numer * rhs.numer, self.denom * rhs.denom)
    }
}

impl<T: PrimInt> Div<Ratio<T>> for Ratio<T> {
    type Output = Ratio<T>;

    fn div(self, rhs: Ratio<T>) -> Self::Output {
        Ratio::new(self.numer * rhs.denom, self.denom * rhs.numer)
    }
}

impl<T: PrimInt> Neg for Ratio<T> {
    type Output = Ratio<T>;

    fn neg(self) -> Self::Output {
        self.complement()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let r = Ratio::new(3, 2);

        assert_eq!(f32::from(&r), 1.5);
    }

    #[test]
    fn new_reduces() {
        let r = Ratio::new(10, 8);

        assert_eq!(r, Ratio { numer: 5, denom: 4 });
    }

    #[test]
    fn new_normalizes() {
        let r = Ratio::new(7, 8);
        let r2 = Ratio::new(15, 4);

        assert_eq!(r, Ratio { numer: 7, denom: 4 });
        assert_eq!(
            r2,
            Ratio {
                numer: 15,
                denom: 8
            }
        );
    }

    #[test]
    fn any_non_1_1_octave_equivalence_is_an_octave() {
        let unison = Ratio::new(1, 1);
        let octave_up = Ratio::new(2, 1);
        let octave_down = Ratio::new(1, 2);
        let octave_up_two = Ratio::new(4, 1);
        let octave_down_two = Ratio::new(1, 4);

        assert_eq!(unison, Ratio { numer: 1, denom: 1 });
        assert_eq!(octave_up, Ratio { numer: 2, denom: 1 });
        assert_eq!(octave_down, Ratio { numer: 2, denom: 1 });
        assert_eq!(octave_up_two, Ratio { numer: 2, denom: 1 });
        assert_eq!(octave_down_two, Ratio { numer: 2, denom: 1 });
    }

    #[test]
    fn mul() {
        let r1 = Ratio::new(4, 3);
        let r2 = Ratio::new(3, 2);

        assert_eq!(r1 * r2, Ratio { numer: 2, denom: 1 });
    }

    #[test]
    fn div() {
        let r1 = Ratio::new(4, 3);
        let r2 = Ratio::new(3, 2);

        assert_eq!(
            r1 / r2,
            Ratio {
                numer: 16,
                denom: 9
            }
        );
        assert_eq!(r2 / r1, Ratio { numer: 9, denom: 8 });
    }

    #[test]
    fn complement() {
        let r1 = Ratio::new(4, 3);
        let r2 = Ratio::new(3, 2);

        assert_eq!(r1.complement(), r2);
        assert_eq!(-r2, r1);
    }

    #[test]
    fn pow() {
        let r = Ratio::new(3, 2);

        assert_eq!(r.pow(0), Ratio { numer: 1, denom: 1 });
        assert_eq!(r.pow(1), r);
        assert_eq!(r.pow(2), Ratio { numer: 9, denom: 8 });
        assert_eq!(
            r.pow(-2),
            Ratio {
                numer: 16,
                denom: 9
            }
        );
    }

    #[test]
    fn limit() {
        assert_eq!(Ratio::new(8, 7).limit(), 7);
        assert_eq!(Ratio::new(9, 8).limit(), 3);
        assert_eq!(Ratio::new(10, 9).limit(), 5);
        assert_eq!(Ratio::new(22, 21).limit(), 11);
    }
}
