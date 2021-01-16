use bevy::{input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel}, prelude::*};

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(orbit_camera.system());
    }
}

pub struct OrbitCamera {
    /// radians per pixel of mouse movement
    pub orbit_speed:  f32,
    /// percentage of distance between point and orbit origin per pixel
    /// 1 line scroll is equivalent to 16px
    pub zoom_speed:  f32,
    /// distance per pixel of mouse movement
    pub pan_speed: f32,
    pub origin: Vec3
}

impl Default for OrbitCamera {
    fn default() -> Self {
        OrbitCamera {
            orbit_speed: 0.01,
            zoom_speed: 0.01,
            pan_speed: 0.02,
            origin: Vec3::zero()
        }
    }
}

#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

enum Mode {
    Pan,
    Orbit
}

fn orbit_camera(
    mut state: Local<State>,
    cameras: Query<(&mut Transform, &mut OrbitCamera)>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mouse_button: Res<Input<MouseButton>>,
    key_code: Res<Input<KeyCode>>
) {
    let mut delta = Vec2 { x: 0f32, y: 0f32 };
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        if mouse_button.pressed(MouseButton::Middle) {
            delta += event.delta;
        }
    }
    let mut zoom = 0.0;
    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        let mul = match event.unit {
            MouseScrollUnit::Line => 16.0,
            MouseScrollUnit::Pixel => 1.0
        };
        zoom += mul * event.y;
    }

    if mouse_button.pressed(MouseButton::Middle) || zoom != 0.0 {
        let mode = if key_code.pressed(KeyCode::LShift) || key_code.pressed(KeyCode::RShift) {
            Mode::Pan
        } else {
            Mode::Orbit
        };
        apply_delta(delta, zoom, mode, cameras);
    }
}

fn apply_delta(
    delta: Vec2,
    zoom: f32,
    mode: Mode,
    mut cameras: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    let camera = cameras.iter_mut().next();
    if let Some((mut camera,mut settings)) = camera {
        let OrbitCamera { zoom_speed, orbit_speed, pan_speed, ref mut origin } = *settings;
        match mode {
            Mode::Orbit => {
                let origin = *origin;
                let rot_y = Quat::from_rotation_y(orbit_speed * delta.x);
                let axis_x = camera.rotation.mul_vec3(Vec3::unit_x());
                let rot_x = Quat::from_axis_angle(axis_x,orbit_speed * delta.y);
                let rot = rot_y * rot_x;
                camera.translation = rot.mul_vec3(camera.translation - origin) + origin;
                camera.look_at(origin, Vec3::unit_y());
            },
            Mode::Pan => {
                let delta: Vec2 = delta * pan_speed;
                let path =  Vec3::new(delta.x * -1.0, delta.y, 0.0);
                let path = camera.rotation.mul_vec3(path);
                *origin += path;
                camera.translation += path;
            }
        }
        // Handle Zoom
        camera.translation = camera.translation.lerp(Vec3::zero(), zoom_speed * zoom);
        camera.look_at(*origin, Vec3::unit_y());
    }
}