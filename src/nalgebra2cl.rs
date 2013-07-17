// XXX: this should be a separate library?
use nalgebra::traits::scalar_op::ScalarMul;
use nalgebra::traits::dot::Dot;
use nalgebra::vec::Vec3;
use cl_type::CLType;

// FIXME make this generic wrt the float type?
pub struct CLVec3f64
{
  priv val:     Vec3<f64>,
  priv padding: f64
}

impl CLVec3f64
{
  pub fn new(val: Vec3<f64>) -> CLVec3f64 
  {
    CLVec3f64 {
      val:     val,
      padding: 0.0
    }
  }
}

impl CLType for CLVec3f64
{
  fn to_cl_type_str() -> ~str
  { ~"double4" }

  fn to_cl_literal_str(&self) -> ~str
  {
    ~"double4(" +
      self.val.at[0].to_str() + ", " +
      self.val.at[1].to_str() + ", " +
      self.val.at[2].to_str() + ", " +
      "0.0" +
    ")"
  }
}

// trait implementation forwarding
impl Dot<f64> for CLVec3f64
{
  fn dot(&self, other: &CLVec3f64) -> f64
  { self.val.dot(&other.val) }
}

impl Add<CLVec3f64, CLVec3f64> for CLVec3f64
{
  fn add(&self, other: &CLVec3f64) -> CLVec3f64
  { CLVec3f64::new(self.val + other.val) }
}

impl Sub<CLVec3f64, CLVec3f64> for CLVec3f64
{
  fn sub(&self, other: &CLVec3f64) -> CLVec3f64
  { CLVec3f64::new(self.val - other.val) }
}

impl ScalarMul<f64> for CLVec3f64
{
  fn scalar_mul(&self, val: &f64) -> CLVec3f64
  { CLVec3f64::new(self.val.scalar_mul(val)) }

  fn scalar_mul_inplace(&mut self, val: &f64)
  { self.val.scalar_mul_inplace(val) }
}
