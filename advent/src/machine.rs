pub trait Apply<T>
where
    T: Clone,
{
    fn apply(&self, change: T) -> Self;
}

#[cfg(test)]
mod test {

    use super::*;

    #[derive(Clone)]
    enum TestChange {
        Add(i8),
        Sub(i8),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestState {
        value: i8,
    }

    impl Apply<TestChange> for TestState {
        fn apply(&self, change: TestChange) -> Self {
            match change {
                TestChange::Add(v) => Self {
                    value: self.value + v,
                },
                TestChange::Sub(v) => Self {
                    value: self.value - v,
                },
            }
        }
    }

    struct TestMachine {
        changes: Vec<TestChange>,
        memory: TestState,

        inst_ptr: usize,
    }

    impl Iterator for TestMachine {
        type Item = TestState;

        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }

    #[test]
    fn test_thing() {
        let state = TestState { value: 4 };
        let change = TestChange::Add(2);
        let next_state = state.apply(change);

        assert_eq!(next_state, TestState { value: 6 });
    }
}
