use crate::{IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};
use encase::{
    matrix::{impl_matrix, AsMutMatrixParts, AsRefMatrixParts, FromMatrixParts, MatrixScalar},
    vector::impl_vector,
};

impl_vector!(2, Vec2, f32; using AsRef AsMut From);
impl_vector!(2, UVec2, u32; using AsRef AsMut From);
impl_vector!(2, IVec2, i32; using AsRef AsMut From);

impl_vector!(3, Vec3, f32; using AsRef AsMut From);
impl_vector!(3, UVec3, u32; using AsRef AsMut From);
impl_vector!(3, IVec3, i32; using AsRef AsMut From);

impl_vector!(4, Vec4, f32; using AsRef AsMut From);
impl_vector!(4, UVec4, u32; using AsRef AsMut From);
impl_vector!(4, IVec4, i32; using AsRef AsMut From);

macro_rules! impl_matrix_traits {
    ($c:literal, $r:literal, $type:ty, $el_ty:ty) => {
        impl AsRefMatrixParts<$el_ty, $c, $r> for $type
        where
            Self: AsRef<[$el_ty; $r * $c]>,
            $el_ty: MatrixScalar,
        {
            fn as_ref_parts(&self) -> &[[$el_ty; $r]; $c] {
                unsafe {
                    ::core::mem::transmute::<&[$el_ty; $r * $c], &[[$el_ty; $r]; $c]>(self.as_ref())
                }
            }
        }

        impl AsMutMatrixParts<$el_ty, $c, $r> for $type
        where
            Self: AsMut<[$el_ty; $r * $c]>,
            $el_ty: MatrixScalar,
        {
            fn as_mut_parts(&mut self) -> &mut [[$el_ty; $r]; $c] {
                unsafe {
                    ::core::mem::transmute::<&mut [$el_ty; $r * $c], &mut [[$el_ty; $r]; $c]>(
                        self.as_mut(),
                    )
                }
            }
        }

        impl FromMatrixParts<$el_ty, $c, $r> for $type {
            fn from_parts(parts: [[$el_ty; $r]; $c]) -> Self {
                Self::from_cols_array_2d(&parts)
            }
        }
    };
}

impl_matrix_traits!(2, 2, Mat2, f32);
impl_matrix_traits!(3, 3, Mat3, f32);
impl_matrix_traits!(4, 4, Mat4, f32);

impl_matrix!(2, 2, Mat2, f32);
impl_matrix!(3, 3, Mat3, f32);
impl_matrix!(4, 4, Mat4, f32);

#[cfg(test)]
mod test {
    use crate::{Mat3, Vec3};
    use encase::StorageBuffer;

    #[test]
    fn vec3() {
        let v = Vec3::new(1.12, 3.04, 6.75);

        let mut buf = [255u8; 12];
        let mut s_buf = StorageBuffer::new(&mut buf);

        s_buf.write(&v).unwrap();
        assert_eq!(&s_buf.as_ref()[0..][..4], &v.x.to_le_bytes());
        assert_eq!(&s_buf.as_ref()[4..][..4], &v.y.to_le_bytes());
        assert_eq!(&s_buf.as_ref()[8..][..4], &v.z.to_le_bytes());

        let mut v_out = Vec3::ZERO;
        s_buf.read(&mut v_out).unwrap();
        assert_eq!(v, v_out);

        assert_eq!(v, s_buf.create().unwrap());
    }

    #[test]
    fn mat3() {
        let m = Mat3::from_cols(
            [1.12, 3.04, 6.75].into(),
            [8.65, 2.34, 94.6].into(),
            [5.45, 6.34, 89.41].into(),
        );

        let mut buf = [255u8; 48];
        let mut s_buf = StorageBuffer::new(&mut buf);

        s_buf.write(&m).unwrap();
        assert_eq!(&s_buf.as_ref()[0..][..4], &m.x_axis.x.to_le_bytes());
        assert_eq!(&s_buf.as_ref()[4..][..4], &m.x_axis.y.to_le_bytes());
        assert_eq!(&s_buf.as_ref()[8..][..4], &m.x_axis.z.to_le_bytes());

        assert_eq!(&s_buf.as_ref()[16..][..4], &m.y_axis.x.to_le_bytes());
        assert_eq!(&s_buf.as_ref()[20..][..4], &m.y_axis.y.to_le_bytes());
        assert_eq!(&s_buf.as_ref()[24..][..4], &m.y_axis.z.to_le_bytes());

        assert_eq!(&s_buf.as_ref()[32..][..4], &m.z_axis.x.to_le_bytes());
        assert_eq!(&s_buf.as_ref()[36..][..4], &m.z_axis.y.to_le_bytes());
        assert_eq!(&s_buf.as_ref()[40..][..4], &m.z_axis.z.to_le_bytes());

        let mut m_out = Mat3::ZERO;
        s_buf.read(&mut m_out).unwrap();
        assert_eq!(m, m_out);

        assert_eq!(m, s_buf.create().unwrap());
    }
}
