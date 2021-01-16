use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(orbit_camera.system());
    }
}

pub struct OrbitCamera<O = MouseButton, P = KeyCode> {
    /// radians per pixel of mouse movement
    pub orbit_speed: f32,
    /// percentage of distance between point and orbit origin per pixel
    /// 1 line scroll is equivalent to 16px
    pub zoom_speed: f32,
    /// distance per pixel of mouse movement
    pub pan_speed: f32,
    pub orbit_input: O,
    pub pan_input: P,
    origin: Vec3,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        OrbitCamera {
            orbit_speed: 0.01,
            zoom_speed: 0.01,
            pan_speed: 0.02,
            orbit_input: MouseButton::Middle,
            pan_input: KeyCode::LShift,
            origin: Vec3::zero(),
        }
    }
}

#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

// FIX ME: OrbitCamera should be generic (otherwise it will not work if the user does not use the default types)
fn orbit_camera(
    mut state: Local<State>,
    mut cameras: Query<(&mut Transform, &mut OrbitCamera)>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    orbit_input: Res<Input<MouseButton>>,
    pan_input: Res<Input<KeyCode>>,
) {
    let mut delta = Vec2 { x: 0f32, y: 0f32 };
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        if orbit_input.pressed(MouseButton::Middle) {
            delta += event.delta;
        }
    }
    let mut zoom = 0.0;
    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        let mul = match event.unit {
            MouseScrollUnit::Line => 16.0,
            MouseScrollUnit::Pixel => 1.0,
        };
        zoom += mul * event.y;
    }
    for (mut transform, mut camera) in cameras.iter_mut() {
        apply_delta(
            delta,
            zoom,
            &mut transform,
            &mut camera,
            &orbit_input,
            &pan_input,
        );
    }
}

fn apply_delta(
    delta: Vec2,
    zoom: f32,
    mut transform: &mut Transform,
    camera: &mut OrbitCamera,
    orbit_input: &Res<Input<MouseButton>>,
    pan_input: &Res<Input<KeyCode>>,
) {
    let OrbitCamera {
        zoom_speed,
        orbit_speed,
        pan_speed,
        ref mut origin,
        orbit_input: orbit_input_type,
        pan_input: pan_input_type,
    } = *camera;
    if  pan_input.pressed(pan_input_type) {
        let delta: Vec2 = delta * pan_speed;
        let path = Vec3::new(delta.x * -1.0, delta.y, 0.0);
        let path = transform.rotation.mul_vec3(path);
        *origin += path;
        transform.translation += path;
    } else if orbit_input.pressed(orbit_input_type) {
        let origin = *origin;
        let rot_y = Quat::from_rotation_y(orbit_speed * delta.x);
        let axis_x = transform.rotation.mul_vec3(Vec3::unit_x());
        let rot_x = Quat::from_axis_angle(axis_x, orbit_speed * delta.y);
        let rot = rot_y * rot_x;
        transform.translation = rot.mul_vec3(transform.translation - origin) + origin;
        transform.look_at(origin, Vec3::unit_y());
    };
    // Handle Zoom
    transform.translation = transform.translation.lerp(Vec3::zero(), zoom_speed * zoom);
    transform.look_at(*origin, Vec3::unit_y());
}
