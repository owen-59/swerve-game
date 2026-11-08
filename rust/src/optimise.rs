use std::f64::consts::PI;

use godot::global::atan2;

use crate::swerve_drive::SwerveModuleState;

pub fn optimise_module_state(
    velocity: f64, 
    angle: f64, 
    current_angle: f64) -> SwerveModuleState {
    let delta = angle - current_angle;
    let delta = atan2(delta.sin(), delta.cos());
    let mut target_angle = angle;
    let mut target_speed = velocity;
    if delta.abs() > PI / 2.0 {
        if delta > 0.0 {
            target_angle -= PI;
        } else {
            target_angle += PI;
        }
        target_speed *= -1.0;
    };

    SwerveModuleState{
        velocity: target_speed,
        rotation: target_angle
    }
}
