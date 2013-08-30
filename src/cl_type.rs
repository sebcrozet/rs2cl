pub trait CLType
{
  fn to_cl_type_str(dummy_self: Option<Self>) -> ~str;
  fn to_cl_literal_str(&self) -> ~str;
}

impl<T: CLType> CLType for ~[T]
{
  fn to_cl_type_str(_: Option<~[T]>) -> ~str
  { CLType::to_cl_type_str(None::<T>) + "*" }

  fn to_cl_literal_str(&self) -> ~str
  { ~"0" } // FIXME: valid?
}

macro_rules! primitive_impl(
  ($t: ty, $expr: expr) => (
    impl CLType for $t
    {
      fn to_cl_type_str(_: Option<$t>) -> ~str
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
