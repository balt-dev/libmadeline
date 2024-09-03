use crate::Hitbox;

pub const MAX_RUN: f32 = 90.;
pub const RUN_ACCEL: f32 = 1000.;
pub const RUN_REDUCE: f32 = 400.;
pub const AIR_MULT: f32 = 0.65;

pub const JUMP_GRACE_TIME: f32 = 0.1;
pub const JUMP_SPEED: f32 = -105.;
pub const JUMP_H_BOOST: f32 = 40.;
pub const VAR_JUMP_TIME: f32 = 0.2;
pub const CEILING_VAR_JUMP_GRACE: f32 = 0.05;
pub const UPWARD_CORNER_CORRECTION: i32 = 4;

pub const WALL_SPEED_RETENTION_TIME: f32 = 0.06;
pub const WALL_JUMP_CHECK_DIST: f32 = 3.;
pub const WALL_JUMP_FORCE_TIME: f32 = 0.16;
pub const WALL_JUMP_H_SPEED: f32 = MAX_RUN + JUMP_H_BOOST;

pub const WALL_SLIDE_TIME: f32 = 1.2;
pub const WALL_SLIDE_START_MAX: f32 = 20.;

pub const MAX_DASHES: u8 = 1;

pub const CLIMB_MAX_STAMINA: f32 = 110.;
pub const CLIMB_UP_COST: f32 = 100. / 2.2;
pub const CLIMB_STILL_COST: f32 = 10.;
pub const CLIMB_JUMP_COST: f32 = CLIMB_MAX_STAMINA / 4.;
pub const CLIMB_TIRED_THRESHOLD: f32 = 20.;

pub const NORMAL_HITBOX: Hitbox = Hitbox::new(8., 11., -4., -11.);
pub const NORMAL_HURTBOX: Hitbox = Hitbox::new(8., 9., -4., -11.);
pub const DUCK_HITBOX: Hitbox = Hitbox::new(8., 6., -4., -6.);
pub const DUCK_HURTBOX: Hitbox = Hitbox::new(8., 4., -4., -6.);

pub const DASH_V_FLOOR_SNAP_DIST: f32 = 3.;

pub const LIFT_X_CAP: f32 = 250.;
pub const LIFT_Y_CAP: f32 = -130.;

pub const SUPER_JUMP_H: f32 = 260.;
pub const DUCK_SUPER_JUMP_X_MULT: f32 = 1.25;
pub const DUCK_SUPER_JUMP_Y_MULT: f32 = 0.5;
pub const SUPER_WALL_JUMP_VAR_TIME: f32 = 0.25;
pub const SUPER_WALL_JUMP_H: f32 = MAX_RUN + JUMP_H_BOOST * 2.;
pub const SUPER_WALL_JUMP_SPEED: f32 = -160.;

pub const CLIMB_JUMP_BOOST_TIME: f32 = 0.2;

pub const REBOUND_SPEED_X: f32 = 140.;
pub const REBOUND_SPEED_Y: f32 = -120.;
pub const REBOUND_VAR_JUMP_TIME: f32 = 0.15;

pub const REFLECT_BOUND_SPEED: f32 = 220.;

pub const DASH_CORNER_CORRECTION: i32 = 4;
pub const DODGE_SLIDE_SPEED_MULT: f32 = 1.2;
pub const FAST_MAX_FALL: f32 = 240.;
pub const MAX_FALL: f32 = 160.;
pub const FAST_MAX_ACCEL: f32 = 300.;

pub const CLIMB_CHECK_DIST: f32 = 2.;
pub const CLIMB_UP_CHECK_DIST: i32 = 2;
pub const CLIMB_GRAB_Y_MULT: f32 = 0.2;
pub const CLIMB_NO_MOVE_TIME: f32 = 0.1;
pub const CLIMB_UP_SPEED: f32 = -45.;
pub const CLIMB_DOWN_SPEED: f32 = 80.;
pub const CLIMB_SLIP_SPEED: f32 = 30.;
pub const CLIMB_ACCEL: f32 = 900.;
pub const CLIMB_HOP_X: f32 = 100.;

pub const DUCK_CORRECT_CHECK: i32 = 4;
pub const DUCK_CORRECT_SLIDE: f32 = 50.;
pub const DUCK_FRICTION: f32 = 500.;

pub const HALF_GRAV_THRESHOLD: f32 = 40.;
pub const GRAVITY: f32 = 900.;

pub const DASH_COOLDOWN: f32 = 0.2;
pub const DASH_REFILL_COOLDOWN: f32 = 0.1;
pub const DASH_ATTACK_TIME: f32 = 0.3;
pub const DASH_SPEED: f32 = 240.;
pub const SWIM_DASH_SPEED_MULT: f32 = 0.75;
pub const DASH_TIME: f32 = 0.15;
pub const END_DASH_SPEED: f32 = 160.;
pub const END_DASH_UP_MULT: f32 = 0.75;

pub const SWIM_Y_SPEED_MULT: f32 = 0.5;    
pub const SWIM_MAX_RISE: f32 = -60.;
pub const SWIM_MAX: f32 = 80.;
pub const SWIM_UNDERWATER_MAX: f32 = 60.;
pub const SWIM_ACCEL: f32 = 600.;
pub const SWIM_REDUCE: f32 = 400.;