use crate::{DMat3, DMat4, DQuat, DVec3, Mat3, Mat3A, Mat4, Quat, Vec3, Vec3A};

/// Euler rotation sequences.
///
/// The angles are applied starting from the right.
/// E.g. XYZ will first apply the z-axis rotation.
///
/// YXZ can be used for yaw (y-axis), pitch (x-axis), roll (z-axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EulerRot {
    // usual orderings
    XYZ,
    XZY,
    YZX,
    YXZ,
    ZXY,
    ZYX,

    // first axis repeated
    XZX,
    XYX,
    YXY,
    YZY,
    ZYZ,
    ZXZ,

    // relative orderings
    XYZr,
    XZYr,
    YZXr,
    YXZr,
    ZXYr,
    ZYXr,

    // relative first axis repeated
    XZXr,
    XYXr,
    YXYr,
    YZYr,
    ZYZr,
    ZXZr,
}

pub(crate) trait ToEuler {
    type Scalar;
    fn to_euler_angles(self, order: EulerRot) -> (Self::Scalar, Self::Scalar, Self::Scalar);
}

pub(crate) trait FromEuler {
    type Scalar;
    fn from_euler_angles(
        order: EulerRot,
        i: Self::Scalar,
        j: Self::Scalar,
        k: Self::Scalar,
    ) -> Self;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Parity {
    Odd = 0,
    Even = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Repeated {
    No = 0,
    Yes = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Frame {
    Relative = 0,
    Static = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Order {
    initial_axis: Axis,
    parity_even: bool,
    initial_repeated: bool,
    frame_static: bool,
}

impl Order {
    fn new(initial_axis: Axis, parity: Parity, initial_repeated: Repeated, frame: Frame) -> Self {
        Self {
            initial_axis,
            parity_even: parity == Parity::Even,
            initial_repeated: initial_repeated == Repeated::Yes,
            frame_static: frame == Frame::Static,
        }
    }

    fn from_euler(euler: EulerRot) -> Self {
        match euler {
            EulerRot::XYZ => Self::new(Axis::X, Parity::Even, Repeated::No, Frame::Static),
            EulerRot::XYX => Self::new(Axis::X, Parity::Even, Repeated::Yes, Frame::Static),
            EulerRot::XZY => Self::new(Axis::X, Parity::Odd, Repeated::No, Frame::Static),
            EulerRot::XZX => Self::new(Axis::X, Parity::Odd, Repeated::Yes, Frame::Static),
            EulerRot::YZX => Self::new(Axis::Y, Parity::Even, Repeated::No, Frame::Static),
            EulerRot::YZY => Self::new(Axis::Y, Parity::Even, Repeated::Yes, Frame::Static),
            EulerRot::YXZ => Self::new(Axis::Y, Parity::Odd, Repeated::No, Frame::Static),
            EulerRot::YXY => Self::new(Axis::Y, Parity::Odd, Repeated::Yes, Frame::Static),
            EulerRot::ZXY => Self::new(Axis::Z, Parity::Even, Repeated::No, Frame::Static),
            EulerRot::ZXZ => Self::new(Axis::Z, Parity::Even, Repeated::Yes, Frame::Static),
            EulerRot::ZYX => Self::new(Axis::Z, Parity::Odd, Repeated::No, Frame::Static),
            EulerRot::ZYZ => Self::new(Axis::Z, Parity::Odd, Repeated::Yes, Frame::Static),
            EulerRot::ZYXr => Self::new(Axis::X, Parity::Even, Repeated::No, Frame::Relative),
            EulerRot::XYXr => Self::new(Axis::X, Parity::Even, Repeated::Yes, Frame::Relative),
            EulerRot::YZXr => Self::new(Axis::X, Parity::Odd, Repeated::No, Frame::Relative),
            EulerRot::XZXr => Self::new(Axis::X, Parity::Odd, Repeated::Yes, Frame::Relative),
            EulerRot::XZYr => Self::new(Axis::Y, Parity::Even, Repeated::No, Frame::Relative),
            EulerRot::YZYr => Self::new(Axis::Y, Parity::Even, Repeated::Yes, Frame::Relative),
            EulerRot::ZXYr => Self::new(Axis::Y, Parity::Odd, Repeated::No, Frame::Relative),
            EulerRot::YXYr => Self::new(Axis::Y, Parity::Odd, Repeated::Yes, Frame::Relative),
            EulerRot::YXZr => Self::new(Axis::Z, Parity::Even, Repeated::No, Frame::Relative),
            EulerRot::ZXZr => Self::new(Axis::Z, Parity::Even, Repeated::Yes, Frame::Relative),
            EulerRot::XYZr => Self::new(Axis::Z, Parity::Odd, Repeated::No, Frame::Relative),
            EulerRot::ZYZr => Self::new(Axis::Z, Parity::Odd, Repeated::Yes, Frame::Relative),
        }
    }

    fn angle_order(self) -> (usize, usize, usize) {
        let next_axis = |i| (i + 1) % 3;
        let prev_axis = |i| {
            if i > 0 {
                i - 1
            } else {
                2
            }
        };
        let i = self.initial_axis as usize;
        let j = if self.parity_even {
            next_axis(i)
        } else {
            prev_axis(i)
        };
        let k = if self.parity_even {
            prev_axis(i)
        } else {
            next_axis(i)
        };
        (i, j, k)
    }
}

macro_rules! impl_euler_to_mat3 {
    ($scalar:ident, $mat3:ident, $vec3:ident) => {
        impl FromEuler for $mat3 {
            type Scalar = $scalar;
            fn from_euler_angles(
                euler: EulerRot,
                x: Self::Scalar,
                y: Self::Scalar,
                z: Self::Scalar,
            ) -> Self {
                use crate::$scalar::math;

                let order = Order::from_euler(euler);
                let (i, j, k) = order.angle_order();

                let mut angles = if order.frame_static {
                    $vec3::new(x, y, z)
                } else {
                    $vec3::new(z, y, x)
                };

                if !order.parity_even {
                    angles = -angles;
                }

                let (si, ci) = math::sin_cos(angles.x);
                let (sj, cj) = math::sin_cos(angles.y);
                let (sh, ch) = math::sin_cos(angles.z);

                let cc = ci * ch;
                let cs = ci * sh;
                let sc = si * ch;
                let ss = si * sh;

                let mut m = [[0.0; 3]; 3];

                if order.initial_repeated {
                    m[i][i] = cj;
                    m[j][i] = sj * si;
                    m[k][i] = sj * ci;
                    m[i][j] = sj * sh;
                    m[j][j] = -cj * ss + cc;
                    m[k][j] = -cj * cs - sc;
                    m[i][k] = -sj * ch;
                    m[j][k] = cj * sc + cs;
                    m[k][k] = cj * cc - ss;
                } else {
                    m[i][i] = cj * ch;
                    m[j][i] = sj * sc - cs;
                    m[k][i] = sj * cc + ss;
                    m[i][j] = cj * sh;
                    m[j][j] = sj * ss + cc;
                    m[k][j] = sj * cs - sc;
                    m[i][k] = -sj;
                    m[j][k] = cj * si;
                    m[k][k] = cj * ci;
                }

                $mat3::from_cols_array_2d(&m)
            }
        }
    };
}

macro_rules! impl_euler_to_mat4 {
    ($scalar:ident, $mat4:ident, $mat3:ident) => {
        impl FromEuler for $mat4 {
            type Scalar = $scalar;
            fn from_euler_angles(
                euler: EulerRot,
                x: Self::Scalar,
                y: Self::Scalar,
                z: Self::Scalar,
            ) -> Self {
                $mat4::from_mat3($mat3::from_euler_angles(euler, x, y, z))
            }
        }
    };
}

macro_rules! impl_euler_to_quat {
    ($scalar:ident, $quat:ident, $vec3:ident) => {
        impl FromEuler for $quat {
            type Scalar = $scalar;
            fn from_euler_angles(
                euler: EulerRot,
                x: Self::Scalar,
                y: Self::Scalar,
                z: Self::Scalar,
            ) -> Self {
                use crate::$scalar::math;

                let order = Order::from_euler(euler);
                let (i, j, k) = order.angle_order();

                let mut angles = if order.frame_static {
                    $vec3::new(x, y, z)
                } else {
                    $vec3::new(z, y, x)
                };

                if !order.parity_even {
                    angles.y = -angles.y;
                }

                let ti = angles.x * 0.5;
                let tj = angles.y * 0.5;
                let th = angles.z * 0.5;
                let (si, ci) = math::sin_cos(ti);
                let (sj, cj) = math::sin_cos(tj);
                let (sh, ch) = math::sin_cos(th);
                let cc = ci * ch;
                let cs = ci * sh;
                let sc = si * ch;
                let ss = si * sh;

                let parity = if order.parity_even { 1.0 } else { -1.0 };

                let mut a = [0.0; 4];

                if order.initial_repeated {
                    a[i] = cj * (cs + sc);
                    a[j] = sj * (cc + ss) * parity;
                    a[k] = sj * (cs - sc);
                    a[3] = cj * (cc - ss);
                } else {
                    a[i] = cj * sc - sj * cs;
                    a[j] = (cj * ss + sj * cc) * parity;
                    a[k] = cj * cs - sj * sc;
                    a[3] = cj * cc + sj * ss;
                }

                $quat::from_array(a)
            }
        }
    };
}

macro_rules! impl_mat3_to_euler {
    ($scalar:ident, $mat3:ident, $vec3:ident) => {
        impl ToEuler for $mat3 {
            type Scalar = $scalar;
            fn to_euler_angles(
                self,
                euler: EulerRot,
            ) -> (Self::Scalar, Self::Scalar, Self::Scalar) {
                use crate::$scalar::math;

                // TODO: is this just rotation from the inital axis?
                let mat3_from_rotation = |r: $vec3| -> $mat3 {
                    // TODO: switch to glam math wrapper
                    let (sin_rz, cos_rz) = math::sin_cos(r.z);
                    let (sin_ry, cos_ry) = math::sin_cos(r.y);
                    let (sin_rx, cos_rx) = math::sin_cos(r.x);

                    $mat3::from_cols(
                        $vec3::new(cos_rz * cos_ry, sin_rz * cos_ry, -sin_ry),
                        $vec3::new(
                            -sin_rz * cos_rx + cos_rz * sin_ry * sin_rx,
                            cos_rz * cos_rx + sin_rz * sin_ry * sin_rx,
                            cos_ry * sin_rx,
                        ),
                        $vec3::new(
                            -sin_rz * -sin_rx + cos_rz * sin_ry * cos_rx,
                            cos_rz * -sin_rx + sin_rz * sin_ry * cos_rx,
                            cos_ry * cos_rx,
                        ),
                    )
                };

                let order = Order::from_euler(euler);
                let (i, j, k) = order.angle_order();

                let (mut x, mut y, mut z) = if order.initial_repeated {
                    // Extract the first angle, x.
                    let x = math::atan2(self.col(j)[i], self.col(k)[i]);

                    // Remove the x rotation from M, so that the remaining
                    // rotation, N, is only around two axes, and gimbal lock
                    // cannot occur.
                    let mut r = $vec3::ZERO;
                    r[i] = if order.parity_even { -x } else { x };

                    let n = mat3_from_rotation(r) * self;

                    // Extract the other two angles, y and z, from N.
                    let sy = math::sqrt(n.col(j)[i] * n.col(j)[i] + n.col(k)[i] * n.col(k)[i]);
                    let y = math::atan2(sy, n.col(i)[i]);
                    let z = math::atan2(n.col(j)[k], n.col(j)[j]);
                    (x, y, z)
                } else {
                    // Extract the first angle, x.
                    let x = math::atan2(self.col(j)[k], self.col(k)[k]);

                    // Remove the x rotation from M, so that the remaining
                    // rotation, N, is only around two axes, and gimbal lock
                    // cannot occur.
                    let mut r = $vec3::ZERO;
                    r[i] = if order.parity_even { -x } else { x };

                    let n = mat3_from_rotation(r) * self;

                    // Extract the other two angles, y and z, from N.
                    let cy = math::sqrt(n.col(i)[i] * n.col(i)[i] + n.col(i)[j] * n.col(i)[j]);
                    let y = math::atan2(-n.col(i)[k], cy);
                    let z = math::atan2(-n.col(j)[i], n.col(j)[j]);
                    (x, y, z)
                };

                if !order.parity_even {
                    x = -x;
                    y = -y;
                    z = -z;
                }

                if !order.frame_static {
                    let t = x;
                    x = z;
                    z = t;
                }

                (x, y, z)
            }
        }
    };
}

macro_rules! impl_mat4_to_euler {
    ($scalar:ident, $mat4:ident, $mat3:ident) => {
        impl ToEuler for $mat4 {
            type Scalar = $scalar;
            fn to_euler_angles(
                self,
                order: EulerRot,
            ) -> (Self::Scalar, Self::Scalar, Self::Scalar) {
                $mat3::from_mat4(self).to_euler_angles(order)
            }
        }
    };
}

macro_rules! impl_quat_to_euler {
    ($scalar:ident, $quat:ident, $mat3:ident) => {
        impl ToEuler for $quat {
            type Scalar = $scalar;
            fn to_euler_angles(
                self,
                order: EulerRot,
            ) -> (Self::Scalar, Self::Scalar, Self::Scalar) {
                $mat3::from_quat(self).to_euler_angles(order)
            }
        }
    };
}

impl_mat3_to_euler!(f32, Mat3, Vec3);
impl_euler_to_mat3!(f32, Mat3, Vec3);
impl_mat3_to_euler!(f32, Mat3A, Vec3A);
impl_euler_to_mat3!(f32, Mat3A, Vec3A);
impl_euler_to_mat4!(f32, Mat4, Mat3);
impl_mat4_to_euler!(f32, Mat4, Mat3);
impl_quat_to_euler!(f32, Quat, Mat3);
impl_euler_to_quat!(f32, Quat, Vec3);

impl_mat3_to_euler!(f64, DMat3, DVec3);
impl_euler_to_mat3!(f64, DMat3, DVec3);
impl_mat4_to_euler!(f64, DMat4, DMat3);
impl_euler_to_mat4!(f64, DMat4, DMat3);
impl_quat_to_euler!(f64, DQuat, DMat3);
impl_euler_to_quat!(f64, DQuat, DVec3);
