use expr::{Expr, TypedExpr};
use indent::Indent;

pub struct If(@TypedExpr<bool>);
pub struct EndIf;

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

impl Expr for EndIf
{
  fn to_cl_str(&self, indent: &mut Indent) -> ~str
  {
    indent.offset = indent.offset - 2;

    indent.to_str() + "}\n"
  }
}
