pub fn add_assign<T: core::ops::AddAssign>(lhs: &mut (T, T), rhs: (T, T)) {
    lhs.0 += rhs.0;
    lhs.1 += rhs.1;
}
