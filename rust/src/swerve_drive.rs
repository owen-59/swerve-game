use godot::global::{atan2, sqrt};
use godot::prelude::*;
use godot::classes::{CharacterBody3D, ICharacterBody3D, Input};
use nalgebra::{SMatrix, matrix};

use crate::optimise::optimise_module_state;

const ROBOT_LENGTH_X: f32 = 0.6;
const ROBOT_WIDTH_Y: f32 = 0.6;

const P1: Vector2 = Vector2::new(ROBOT_LENGTH_X / 2.0, ROBOT_WIDTH_Y / 2.0);
const P2: Vector2 = Vector2::new(ROBOT_LENGTH_X / 2.0, -ROBOT_WIDTH_Y / 2.0);
const P3: Vector2 = Vector2::new(-ROBOT_LENGTH_X / 2.0, ROBOT_WIDTH_Y / 2.0);
const P4: Vector2 = Vector2::new(-ROBOT_LENGTH_X / 2.0, -ROBOT_WIDTH_Y / 2.0);

// const RADIUS = 0.1;
//
const MODULE_POSITIONS: [Vector2; 4] = [P1, P2, P3, P4];

pub struct SwerveModuleState {
    pub velocity: f64,
    pub rotation: f64,
}

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct SwerveDrive {
    base: Base<CharacterBody3D>,
    rotation_velocity: f64
}


#[godot_api]
impl ICharacterBody3D for SwerveDrive {
    fn init(base: Base<CharacterBody3D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        Self {
            base,
            rotation_velocity: 0.0
        }
    }

    // fn physics_process(&mut self, _delta: f64) {
    //     let mut base = self.base_mut();
    //
    // }

    fn physics_process(&mut self, _delta: f64) {
        self.rotation_velocity *= 0.9;
        let mut base = self.base_mut();
        let input = Input::singleton();

        let current_velocity = base.get_velocity();
        base.set_velocity(current_velocity * 0.9);

        let target_vx = input.get_axis("backward", "forward");
        let target_vy = input.get_axis("left", "right");
        // let input_rotation_vector = input.get_vector("rotate_left", "rotate_right", "rotate_down", "rotate_up");
        // let target_omega = input_rotation_vector.angle();
        let target_omega = input.get_axis("rotate_left", "rotate_right") + base.get_rotation().y;
        godot_print!("{}", target_omega);

        let yaw = base.get_rotation().y;
        let robot_vx = -target_vx * yaw.cos() + target_vy * yaw.sin();
        let robot_vy = -target_vx * yaw.sin() + target_vy * yaw.cos();

        let target_states: [SwerveModuleState; 4] = MODULE_POSITIONS.map(|module| {
            let vx_wheel = robot_vx - (target_omega * module.y);
            let vy_wheel = robot_vy - (target_omega * module.x);

            let raw_speed = sqrt((vx_wheel.powf(2.0) + vy_wheel.powf(2.0)).into());
            let raw_angle = atan2(vy_wheel.into(), vx_wheel.into());

            optimise_module_state(raw_speed, raw_angle, yaw.into())
        });

        let mut a: SMatrix<f64, 8, 3> = SMatrix::zeros();
        for i in 1..=4 {
            a[(i*2-1, 1)] = 1.0;
            a[(i*2-1, 2)] = MODULE_POSITIONS[0].x as f64
        }
        for i in 1..=4 {
            a[(i*2-2, 0)] = 1.0;
            a[(i*2-2, 2)] = -MODULE_POSITIONS[0].y as f64
        }

        let mut b: SMatrix<f64, 8, 1> = SMatrix::zeros();
        for i in 0..=3 {
            let vi = target_states[i].velocity;
            let omega_i = target_states[i].rotation;
            let vix = vi * omega_i.cos();
            let viy = vi * omega_i.sin();

            b[(2*i, 0)] = vix;
            b[(2*i+1, 0)] = viy;
            // b[()]
        }
        godot_print!("{}", a);
        godot_print!("{}", b);

        let sum_yi: f32 = (0..4).map(|i| MODULE_POSITIONS[i].y).sum();
        let sum_xi: f32 = (0..4).map(|i| MODULE_POSITIONS[i].x).sum();

        let a_t = matrix![
            4.0, 0.0, -sum_yi;
            0.0, 4.0,  sum_xi;
            -sum_yi, sum_xi, (0..4).map(|i| MODULE_POSITIONS[i].x.powi(2) + MODULE_POSITIONS[i].y.powi(2)).sum();
        ];

        let a_t_inverse = matrix![
            0.25, 0.0, 0.0;
            0.0, 0.25, 0.0;
            0.0, 0.0, (1.0/(0..4).map(|i| MODULE_POSITIONS[i].x.powi(2) + MODULE_POSITIONS[i].y.powi(2)).sum::<f32>());
        ];

        // let a_t = a.transpose();
        // let a_t_by_a = a_t * a;
        // let x = (a_t * a).try_inverse().expect("matrix to not be singular") * a_t * b;

        godot_print!("{}", a_t);
        godot_print!("{}", a_t_inverse);

        // a[(0,0)] = 0.0;


        let mut a: Vec<[f32; 3]> = vec![];
        for pos in MODULE_POSITIONS {
            a.push([1.0,0.0,pos.x]);
            a.push([0.0,1.0,pos.y]);
        }
        let a = nalgebra::DMatrix::from_row_iterator(8, 3, a.into_iter().flatten());
        let a_svd = a.svd(true, true);
        let a_pseudo_inverse = a_svd.pseudo_inverse(1e-15).unwrap();

        let mut b = vec![];
        for wheel in target_states {
            b.push([wheel.velocity * wheel.rotation.cos()]);
            b.push([wheel.velocity * wheel.rotation.sin()]);
        }
        let b = nalgebra::DMatrix::from_row_iterator(8, 1, b.iter().flatten().cloned().map(|x| x as f32));

        let test = a_pseudo_inverse * b;
        let vx = test[0];
        let vy = test[1];
        let omega = test[2];

        godot_print!("{}", test);

        let velocity = base.get_velocity();
        let current_rotation = base.get_rotation();
        base.set_rotation(current_rotation + Vector3::new(0.0, omega, 0.0));
        base.set_velocity(velocity + Vector3::new(vx,0.0,vy));
        base.move_and_slide();
    }
}
