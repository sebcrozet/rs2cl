use expr::{Expr, TypedExpr};
use cl_type::CLType;
use indent::Indent;

pub struct If(@TypedExpr<bool>);
pub struct End;

impl Expr for If
{
  fn to_cl_str(&self, indent: &mut Indent) -> ~str
  {
    let res = indent.to_str() + "\n" +
              indent.to_str() + "if (" + (**self).to_cl_str(indent) + ")\n" +
              indent.to_str() + "{";

    indent.offset = indent.offset + 2;

    res
  }
}

impl Expr for End
{
  fn to_cl_str(&self, indent: &mut Indent) -> ~str
  {
    indent.offset = indent.offset - 2;

    indent.to_str() + "}\n"
  }
}

// XXX: this is a simplified for loop. Replace that by a real for loop.
pub struct Iterate
{
  begin: @TypedExpr<u32>,
  end:   @TypedExpr<u32>,
  i:     @TypedExpr<u32>
}

impl Iterate
{
  pub fn new(begin: @TypedExpr<u32>, end: @TypedExpr<u32>, i: @TypedExpr<u32>) -> Iterate
  {
    Iterate {
      begin: begin,
      end:   end,
      i:     i
    }
  }
}

impl Expr for Iterate
{
  fn to_cl_str(&self, indent: &mut Indent) -> ~str
  {
    let res = indent.to_str() + "\n" +
              indent.to_str() + "for (" + CLType::to_cl_type_str::<u32>() + " " + self.i.to_cl_str(indent) + " = " + self.begin.to_cl_str(indent) +
                                "; " + self.i.to_cl_str(indent) + " < " + self.end.to_cl_str(indent) +
                                "; ++" + self.i.to_cl_str(indent) + ")\n" +
              indent.to_str() + "{";

    indent.offset = indent.offset + 2;

    res
  }
}
