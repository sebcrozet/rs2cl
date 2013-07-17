pub trait ClEq<R>
{
  pub fn cl_eq(&self, &Self) -> R;
  pub fn cl_ne(&self, &Self) -> R;
}

pub trait ClOrd<R>
{
  pub fn cl_ge(&self, &Self) -> R;
  pub fn cl_le(&self, &Self) -> R;
  pub fn cl_gt(&self, &Self) -> R;
  pub fn cl_lt(&self, &Self) -> R;
}
