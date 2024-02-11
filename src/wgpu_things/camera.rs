use wgpu::{util::DeviceExt, BindingResource};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn from_camera(camera: &Camera) -> Self {
        let view_proj = camera.build_view_projection_matrix();
        Self {
            view_proj: view_proj.into(),
        }
    }

    fn slice(self) -> Vec<u8> {
        bytemuck::cast_slice(&[self]).to_vec()
    }
}

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub camera_buffer: Option<wgpu::Buffer>,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    fn uniform(&self) -> CameraUniform {
        let camera_uniform = CameraUniform::from_camera(&self);
        return camera_uniform;
    }

    pub fn create_binding_resource<'a>(&'a mut self, device: &wgpu::Device) -> BindingResource<'a> {
        let camera_uniform = CameraUniform::from_camera(self);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        self.camera_buffer = Some(camera_buffer);
        self.camera_buffer.as_ref().unwrap().as_entire_binding()
    }
}

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    pub camera: Camera,
}

impl CameraController {
    pub fn new(speed: f32, camera: Camera) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            camera,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        let result = match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        };
        return result;
    }

    pub fn update_camera(&mut self) {
        use cgmath::InnerSpace;
        let camera = &mut self.camera;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }

    pub fn update_camera_buffer(&self, queue: &wgpu::Queue) {
        self.camera
            .camera_buffer
            .as_ref()
            .map(|buff| queue.write_buffer(&buff, 0, &self.camera.uniform().slice()));
    }
}
