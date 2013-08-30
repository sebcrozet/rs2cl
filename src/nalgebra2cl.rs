// XXX: this should be a separate library?
use std::libc;
use std::sys;
use std::ptr;
use std::num::Zero;
use nalgebra::traits::vector::{Vec, AlgebraicVec};
use nalgebra::traits::dim::Dim;
use nalgebra::vec::Vec3;
use cl_type::CLType;
use OpenCL;

// FIXME make this generic wrt the float type?
#[deriving(Eq, ToStr, Clone)]
pub struct CLVec3f64
{
  val:          Vec3<f64>,
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

impl OpenCL::vector::VectorType for CLVec3f64;
impl OpenCL::hl::KernelArg for CLVec3f64
{
  fn get_value(&self) -> (libc::size_t, *libc::c_void)
  {
    (sys::size_of::<CLVec3f64>() as libc::size_t, ptr::to_unsafe_ptr(self) as *libc::c_void)
  }
}

impl CLType for CLVec3f64
{
  fn to_cl_type_str(_: Option<CLVec3f64>) -> ~str
  { ~"double4" }

  fn to_cl_literal_str(&self) -> ~str
  {
    ~"double4(" +
      self.val.x.to_str() + ", " +
      self.val.y.to_str() + ", " +
      self.val.z.to_str() + ", " +
      "0.0" +
    ")"
  }
}

// trait implementation forwarding
impl Zero for CLVec3f64
{
  fn zero() -> CLVec3f64
  { CLVec3f64::new(Zero::zero()) }

  fn is_zero(&self) -> bool
  { self.val.is_zero() }
}

impl Dim for CLVec3f64 {
    fn dim(_: Option<CLVec3f64>) -> uint {
        3
    }
}

impl Neg<CLVec3f64> for CLVec3f64 {
    fn neg(&self) -> CLVec3f64 {
        CLVec3f64::new(-self.val)
    }
}

impl Div<f64, CLVec3f64> for CLVec3f64 {
    fn div(&self, d: &f64) -> CLVec3f64 {
        CLVec3f64::new(self.val / *d)
    }
}

impl AlgebraicVec<f64> for CLVec3f64
{
  fn norm(&self) -> f64
  { self.val.norm() }

  fn sqnorm(&self) -> f64
  { self.val.sqnorm() }

  fn normalized(&self) -> CLVec3f64
  { CLVec3f64::new(self.val.normalized()) }

  fn normalize(&mut self) -> f64
  { self.val.normalize() }
}

impl Vec<f64> for CLVec3f64
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

impl Mul<f64, CLVec3f64> for CLVec3f64
{
  fn mul(&self, val: &f64) -> CLVec3f64
  { CLVec3f64::new(self.val * *val) }
}
