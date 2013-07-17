pub enum Pragma
{
  Enable(Extension),
  Disable(Extension)
}

// FIXME: it could be cool to detect automatically which extension are needed!
pub enum Extension
{
  cl_khr_fp64
}

impl ToStr for Pragma
{
  fn to_str(&self) -> ~str
  {
    match *self
    {
      Enable(ref ext)  => "#pragma OPENCL " + ext.to_str() + " : enable",
      Disable(ref ext) => "#pragma OPENCL " + ext.to_str() + " : disable"
    }
  }
}

impl ToStr for Extension
{
  fn to_str(&self) -> ~str
  {
    match *self
    {
      cl_khr_fp64 => ~"EXTENSION cl_khr_fp64"
    }
  }
}
