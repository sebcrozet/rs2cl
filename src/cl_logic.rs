pub trait ClEq<R> {
    fn cl_eq(&self, &Self) -> R;
    fn cl_ne(&self, &Self) -> R;
}

pub trait ClOrd<R> {
    fn cl_ge(&self, &Self) -> R;
    fn cl_le(&self, &Self) -> R;
    fn cl_gt(&self, &Self) -> R;
    fn cl_lt(&self, &Self) -> R;
}
