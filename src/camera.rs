use nalgebra_glm as glm;

#[derive(Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

pub struct Camera {
    position: glm::Vec3,
    front: glm::Vec3,
    right: glm::Vec3,
    up: glm::Vec3,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn get_view_matrix(self: &Self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn get_position(self: &Self) -> glm::Vec3 {
        self.position
    }

    pub fn update_position(self: &mut Self, direction: Direction, velocity: f32) {
        match direction {
            Direction::Forward => self.position += velocity * self.front,
            Direction::Backward => self.position -= velocity * self.front,
            Direction::Left => self.position -= velocity * self.right,
            Direction::Right => self.position += velocity * self.right,
            Direction::Up => self.position += velocity * self.up,
            Direction::Down => self.position -= velocity * self.up,
        }
    }

    pub fn update_orientation(self: &mut Self, x_offset: f32, y_offset: f32, sensitivity: f32) {
        (self.pitch, self.yaw) =
            modify_pitch_and_yaw(self.pitch, self.yaw, x_offset, y_offset, sensitivity);

        (self.front, self.right, self.up) =
            calculate_camera_vectors(&glm::vec3(0.0, 1.0, 0.0), self.yaw, self.pitch);
    }
}

pub fn start_from_world_pos(position: glm::Vec3) -> Camera {
    let yaw = -90.0;
    let pitch = 0.0;
    let (front, right, up) = calculate_camera_vectors(&glm::vec3(0.0, 1.0, 0.0), yaw, pitch);
    Camera {
        position,
        front,
        right,
        up,
        yaw,
        pitch,
    }
}

fn radians(value: f32) -> f32 {
    num::Float::to_radians(value)
}

fn calculate_camera_vectors(
    world_up: &glm::Vec3,
    yaw: f32,
    pitch: f32,
) -> (glm::Vec3, glm::Vec3, glm::Vec3) {
    let front = glm::normalize(&glm::vec3(
        radians(yaw).cos() * radians(pitch).cos(),
        radians(pitch).sin(),
        radians(yaw).sin() * radians(pitch).cos(),
    ));

    let right = glm::normalize(&glm::cross(&front, &world_up));
    let up = glm::normalize(&glm::cross(&right, &front));

    (front, right, up)
}

fn modify_pitch_and_yaw(
    pitch: f32,
    yaw: f32,
    x_offset: f32,
    y_offset: f32,
    sensitivity: f32,
) -> (f32, f32) {
    let yaw = yaw + (sensitivity * x_offset);
    let pitch = pitch + (sensitivity * y_offset);
    let pitch = num::clamp(pitch, -89.0, 89.0);

    return (pitch, yaw);
}
