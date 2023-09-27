#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edo {
    divisions: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdoInterval<'a> {
    edo: &'a Edo,
    steps: usize,
}

impl Edo {
    pub fn new(divisions: usize) -> Self {
        Self { divisions }
    }

    pub fn interval(&self, steps: usize) -> EdoInterval {
        EdoInterval { edo: self, steps }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let edo = Edo::new(12);

        assert_eq!(edo, Edo { divisions: 12 });
    }

    #[test]
    fn interval() {
        let edo = Edo::new(12);

        let p5 = edo.interval(7);

        assert_eq!(
            p5,
            EdoInterval {
                edo: &edo,
                steps: 7
            }
        );
    }
}
