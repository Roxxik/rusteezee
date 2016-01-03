use std::fmt;
use std::ops::Neg;

use cgmath;
use cgmath::{ Vector3, Point, Point3, Angle, Deg, Vector, EuclideanVector, Matrix4 };
use bit_set::BitSet;

use super::{ HDirection, VDirection };

const CAM_POS_STEP: f32 = 0.1;
const CAM_DIR_STEP: Deg<f32> = Deg { s: 0.5 };

const UP: Vector3<f32> = Vector3{ x: 0.0, y: 1.0, z: 0.0 };

#[derive(Clone, Copy, Debug)]
enum Direction {
    Forth,
    Back,
    Up,
    Down,
    Left,
    Right,
    TurnUp,
    TurnDown,
    TurnLeft,
    TurnRight,
}

impl Direction {
    pub fn moves() -> Vec<Direction> {
        use self::Direction::*;
        vec![
            Forth,
            Back,
            Up,
            Down,
            Left,
            Right,
        ]
    }

    pub fn turns() -> Vec<Direction> {
        use self::Direction::*;
        vec![
            TurnUp,
            TurnDown,
            TurnLeft,
            TurnRight,
        ]
    }

    pub fn to_vec(&self, phi: Deg<f32>) -> Vector3<f32> {
        use self::Direction::*;
        Vector3::from(match *self {
            Up    => (0.0,  1.0, 0.0),
            Down  => (0.0, -1.0, 0.0),
            Forth |
            Back  |
            Left  |
            Right => {
                if let Some(a) = self.to_angle() {
                    let a = phi + a;
                    (
                        a.sin(),
                        0.0,
                        -a.cos(),
                    )
                } else {
                    (0.0, 0.0, 0.0)
                }
            },
            _ => (0.0, 0.0, 0.0),
        }) * CAM_POS_STEP
    }

    fn to_angle(&self) -> Option<Deg<f32>> {
        use self::Direction::*;
        match *self {
            Forth => Some(0.0),
            Back  => Some(180.0),
            Left  => Some(270.0),
            Right => Some(90.0),
            _ => None
        }.map(cgmath::deg)
    }
}


#[derive(Clone, Debug)]
pub struct Camera {
    pos: Point3<f32>,
    phi: Deg<f32>,
    theta: Deg<f32>,
    state: BitSet,
}

impl Camera {
    pub fn new(pos: Point3<f32>, phi: Deg<f32>, theta: Deg<f32>) -> Camera {
        let mut cam = Camera { pos: pos, phi: phi, theta: theta, state: BitSet::new() };
        cam.norm_phi();
        cam.norm_theta();
        cam
    }

    pub fn at(pos: Point3<f32>, target: Point3<f32>) -> Camera {
        let dir = target - pos;

        let dir_plane = Vector3::new(dir.x, 0.0, dir.z);

        let phi = dir_plane.angle(Vector3::unit_z().neg()) * dir_plane.x.signum();
        let theta = dir.angle(dir_plane) * dir.y.signum();

        Camera::new(pos, Deg::from(phi), Deg::from(theta))
    }

    fn norm_phi(&mut self) {
        self.phi = self.phi.normalize();
    }

    fn norm_theta(&mut self) {
        self.theta = cgmath::deg(self.theta.s.max(-89.999).min(89.999));
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        // forward
        let f = Vector3::new(
            self.theta.cos() * self.phi.sin(),
            self.theta.sin(),
            self.theta.cos() * self.phi.cos().neg(),
        ).normalize();
        // sideways
        let s = f.cross(UP).normalize();
        // up
        let u = s.cross(f);

        Matrix4::new(
            s.x,              u.x,              f.x,              0.0,
            s.y,              u.y,              f.y,              0.0,
            s.z,              u.z,              f.z,              0.0,
            -self.pos.dot(s), -self.pos.dot(u), -self.pos.dot(f), 1.0,
        )
    }

    pub fn add_phi(&mut self, delta_phi: f32) {
        self.phi = self.phi + cgmath::deg(delta_phi);
        self.norm_phi();
    }

    pub fn add_theta(&mut self, delta_theta: f32) {
        self.theta = self.theta + cgmath::deg(delta_theta);
        self.norm_theta();
    }

    pub fn mov(&mut self, dir: HDirection, toogle: bool) {
        use super::HDirection as H;
        use self::Direction as D;
        let dir = match dir {
            H::Forth => D::Forth,
            H::Back  => D::Back,
            H::Left  => D::Left,
            H::Right => D::Right,
        };
        self.set_dir(dir, toogle);
    }

    pub fn turn(&mut self, dir: HDirection, toogle: bool) {
        use super::HDirection as H;
        use self::Direction as D;
        let dir = match dir {
            H::Forth => D::TurnUp,
            H::Back  => D::TurnDown,
            H::Left  => D::TurnLeft,
            H::Right => D::TurnRight,
        };
        self.set_dir(dir, toogle);
    }

    pub fn fly(&mut self, dir: VDirection, toogle: bool) {
        use super::VDirection as V;
        use self::Direction as D;
        let dir = match dir {
            V::Up   => D::Up,
            V::Down => D::Down,
        };
        self.set_dir(dir, toogle);
    }

    fn set_dir(&mut self, dir: Direction, toogle: bool) {
        if toogle {
            self.state.insert(dir as usize);
        } else {
            self.state.remove(&(dir as usize));
        }
    }

    pub fn update(&mut self) {
        for turn in Direction::turns() {
            if self.state.contains(&(turn as usize)) {
                use self::Direction::*;
                match turn {
                    TurnUp    => self.theta = self.theta + CAM_DIR_STEP,
                    TurnDown  => self.theta = self.theta - CAM_DIR_STEP ,
                    TurnLeft  => self.phi = self.phi - CAM_DIR_STEP,
                    TurnRight => self.phi = self.phi + CAM_DIR_STEP,
                    _ => {},
                }
            }
        }
        self.norm_phi();
        self.norm_theta();
        for dir in Direction::moves() {
            if self.state.contains(&(dir as usize)) {
                self.pos = self.pos + dir.to_vec(self.phi);
            }
        }
    }
}

impl fmt::Display for Camera {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "x: {}, y: {}, z: {}, phi: {}, theta: {}",
            self.pos.x,
            self.pos.y,
            self.pos.z,
            self.phi.s,
            self.theta.s,
        )
    }
}
