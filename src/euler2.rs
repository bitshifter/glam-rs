use crate::{Mat3, Quat, Vec3};

pub enum EulerRot {
    // "usual" orderings
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

    // relative orderings -- not common
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

/// Conversion from quaternion to euler angles.
pub(crate) trait QuatToEuler {
    type Scalar;

    /// Compute all angles of a rotation in the notation order
    fn extract_angles(self, order: EulerRot) -> (Self::Scalar, Self::Scalar, Self::Scalar);
}

/// Conversion from euler angles to quaternion.
pub(crate) trait QuatFromEuler {
    type Scalar;

    /// Create the rotation quaternion for the three angles of this euler rotation sequence.
    fn from_euler_angles(order: EulerRot, u: Self::Scalar, v: Self::Scalar, w: Self::Scalar) -> Self;
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

    fn angles(self, x: f32, y: f32, z: f32) -> Vec3 {
        if self.frame_static {
            Vec3::new(x, y, z)
        } else {
            Vec3::new(z, y, x)
        }
    }
}

pub fn euler_to_quat(euler: EulerRot, x: f32, y: f32, z: f32) -> Quat {
    use crate::f32::math;
    let order = Order::from_euler(euler);
    let (i, j, k) = order.angle_order();
    let mut angles = if order.frame_static {
        Vec3::new(x, y, z)
    } else {
        Vec3::new(z, y, x)
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

    Quat::from_array(a)
}

pub fn quat_to_euler(q: Quat, euler: EulerRot) -> (f32, f32, f32) {
    mat3_to_euler(Mat3::from_quat(q), euler)
}

fn mat3_from_rotation(r: Vec3) -> Mat3 {
    use crate::f32::math;

    let (sin_rz, cos_rz) = math::sin_cos(r.z);
    let (sin_ry, cos_ry) = math::sin_cos(r.y);
    let (sin_rx, cos_rx) = math::sin_cos(r.x);

    Mat3::from_cols(
        Vec3::new(cos_rz * cos_ry, sin_rz * cos_ry, -sin_ry),
        Vec3::new(
            -sin_rz * cos_rx + cos_rz * sin_ry * sin_rx,
            cos_rz * cos_rx + sin_rz * sin_ry * sin_rx,
            cos_ry * sin_rx,
        ),
        Vec3::new(
            -sin_rz * -sin_rx + cos_rz * sin_ry * cos_rx,
            cos_rz * -sin_rx + sin_rz * sin_ry * cos_rx,
            cos_ry * cos_rx,
        ),
    )
}

pub fn mat3_to_euler(m: Mat3, euler: EulerRot) -> (f32, f32, f32) {
    use crate::f32::math;
    let order = Order::from_euler(euler);
    let (i, j, k) = order.angle_order();

    let (mut x, mut y, mut z) = if order.initial_repeated {
        // Extract the first angle, x.
        let x = math::atan2(m.col(j)[i], m.col(k)[i]);

        // Remove the x rotation from M, so that the remaining
        // rotation, N, is only around two axes, and gimbal lock
        // cannot occur.
        let mut r = Vec3::ZERO;
        r[i] = if order.parity_even { -x } else { x };

        let n = mat3_from_rotation(r) * m;

        // Extract the other two angles, y and z, from N.
        let sy = math::sqrt(n.col(j)[i] * n.col(j)[i] + n.col(k)[i] * n.col(k)[i]);
        let y = math::atan2(sy, n.col(i)[i]);
        let z = math::atan2(n.col(j)[k], n.col(j)[j]);
        (x, y, z)
    } else {
        // Extract the first angle, x.
        let x = math::atan2(m.col(j)[k], m.col(k)[k]);

        // Remove the x rotation from M, so that the remaining
        // rotation, N, is only around two axes, and gimbal lock
        // cannot occur.
        let mut r = Vec3::ZERO;
        r[i] = if order.parity_even { -x } else { x };

        let n = mat3_from_rotation(r) * m;

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

impl QuatToEuler for Quat {
    type Scalar = f32;
    fn extract_angles(self, order: EulerRot) -> (f32, f32, f32) {
        quat_to_euler(self, order)
    }
}

impl QuatFromEuler for Quat {
    type Scalar = f32;
    fn from_euler_angles(order: EulerRot, i: f32, j: f32, k: f32) -> Self {
        euler_to_quat(order, i, j, k)
    }
}
