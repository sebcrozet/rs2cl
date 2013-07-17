pub trait CLType
{
  fn to_cl_type_str()         -> ~str;
  fn to_cl_literal_str(&self) -> ~str;
}

impl<T: CLType> CLType for ~[T]
{
  fn to_cl_type_str() -> ~str
  { CLType::to_cl_type_str::<T>() + "*" }

  fn to_cl_literal_str(&self) -> ~str
  { ~"0" } // FIXME: valid?
}

macro_rules! primitive_impl(
  ($t: ty, $expr: expr) => (
    impl CLType for $t
    {
      fn to_cl_type_str() -> ~str
      { ~$expr }

      fn to_cl_literal_str(&self) -> ~str
      { self.to_str() }
    }
  )
)

primitive_impl!(i32,  "int")
primitive_impl!(i64,  "long")
primitive_impl!(u32,  "unsigned int")
primitive_impl!(u64,  "unsigned long")
primitive_impl!(f32,  "float")
primitive_impl!(f64,  "double")
primitive_impl!(bool, "bool")
