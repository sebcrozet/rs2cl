extern mod rs2cl;

use rs2cl::pragma;
use rs2cl::expr;
use rs2cl::kernel::Kernel;

#[main]
fn addition_kernel()
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
