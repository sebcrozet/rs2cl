#[test]
// #[test]
// use nalgebra::traits::scalar_op::ScalarMul;
// #[test]
// use nalgebra::traits::dot::Dot;
#[test]
use kernel::Kernel;
// #[test]
// use cl_logic::ClOrd;
#[test]
use expr;
// #[test]
// use nalgebra2cl::*;
#[test]
use pragma;

#[test]
fn addition()
{
    let k = Kernel::new(~"add");

    k.enable_extension(pragma::cl_khr_fp64);

    let in_a = k.param::<~[f64]>(expr::Global);
    let in_b = k.param::<~[f64]>(expr::Global);
    let out  = k.param::<~[f64]>(expr::Global);

    let id = k.var();

    id.assign(k.get_global_id(0));

    out[id].assign(in_a[id] + in_b[id]);

    println(k.to_str());
}

// test
// #[test]
// fn integration_kernel()
// {
//   let k          = @mut Kernel::new(~"integrate");
// 
//   k.enable_extension(pragma::cl_khr_fp64);
// 
//   let velocities = k.named_param::<~[CLVec3f64]>(~"velocities", expr::Global);
//   let positions  = k.named_param::<~[CLVec3f64]>(~"positions", expr::Global);
//   let fext       = k.named_param::<CLVec3f64>(~"fext", expr::Global);
//   let dt         = k.named_param::<f64>(~"dt", expr::Global);
// 
//   // FIXME: let id         = k.global_id();
//   let id = expr::literal(0i32);
// 
//   velocities[id].assign(velocities[id] + fext.scalar_mul(&dt));
//   positions[id].assign(positions[id] + velocities[id].scalar_mul(&dt));
// 
//   println(k.to_str());
// }

// #[test]
// fn lin_pgs_kernel()
// {
//   let k = @mut Kernel::new(~"lin_pgs_solve");
// 
//   k.enable_extension(pragma::cl_khr_fp64);
// 
//   /*
//    * Params
//    */
//   let num        = k.named_param::<i32>(~"num", expr::Const);
//   let id1s       = k.named_param::<~[i32]>(~"id1s", expr::Global);
//   let id2s       = k.named_param::<~[i32]>(~"id2s", expr::Global);
//   let normals    = k.named_param::<~[CLVec3f64]>(~"normals", expr::Global);
//   let inv_masses = k.named_param::<~[f64]>(~"inv_masses", expr::Global);
//   let impulses   = k.named_param::<~[f64]>(~"impulses", expr::Global);
//   let lobounds   = k.named_param::<~[f64]>(~"lobounds", expr::Global);
//   let hibounds   = k.named_param::<~[f64]>(~"hibounds", expr::Global);
//   let objectives = k.named_param::<~[f64]>(~"objectives", expr::Global);
//   let pmasses    = k.named_param::<~[f64]>(~"pmasses", expr::Global);
//   let MJLambdas  = k.named_param::<~[CLVec3f64]>(~"MJLambdas", expr::Global);
// 
//   do k.iterate(expr::literal(0), num) |i|
//   {
//     let d_lambda_i = k.named_var::<f64>(~"d_lambda_i");
//     let id1        = k.named_var::<i32>(~"id1");
//     let id2        = k.named_var::<i32>(~"id2");
// 
//     id1.assign(id1s[i]);
//     id2.assign(id2s[i]);
// 
//     /*
//      * The solver itself
//      */
//     d_lambda_i.assign(objectives[i]);
// 
//     do k.if_(id1.cl_ge(&Zero::zero()))
//     { d_lambda_i.assign(d_lambda_i + normals[i].dot(&MJLambdas[id1])); }
// 
//     do k.if_(id2.cl_ge(&Zero::zero()))
//     { d_lambda_i.assign(d_lambda_i - normals[i].dot(&MJLambdas[id2])); }
// 
//     d_lambda_i.assign(d_lambda_i / pmasses[i]);
// 
//     let lambda_i_0 = k.var::<f64>();
// 
//     lambda_i_0.assign(impulses[i]);
// 
//     impulses[i].assign((lambda_i_0 + d_lambda_i).clamp(&lobounds[i], &hibounds[i]));
// 
//     d_lambda_i.assign(impulses[i] - lambda_i_0);
// 
//     do k.if_(id1.cl_ge(&Zero::zero()))
//     { MJLambdas[id1].assign(MJLambdas[id1] - normals[i].scalar_mul(&(inv_masses[id1] * d_lambda_i))); }
// 
//     do k.if_(id2.cl_ge(&Zero::zero()))
//     { MJLambdas[id2].assign(MJLambdas[id2] + normals[i].scalar_mul(&(inv_masses[id2] * d_lambda_i))); }
//   }
// 
//   println(k.to_str());
// }
