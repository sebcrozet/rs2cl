use nalgebra::vec::{Vec1, Vec2, PVec3, Vec4};
use cl_type::CLType;

impl<N: CLType + ToStr> CLType for Vec4<N>{
    fn to_cl_type_str(_: Option<Vec4<N>>) -> ~str {
        CLType::to_cl_type_str(None::<N>) + "4"
    }

    fn to_cl_literal_str(&self) -> ~str {
        CLType::to_cl_type_str(None::<Vec4<N>>) + "(" +
            self.x.to_str() + ", " +
            self.y.to_str() + ", " +
            self.z.to_str() + ", " +
            self.w.to_str() + ")"
    }
}

// FIXME: should this be in homogeneous coordinates?
impl<N: CLType + ToStr> CLType for PVec3<N>{
    fn to_cl_type_str(_: Option<PVec3<N>>) -> ~str {
        CLType::to_cl_type_str(None::<N>) + "4"
    }

    fn to_cl_literal_str(&self) -> ~str {
        CLType::to_cl_type_str(None::<PVec3<N>>) + "(" +
            self.x.to_str() + ", " +
            self.y.to_str() + ", " +
            self.z.to_str() + ", 0.0)"
    }
}

impl<N: CLType + ToStr> CLType for Vec2<N>{
    fn to_cl_type_str(_: Option<Vec2<N>>) -> ~str {
        CLType::to_cl_type_str(None::<N>) + "2"
    }

    fn to_cl_literal_str(&self) -> ~str {
        CLType::to_cl_type_str(None::<Vec2<N>>) + "(" +
            self.x.to_str() + ", " +
            self.y.to_str() + ")"
    }
}

impl<N: CLType + ToStr> CLType for Vec1<N>{
    fn to_cl_type_str(_: Option<Vec1<N>>) -> ~str {
        CLType::to_cl_type_str(None::<N>)
    }

    fn to_cl_literal_str(&self) -> ~str {
        CLType::to_cl_type_str(None::<Vec1<N>>) + " " + self.x.to_str()
    }
}
