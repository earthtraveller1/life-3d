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

    fn update_up_and_right(&mut self) {
        self.right = self.front.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize();
        self.up = self.right.cross(&self.front).normalize();
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

pub struct ThirdPersonCamera {
    camera: Camera,
    target: Vec3,
    distance: f32,
    yaw: f32,
    pitch: f32,
}

impl ThirdPersonCamera {
    pub fn new(target: Vec3, distance: f32, yaw: f32, pitch: f32) -> ThirdPersonCamera {
        let camera_position = (Vec3 {
            x: yaw.to_radians().cos() * pitch.to_radians().cos(),
            y: pitch.to_radians().sin(),
            z: yaw.to_radians().sin() * pitch.to_radians().cos(),
        } * distance)
            + target;

        let camera_front = (target - camera_position).normalize();

        ThirdPersonCamera {
            camera: Camera::new(&camera_position, &camera_front),
            target,
            distance,
            yaw,
            pitch,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        self.camera.view_matrix()
    }

    pub fn rotate_camera(&mut self, yaw: f32, pitch: f32) {
        self.yaw += yaw;
        self.pitch += pitch;

        self.camera.position = (Vec3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        } * self.distance)
            + self.target;
        self.camera.front = (self.target - self.camera.position).normalize();

        self.camera.update_up_and_right();
    }
}
