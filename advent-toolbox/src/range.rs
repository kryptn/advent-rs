use std::ops::Add;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Range(pub isize, pub isize);

impl Range {
    pub fn intersects(&self, other: &Self) -> bool {
        !(self.1 <= other.0 || self.0 >= other.1)
    }

    pub fn contained_by(&self, other: &Self) -> bool {
        other.0 <= self.0 && self.1 <= other.1
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && other.1 <= self.1
    }

    pub fn len(&self) -> usize {
        (self.1 - self.0).abs() as usize
    }

    pub fn separate(&self, other: &Self) -> Vec<Self> {
        if self.contained_by(other) || !self.intersects(other) {
            return vec![*self];
        } else if self.contains(other) {
            let left = Range(self.0, other.0);
            let right = Range(other.1, self.1);
            return vec![left, *other, right];
        } else {
            // { ( } )
            // ( { ) }
            if self.0 < other.0 {
                let left = Range(self.0, other.0);
                let middle = Range(other.0, self.1);
                return vec![left, middle];
            } else {
                let middle = Range(self.0, other.1);
                let right = Range(other.1, self.1);
                return vec![middle, right];
            }
        }
    }

    pub fn coalesce(&self, other: &Self) -> Option<Self> {
        // dbg!(self, other);
        if self.intersects(other) || self.0 == other.1 || self.1 == other.0 {
            Some(Range(self.0.min(other.0), self.1.max(other.1)))
        } else {
            None
        }
    }

    pub fn combine(&self, other: &Self) -> Vec<Self> {
        if self.intersects(other) {
            vec![Range(self.0.min(other.0), self.1.max(other.1))]
        } else {
            vec![*self, *other]
        }
    }

    pub fn contains_value(&self, value: isize) -> bool {
        self.0 <= value && value <= self.1
    }
}

impl From<(isize, isize)> for Range {
    fn from(input: (isize, isize)) -> Self {
        Self(input.0, input.1)
    }
}

impl Add<isize> for Range {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs)
    }
}

pub struct Ranges(pub Vec<Range>);

impl Ranges {
    pub fn coalesce(&self) -> Self {
        let mut ranges = self.0.clone();
        let mut out = vec![ranges[0]];
        ranges.sort();
        for range in ranges.iter() {
            let current = out.last_mut().unwrap();
            if let Some(coalesced) = current.coalesce(range) {
                // dbg!(&coalesced);
                *current = coalesced;
            } else {
                out.push(*range);
            }
        }
        Self(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::collections::HashSet;

    #[rstest]
    #[case((10, 15), (11, 16), true)]
    #[case((10, 15), (9, 14), true)]
    #[case((10, 15), (11, 14), true)]
    #[case((10, 15), (9, 16), true)]
    #[case((10, 15), (5, 7), false)]
    #[case((10, 15), (17, 20), false)]
    fn test_intersects(
        #[case] a: impl Into<Range>,
        #[case] b: impl Into<Range>,
        #[case] expected: bool,
    ) {
        let a = a.into();
        let b = b.into();
        assert_eq!(a.intersects(&b), expected);
    }

    #[rstest]
    #[case((10, 15), (15, 20), false)]
    #[case((10, 15), (5, 10), false)]
    fn test_exclusive_intersects(
        #[case] a: impl Into<Range>,
        #[case] b: impl Into<Range>,
        #[case] expected: bool,
    ) {
        let a = a.into();
        let b = b.into();
        assert_eq!(a.intersects(&b), expected);
    }

    #[rstest]
    #[case((10, 15), (10, 15), true, true)]
    #[case((10, 15), (11, 14), true, true)]
    #[case((10, 15), (10, 14), true, true)]
    #[case((10, 15), (11, 15), true, true)]
    #[case((10, 15), (9, 16), true, false)]
    #[case((10, 15), (5, 7), false, false)]
    fn test_contains(
        #[case] a: impl Into<Range>,
        #[case] b: impl Into<Range>,
        #[case] expected_intersects: bool,
        #[case] expected_contains: bool,
    ) {
        let a = a.into();
        let b = b.into();
        assert_eq!(a.intersects(&b), expected_intersects);
        assert_eq!(b.contained_by(&a), expected_contains);
    }

    #[rstest]
    #[case((10, 15), (10, 15), true, true)]
    #[case((10, 15), (9, 15), true, true)]
    #[case((10, 15), (10, 16), true, true)]
    #[case((10, 15), (9, 16), true, true)]
    #[case((10, 15), (9, 14), true, false)]
    #[case((10, 15), (11, 16), true, false)]
    #[case((10, 15), (11, 16), true, false)]
    fn test_contained_by(
        #[case] a: impl Into<Range>,
        #[case] b: impl Into<Range>,
        #[case] expected_intersects: bool,
        #[case] expected_contained_by: bool,
    ) {
        let a = a.into();
        let b = b.into();
        assert_eq!(a.intersects(&b), expected_intersects);
        assert_eq!(a.contained_by(&b), expected_contained_by);
    }

    #[rstest]
    #[case((10, 15), (10, 15), &[(10, 15)])]
    #[case((10, 15), (15, 20), &[(10, 15)])]
    #[case((10, 15), (5, 10), &[(10, 15)])]
    #[case((10, 15), (10, 20), &[(10, 15)])]
    #[case((10, 15), (5, 15), &[(10, 15)])]
    #[case((10, 15), (5, 20), &[(10, 15)])]
    #[case((10, 15), (7, 12), &[(10, 12), (12, 15)])]
    #[case((10, 15), (13, 18), &[(10, 13), (13, 15)])]
    #[case((10, 15), (12, 14), &[(10, 12), (12, 14), (14, 15)])]
    fn test_funnel(
        #[case] a: impl Into<Range>,
        #[case] b: impl Into<Range>,
        #[case] expected: &[(isize, isize)],
    ) {
        let a = a.into();
        let b = b.into();
        let expected = expected
            .into_iter()
            .map(|r| Range::from(*r))
            .collect::<std::collections::HashSet<_>>();
        let separated = a.separate(&b).into_iter().collect::<HashSet<_>>();

        assert_eq!(separated, expected);
        // assert_eq!(separated.len(), expected.len());
        // for r in separated {
        //     assert!(expected.contains(&r));
        // }
    }

    #[rstest]
    #[case(&[(5, 10), (10, 15)], &[(5, 15)])]
    #[case(&[(5, 10), (0, 5)], &[(0, 10)])]
    #[case(&[(5, 10), (0, 15)], &[(0, 15)])]
    #[case(&[(5, 10), (7, 9)], &[(5, 10)])]
    #[case(&[(5, 10), (15, 20)], &[(5, 10), (15, 20)])]

    fn test_coalesce(#[case] ranges: &[(isize, isize)], #[case] expected: &[(isize, isize)]) {
        let ranges = ranges
            .into_iter()
            .map(|r| Range::from(*r))
            .collect::<Vec<_>>();
        let expected = expected
            .into_iter()
            .map(|r| Range::from(*r))
            .collect::<HashSet<_>>();

        let coalesced = Ranges(ranges)
            .coalesce()
            .0
            .into_iter()
            .collect::<HashSet<_>>();
        assert_eq!(coalesced, expected);
    }
}
