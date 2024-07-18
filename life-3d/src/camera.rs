use crate::math::Mat4;

use super::math::Vec3;

pub struct Camera {
    position: Vec3,
    front: Vec3,
    right: Vec3,
    up: Vec3,
}

impl Camera {
    pub fn new(position: &Vec3, front: &Vec3) -> Camera {
        let right = front.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize();

        Camera {
            position: position.clone(),
            front: front.clone(),
            right: right.clone(),
            up: front.cross(&right).normalize(),
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4 {
            data: [
                [self.right.x, self.up.x, self.front.x, self.position.x],
                [self.right.y, self.up.y, self.front.y, self.position.y],
                [self.right.z, self.up.z, self.front.z, self.position.z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}
