use nalgebra::vec::Vec3;
use cl_type::CLType;

// FIXME: really 4 ?
impl<N: CLType + ToStr> CLType for Vec3<N>{
    fn to_cl_type_str(_: Option<Vec3<N>>) -> ~str {
        CLType::to_cl_type_str(None::<N>) + "4"
    }

    fn to_cl_literal_str(&self) -> ~str {
        CLType::to_cl_type_str(None::<Vec3<N>>) + "(" +
            self.x.to_str() + ", " +
            self.y.to_str() + ", " +
            self.z.to_str() + ", " +
            "0.0" + ")"
    }
}
