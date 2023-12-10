pub fn add_assign<T: core::ops::AddAssign>(lhs: &mut (T, T), rhs: (T, T)) {
    lhs.0 += rhs.0;
    lhs.1 += rhs.1;
}

pub fn add<T: core::ops::Add>(lhs: (T, T), rhs: (T, T)) -> (T::Output, T::Output) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}
