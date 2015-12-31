use cgmath;
use cgmath::{ Vector3, Point, Point3, Angle, Deg, Vector, EuclideanVector, Matrix4 };

use std::ops::Neg;
use std::convert::From;

use glium::glutin::Event;

pub const CAM_POS_STEP: f32 = 0.1;
pub const CAM_DIR_STEP: Deg<f32> = Deg { s: 1.0 };

pub const UP: Vector3<f32> = Vector3{ x: 0.0, y: 1.0, z: 0.0 };

//phi >= 0.0 && phi <= 360.0
//theta > -90.0 && theta < 90.0

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub pos: Point3<f32>,
    pub phi: Deg<f32>,
    pub theta: Deg<f32>,
}

impl Camera {
    pub fn new(pos: Point3<f32>, phi: Deg<f32>, theta: Deg<f32>) -> Camera {
        Camera { pos: pos, phi: phi, theta: theta }.norm_phi().norm_theta()
    }

    pub fn at(pos: Point3<f32>, target: Point3<f32>) -> Camera {
        let dir = target - pos;

        let dir_plane = Vector3::new(dir.x, 0.0, dir.z);

        let phi = dir_plane.angle(Vector3::unit_z().neg()) * dir.x.signum();
        let theta = dir.angle(dir_plane) * dir.y.signum();

        Camera::new(pos, Deg::from(phi), Deg::from(theta))
    }

    pub fn norm_phi(mut self) -> Self {
        self.phi = self.phi.normalize();
        self
    }

    pub fn norm_theta(mut self) -> Self {
        self.theta = cgmath::deg(self.theta.s.max(-89.999).min(89.999));
        self
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
}
