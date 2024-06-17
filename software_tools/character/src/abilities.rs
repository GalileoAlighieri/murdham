use std::collections::BTreeSet;

use super::utils::*;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Abilities {
    pub str: i8,
    pub agi: i8,
    pub wit: i8,
}

static_gen_fn!(starting_arrays, gen_starting_arrays, Abilities, [i8; 3], {
    let min = 5i8;
    let max = 14i8;
    let mut values = BTreeSet::new();
    for a in min..=max {
        for b in min..=max {
            for c in min..=max {
                if a + b + c == 24 {
                    let mut array = [a, b, c];
                    array.sort_by(|a, b| b.cmp(a));
                    values.insert(array);
                }
            }
        }
    }
    values.into_iter().collect()
});

impl From<[i8; 3]> for Abilities {
    fn from(v: [i8; 3]) -> Self {
        Abilities {
            str: v[0],
            agi: v[1],
            wit: v[2],
        }
    }
}

impl std::ops::AddAssign<&Abilities> for Abilities {
    fn add_assign(&mut self, rhs: &Self) {
        self.str += rhs.str;
        self.agi += rhs.agi;
        self.wit += rhs.wit;
    }
}

impl std::ops::Add<&Abilities> for Abilities {
    type Output = Abilities;

    fn add(mut self, rhs: &Abilities) -> Self::Output {
        self += rhs;
        self
    }
}

impl Abilities {
    pub fn new(str: i8, agi: i8, wit: i8) -> Self {
        Self { str, agi, wit }
    }
}
