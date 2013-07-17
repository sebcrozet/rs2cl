use expr::{Location, Expr, TypedExpr};
use expr;
use branching;
use indent::Indent;
use cl_type::CLType;
use pragma::{Enable, Disable, Extension, Pragma};

pub struct Kernel
{
  priv name:          ~str,
  priv last_param_id: uint,
  priv last_var_id:   uint,
  priv pragmas:       ~[Pragma],
  priv params:        ~[@Expr],
  priv exprs:         ~[@Expr]
}

impl Kernel
{
  pub fn new(name: ~str) -> Kernel
  {
    Kernel {
      name:          name,
      last_param_id: 0,
      last_var_id:   0,
      pragmas:       ~[],
      params:        ~[],
      exprs:         ~[]

    }
  }

  pub fn push_expr<T: 'static + Expr>(@mut self, expr: @T)
  {
    self.exprs.push(expr as @Expr);
  }

  pub fn enable_extension(&mut self, ext: Extension)
  {
    // FIXME: check that the same extension is not both enabled and disabled?
    self.pragmas.push(Enable(ext));
  }

  pub fn disable_extension(&mut self, ext: Extension)
  {
    // FIXME: check that the same extension is not both enabled and disabled?
    self.pragmas.push(Disable(ext));
  }

  pub fn get_global_id(@mut self, dim: u32) -> @TypedExpr<u32>
  { @expr::RValue(expr::RStrExpr("get_global_id(" + dim.to_str() + ")")) }

  pub fn param<T: 'static + CLType>(@mut self, location: Location) -> @TypedExpr<T>
  {
    let res = self.named_param(~"rs2cl_p" + self.last_param_id.to_str(), location);

    self.last_param_id = self.last_param_id + 1;

    res
  }

  pub fn var<T: 'static + CLType>(@mut self) -> @TypedExpr<T>
  {
    let res = self.named_var(~"rs2cl_v" + self.last_var_id.to_str());

    self.last_var_id = self.last_var_id + 1;

    res
  }

  pub fn named_param<T: 'static + CLType>(@mut self, name: ~str, location: Location) -> @TypedExpr<T>
  {
    self.params.push(@expr::Param::<T>(name.clone(), location) as @Expr);

    // FIXME: return an rvalue if the location is const?
    @expr::LValue(expr::LVariable(name, location), self)
  }

  pub fn named_var<T: 'static + CLType>(@mut self, name: ~str) -> @TypedExpr<T>
  {
    self.exprs.push(@expr::Declare::<T>(name.clone(), expr::Nowhere) as @Expr);

    @expr::LValue(expr::LVariable(name, expr::Nowhere), self)
  }

  // FIXME: implement else_ and elif_
  pub fn if_(@mut self, cond: @TypedExpr<bool>, f: &fn())
  {
    self.exprs.push(@branching::If(cond) as @Expr);
    f();
    self.exprs.push(@branching::EndIf as @Expr);
  }
}

impl ToStr for Kernel
{
  fn to_str(&self) -> ~str
  {
    let mut indent = Indent::new();

    let mut res = ~"";
    
    // pragmas
    for self.pragmas.iter().advance |p|
    { res = res + p.to_str() + "\n" }

    if !self.pragmas.is_empty()
    { res = res + "\n" }
    
    // signature
    res = res + "__kernel void " + self.name.clone() + "(\n";

    indent.offset = 4;

    let mut iter = self.params.iter();

    match iter.next()
    {
      Some(exp) => {
        res = res + exp.to_cl_str(&mut indent);
        for iter.advance |p|
        { res = res + ",\n" + p.to_cl_str(&mut indent) }
      },
      None => { },
    }

    res = res + "\n)\n";

    // body
    res = res + "{\n";

    indent.offset = 2;

    for self.exprs.iter().advance |e|
    { res = res + e.to_cl_str(&mut indent) + "\n" }

    res = res + "}\n";

    res
  }
}
