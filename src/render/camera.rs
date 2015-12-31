use cgmath;
use cgmath::{ Vector3, Point, Point3, Angle, Deg, Vector, EuclideanVector, Matrix4 };

use std::ops::{ Neg, Index, IndexMut };
use std::convert::From;

pub const CAM_POS_STEP: f32 = 0.1;
pub const CAM_DIR_STEP: Deg<f32> = Deg { s: 0.5 };

pub const UP: Vector3<f32> = Vector3{ x: 0.0, y: 1.0, z: 0.0 };

//phi >= 0.0 && phi <= 360.0
//theta > -90.0 && theta < 90.0

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Forward,
    Backward,
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
            Forward,
            Backward,
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
            Up       => (0.0,  1.0, 0.0),
            Down     => (0.0, -1.0, 0.0),
            Forward  |
            Backward |
            Left     |
            Right    => {
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
            Forward  => Some(0.0),
            Backward => Some(180.0),
            Left     => Some(270.0),
            Right    => Some(90.0),
            _ => None
        }.map(cgmath::deg)
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub pos: Point3<f32>,
    pub phi: Deg<f32>,
    pub theta: Deg<f32>,
    pub movement: [bool; 10],
}

impl Camera {
    pub fn new(pos: Point3<f32>, phi: Deg<f32>, theta: Deg<f32>) -> Camera {
        let mut cam = Camera { pos: pos, phi: phi, theta: theta, movement: [false; 10] };
        cam.norm_phi();
        cam.norm_theta();
        cam
    }

    pub fn at(pos: Point3<f32>, target: Point3<f32>) -> Camera {
        let dir = target - pos;

        let dir_plane = Vector3::new(dir.x, 0.0, dir.z);

        let phi = dir_plane.angle(Vector3::unit_z().neg()) * dir.x.signum();
        let theta = dir.angle(dir_plane) * dir.y.signum();

        Camera::new(pos, Deg::from(phi), Deg::from(theta))
    }

    pub fn norm_phi(&mut self) {
        self.phi = self.phi.normalize();
    }

    pub fn norm_theta(&mut self) {
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

    pub fn set_dir(&mut self, state: bool, dir: Direction) {
        self[dir] = state;
    }

    pub fn update(&mut self) {
        for turn in Direction::turns() {
            if self[turn] {
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
            if self[dir] {
                self.pos = self.pos + dir.to_vec(self.phi);
            }
        }
    }
}

impl Index<Direction> for Camera {
    type Output = bool;

    fn index(&self, index: Direction) -> &Self::Output {
        use self::Direction::*;
        &self.movement[match index {
            Forward => 0,
            Backward => 1,
            Up => 2,
            Down => 3,
            Left => 4,
            Right => 5,
            TurnUp => 6,
            TurnDown => 7,
            TurnLeft => 8,
            TurnRight => 9,
        }]
    }
}

impl IndexMut<Direction> for Camera {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        use self::Direction::*;
        &mut self.movement[match index {
            Forward => 0,
            Backward => 1,
            Up => 2,
            Down => 3,
            Left => 4,
            Right => 5,
            TurnUp => 6,
            TurnDown => 7,
            TurnLeft => 8,
            TurnRight => 9,
        }]
    }
}
