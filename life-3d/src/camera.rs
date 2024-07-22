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
            up: right.cross(front).normalize(),
        }
    }

    pub fn look_at(position: Vec3, target: Vec3) -> Camera {
        let front = (target - position.clone()).normalize();
        Camera::new(&position, &front)
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4 {
            data: [
                [self.right.x, self.up.x, -self.front.x, 0.0],
                [self.right.y, self.up.y, -self.front.y, 0.0],
                [self.right.z, self.up.z, -self.front.z, 0.0],
                [
                    -self.right.dot(&self.position),
                    -self.up.dot(&self.position),
                    self.front.dot(&self.position),
                    1.0,
                ],
            ],
        }
    }

    pub fn move_relative(&mut self, direction: Vec3) {
        self.position = self.position.clone() + direction * self.front.clone();
    }
}
