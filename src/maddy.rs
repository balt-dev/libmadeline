/// Don't look at the source code. It's not worth it.

// how the fuck did you write this,
// in C#,
// without going insane
// holy fucking spaghetti code.

use std::{cmp::Ordering, ffi};

use crate::{Color, Facings, Hitbox, Input, Inventory, RumbleLength, RumbleStrength, Vector2};

pub mod constants;
use constants::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum DashCollisionResults {
    #[default]
    ResIgnore,
    ResRebound,
    ResBounce
}

trait FloatExt {
    fn approach(self, target: Self, amount: Self) -> Self;
    fn lerp(self, target: Self, factor: Self) -> Self;
}

impl FloatExt for f32 {
    fn approach(self, target: Self, amount: Self) -> Self {
        // Method implementation based off of the PICO-8 version of Celeste
        // (specifically, the function by the same name)
        if self > target {
            target.max(self - amount)
        } else {
            target.min(self + amount)
        }
    }

    fn lerp(self, target: Self, factor: Self) -> Self {
        self + (target - self) * factor
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
enum DashCoroutineBreakpoint {
    #[default]
    BreakpointStart,
    Breakpoint1,
    Breakpoint2,
}

#[repr(C)]
#[derive(Debug, Default)]
/// An instance of Madeline's movement controller.
/// 
/// Don't set any fields that don't have documentation
/// unless you know what you're doing
/// or it's clear what that will do.
pub struct Madeline {
    
    // Our stuff
    /// Madeline's state. DO NOT manually set this.
    /// Instead, use CLST_SetState.
    pub state: State,
    /// The player's input.
    pub input: Input,

    /// A callback when sound is supposed to be played.
    /// These sounds can be mapped to whatever you want.
    /// 
    /// Note that these sound names are not allocated, and are in fact pointers to static values!
    /// You do not need to, and in fact should not, delete, free, or use CLST_DropDebugString on them.
    pub sound_callback: Option<extern "C" fn(*const ffi::c_char)>,
    /// A callback for controller rumble.
    pub rumble_callback: Option<extern "C" fn(RumbleStrength, RumbleLength)>,
    /// A callback to run to see if a position is solid.
    /// If unset, this will default to false.
    pub collision_callback: Option<extern "C" fn(&Self, Vector2) -> bool>,
    /// A callback to run to see if a position has swimmable water in it.
    /// If unset, this will default to false.
    pub water_callback: Option<extern "C" fn(&Self, Vector2) -> bool>,
    /// A callback to run to see if Madeline can climb a wall at a position.
    /// If unset, this will default to true.
    pub can_climb_callback: Option<extern "C" fn(&Self, Vector2) -> bool>,
    /// A callback that's run when Madeline collides with a solid object while dashing.
    /// If unset, this will default to Ignore.
    pub dash_collision_callback: Option<extern "C" fn(&Self, Vector2) -> DashCollisionResults>,
    /// A callback to call to determine a friction factor.
    /// Returning 0 means no friction, and returning 1 means full friction. 
    /// If unset, this will default to 1.
    pub friction_callback: Option<extern "C" fn(&Self) -> f32>,
    /// Madeline's current inventory.
    pub inventory: Inventory,
    /// The speed of the surface that Madeline is on.
    /// Make sure you set this when Madeline is on top of a moving object!
    pub lift_speed: Vector2,
    pub time_active: f32,
    
    coroutine_timer: f32,
    dash_coroutine_breakpoint: DashCoroutineBreakpoint,

    // Actual fields
    position: Vector2,
    rem_position: Vector2,
    speed: Vector2,
    collider: Hitbox,
    hurtbox: Hitbox,
    facing: Facings,
    just_respawned: bool,
    stamina: f32,

    last_dashes: u8,
    dashes: u8,
    max_dashes: u8,
    dash_attack_timer: f32,
    dash_cooldown_timer: f32,
    dash_refill_cooldown_timer: f32,
    dash_dir: Vector2,
    before_dash_speed: Vector2,
    started_dashing: bool,
    dash_started_on_ground: bool,
    last_aim: Vector2,
    
    idle_timer: f32,
    on_ground: bool,
    was_on_ground: bool,
    wall_slide_dir: i8,
    wall_slide_timer: f32,
    wall_boost_dir: i8,
    wall_boost_timer: f32,
    auto_jump: bool,
    jump_grace_timer: f32,
    var_jump_timer: f32,
    var_jump_speed: f32,
    auto_jump_timer: f32,
    
    move_x: i8,
    force_move_x: i8,
    force_move_x_timer: f32,

    wall_speed_retention_timer: f32,
    wall_speed_retained: f32,

    hair_color: Color,
    hair_flash_timer: f32,
    override_hair_color: Color,
    flash: bool,

    play_footstep_on_land: f32,
    highest_air_y: f32,

    hop_wait_x: i8,
    hop_wait_x_speed: f32,

    was_tired: bool,

    was_ducking: bool,

    max_fall: f32,

    climb_no_move_timer: f32,
    last_climb_move: i8
}

impl Madeline {

    /// Allocates a new instance of Madeline on the heap.
    /// 
    /// **DO NOT DELETE OR FREE THIS.**
    /// Instead, use CLST_Drop to safely deallocate the object.
    #[no_mangle]
    pub extern "C" fn CLST_New(
        position: Vector2
    ) -> *mut Self {
        let mut this = Box::<Self>::default();
        
        this.position = position;
        this.collider = NORMAL_HITBOX;
        this.hurtbox = NORMAL_HURTBOX;
        this.dashes = MAX_DASHES;
        this.last_dashes = MAX_DASHES;
        this.stamina = CLIMB_MAX_STAMINA;
        this.max_dashes = MAX_DASHES;
        this.just_respawned = true;
        this.hair_color = Color::NORMAL_HAIR;
        this.highest_air_y = position.y;

        Box::into_raw(this)
    }

    #[no_mangle]
    /// Deallocates the object allocated from CLST_New.
    /// 
    /// The pointer passed in **must come from CLST_New.**
    pub unsafe extern "C" fn CLST_Drop(this: *mut Self) {
        std::mem::drop(Box::from_raw(this))
    }

    #[no_mangle]
    /// Gets a string representaiton of this object.
    /// This should probably only be used for debugging.
    /// 
    /// **DO NOT DELETE OR FREE THE STRING.**
    /// Instead, pass it to CLST_DropDebugString to safely deallocate it.
    /// 
    /// Along with this, do not alter the string before deallocating it.
    pub extern "C" fn CLST_DebugString(&self) -> *mut ffi::c_char {
        let string = ffi::CString::new(format!("{self:#?}"))
            .expect("debug representation of struct should not have null byte");

        string.into_raw()
    }

    #[no_mangle]
    /// Safely deallocates a string obtained via CLST_DebugString.
    /// 
    /// Do not pass a string from sound_callback to this function! 
    pub unsafe extern "C" fn CLST_DropDebugString(char: *mut ffi::c_char) {
        let string = ffi::CString::from_raw(char);
        std::mem::drop(string);
    }

    #[no_mangle]
    /// Ticks Madeline's internal state, using the delta-time between the last tick.
    pub extern "C" fn CLST_Tick(&mut self, delta_time: f32) {
        self.input.refresh();

        self.time_active += delta_time;

        // vars
        self.idle_timer += delta_time;

        if self.speed.x != 0. || self.speed.y != 0. {
            self.idle_timer = 0.;
        }

        self.just_respawned &= self.speed == Vector2::ZERO;

        self.on_ground = self.speed.y >= 0. && self.collide(self.position + Vector2::UNIT_Y);

        self.play_footstep_on_land -= delta_time;

        self.highest_air_y = if self.on_ground {
            self.position.y
        } else {
            self.position.y.min(self.highest_air_y)
        };

        self.flash = self.flash ^ self.on_interval(0.05, delta_time);

        if self.wall_slide_dir != 0 {
            self.wall_slide_timer = 0f32.max(self.wall_slide_timer - delta_time);
        }

        if self.wall_boost_timer > 0. {
            self.wall_boost_timer -= delta_time;
            if self.move_x == self.wall_boost_dir {
                self.speed.x = WALL_JUMP_H_SPEED * (self.move_x as f32);
                self.stamina += CLIMB_JUMP_COST;
                self.wall_boost_timer = 0.;
            }
        }

        if self.on_ground && self.state != State::StClimb {
            self.auto_jump = false;
            self.stamina = CLIMB_MAX_STAMINA;
            self.wall_slide_timer = WALL_SLIDE_TIME;
        }

        if self.dash_attack_timer > 0. {
            self.dash_attack_timer -= delta_time;
        }

        if self.on_ground {
            self.jump_grace_timer = JUMP_GRACE_TIME;
        } else if self.jump_grace_timer > 0. {
            self.jump_grace_timer -= delta_time;
        }

        // dashes
        if self.dash_cooldown_timer > 0. {
            self.dash_cooldown_timer -= delta_time;
        }

        if self.dash_refill_cooldown_timer > 0. {
            self.dash_refill_cooldown_timer -= delta_time;
        } else if !self.inventory.no_refills {
            if self.state == State::StSwim || (self.on_ground && self.collide(self.position + Vector2::UNIT_Y)) {
                self.refill_dash();
            }
        }

        if self.var_jump_timer > 0. {
            self.var_jump_timer -= delta_time;
        }

        if self.auto_jump_timer > 0. {
            if self.auto_jump {
                self.auto_jump_timer -= delta_time;
                if self.auto_jump_timer <= 0. {
                    self.auto_jump = false
                }
            } else {
                self.auto_jump_timer = 0.;
            }
        }

        if self.force_move_x_timer > 0. {
            self.force_move_x_timer -= delta_time;
            self.move_x = self.force_move_x;
        } else {
            self.move_x = self.input.get_move_x();
        }

        // facing
        if self.move_x != 0 && self.state != State::StClimb {
            let facing = if self.move_x > 0 { Facings::FacingRight } else { Facings::FacingLeft };
            self.facing = facing;
        }

        self.last_aim = self.input.get_aim_vector(self.facing as i8, true);

        // wall speed retention
        if self.wall_speed_retention_timer > 0. {
            if self.speed.x.is_sign_positive() != self.wall_speed_retained.is_sign_positive() {
                self.wall_speed_retention_timer = 0.
            } else if !self.collide(self.position + Vector2::UNIT_X * self.wall_speed_retained.signum()) {
                self.speed.x = self.wall_speed_retained;
                self.wall_speed_retained = 0.;
            } else {
                self.wall_speed_retention_timer -= delta_time;
            }
        }

        if self.hop_wait_x != 0 {
            if (self.speed.x.is_sign_positive() && self.hop_wait_x < 0) || self.speed.y > 0. {
                self.hop_wait_x = 0;
            } else if
                !self.collide(self.position + Vector2::UNIT_X * self.hop_wait_x as f32) 
            {
                self.speed.x = self.hop_wait_x_speed;
                self.hop_wait_x = 0;
            }
        }

        if self.check_stamina() < CLIMB_TIRED_THRESHOLD {
            self.rumble(RumbleStrength::StrLight, RumbleLength::LenShort);
            self.was_tired = true;
        }

        self.state.update(self, delta_time);

        if !self.on_ground && self.dash_attacking() && self.dash_dir.y == 0. {
            if self.collide(self.position + Vector2::UNIT_Y * DASH_V_FLOOR_SNAP_DIST) {
                self.CLST_MoveVExact(DASH_V_FLOOR_SNAP_DIST as i32, false);
            }
        }

        if self.speed.y > 0. && self.can_unduck() && !self.on_ground {
            self.set_ducking(false);
        }

        // physics
        self.CLST_MoveH(self.speed.x * delta_time, true);
        self.CLST_MoveV(self.speed.y * delta_time, true);

        // swimming
        if self.state == State::StSwim {
            if self.speed.y < 0. && self.speed.y >= SWIM_MAX_RISE {
                while !self.swim_check() {
                    self.speed.y = 0.;
                    if self.CLST_MoveVExact(1, false) {
                        break;
                    }
                }
            }
        } else if matches!(self.state, State::StNormal | State::StClimb) && self.swim_check() {
            self.CLST_SetState(State::StSwim);
        }

        let ducking = self.ducking();
        if ducking != self.was_ducking {
            self.was_ducking = ducking;
            if self.was_ducking {
                self.play_sound(c"char_mad_duck");
            } else {
                self.play_sound(c"char_mad_stand");
            }
        }

        self.was_on_ground = self.on_ground;
    }

    fn refill_dash(&mut self) -> bool {
        if self.dashes < self.max_dashes {
            self.dashes = self.max_dashes;
            true
        } else {
            false
        }
    }

    fn collide_water(&self, position: Vector2) -> bool {
        self.water_callback
            .map_or(
                false,
                |func| func(self, position)
            )
    }

    fn dash_attacking(&self) -> bool {
        self.dash_attack_timer > 0.
    }

    fn is_tired(&self) -> bool {
        self.check_stamina() < CLIMB_TIRED_THRESHOLD
    }

    fn ducking(&self) -> bool {
        self.collider == DUCK_HITBOX || self.hurtbox == DUCK_HURTBOX
    }

    fn friction_mult(&self) -> f32 {
        self.friction_callback
            .map_or(
                1.,
                |func| func(self)
            )
    }
    
    fn collide(&self, position: Vector2) -> bool {
        self.collision_callback
            .map_or(
                false, 
                |func| func(self, position)
            )
    }

    fn slip_check(&self, add_y: f32) -> bool {
        let at = match self.facing {
            Facings::FacingRight => self.collider.top_right() + Vector2::UNIT_Y * (4. + add_y),
            Facings::FacingLeft => self.collider.top_left() - Vector2::UNIT_X + Vector2::UNIT_Y * (4. + add_y)
        };

        !self.collide(at) && !self.collide(at + Vector2::UNIT_Y * (-4. + add_y))
    }

    fn on_interval(&self, interval: f32, delta_time: f32) -> bool {
        (self.time_active / interval).trunc() !=
            ((self.time_active + delta_time) / interval).trunc()
    }

    #[no_mangle]
    // Did a bit of detective work, it seems the Exact movement functions take integers based on this on line 1855:
    // MoveVExact((int)(fromY - Bottom));
    pub extern "C" fn CLST_MoveHExact(&mut self, amount: i32, callback: bool) -> bool {
        // Do a raycast
        let mut move_amount = 0;
        let step = amount.signum();
        while move_amount != amount {
            move_amount += step;

            if self.collide(self.position + Vector2::UNIT_X * move_amount as f32) {
                self.position.x += move_amount as f32;
                if callback {
                    self.on_collide_h();
                }
                return true;
            }
        }
        self.position.x += amount as f32;
        false
    }

    #[no_mangle]
    pub extern "C" fn CLST_MoveVExact(&mut self, amount: i32, callback: bool) -> bool {
        let mut move_amount = 0;
        let step = amount.signum();
        while move_amount != amount {
            move_amount += step;

            if self.collide(self.position + Vector2::UNIT_Y * move_amount as f32) {
                self.position.y += move_amount as f32;
                if callback {
                    self.on_collide_v();
                }
                return true;
            }
        }
        self.position.y += amount as f32;
        false
    }

    #[no_mangle]
    /// Moves Madeline on the X axis.
    pub extern "C" fn CLST_MoveH(&mut self, amount: f32, callback: bool) -> bool {
        self.rem_position.x += amount;
        // Subpixels seem to be stored separately? This is mostly just guesswork
        // That would make sense, looking at the PICO-8 player code
        let rounded_x = self.rem_position.x.round();
        if rounded_x != 0. {
            self.rem_position.x -= rounded_x;
            self.CLST_MoveHExact(amount as i32, callback)
        } else { false }
    }

    #[no_mangle]
    /// Moves Madeline on the Y axis.
    pub extern "C" fn CLST_MoveV(&mut self, amount: f32, callback: bool) -> bool {
        let rounded_y = self.rem_position.y.round();
        if rounded_y != 0. {
            self.rem_position.y -= rounded_y;
            self.CLST_MoveVExact(amount as i32, callback)
        } else { false }
    }

    fn on_collide_h(&mut self) {
        if self.dash_attacking() {
            if let Some(cb) = self.dash_collision_callback {
                let res = cb(self, self.dash_dir);
                match res {
                    DashCollisionResults::ResBounce =>
                        self.reflect_bounce(Vector2::new(
                            -self.speed.x.signum(),
                            0.
                        )),
                    DashCollisionResults::ResRebound =>
                        self.rebound(self.speed.x.signum() as i8),
                    DashCollisionResults::ResIgnore => ()
                }
                return;
            }
        }

        if self.state == State::StDash {
            if self.on_ground && self.can_unduck_at(self.position + Vector2::UNIT_X * self.speed.x.signum()) {
                self.set_ducking(true);
                return;
            } else if self.speed.y == 0. && self.speed.x != 0. {
                for i in 1..=DASH_CORNER_CORRECTION {
                    for j in [1, -1] {
                        if !self.collide(self.position + Vector2::new(self.speed.x.signum(), i as f32 * j as f32)) {
                            self.CLST_MoveVExact(i * j, false);
                            self.CLST_MoveHExact(self.speed.x.signum() as i32, false);
                            return;
                        }
                    }
                }
            }
        }

        if self.wall_speed_retention_timer <= 0. {
            self.wall_speed_retained = self.speed.x;
            self.wall_speed_retention_timer = WALL_SPEED_RETENTION_TIME;
        }

        self.speed.x = 0.;
        self.dash_attack_timer = 0.;
    }

    fn on_collide_v(&mut self) -> bool {
        if self.state == State::StSwim {
            self.speed.y = 0.;
            return true;
        }

        if self.dash_attacking() {
            if let Some(cb) = self.dash_collision_callback {
                let res = cb(self, self.dash_dir);
                match res {
                    DashCollisionResults::ResBounce =>
                        self.reflect_bounce(Vector2::new(
                            0.,
                            -self.speed.y.signum()
                        )),
                    DashCollisionResults::ResRebound =>
                        self.rebound(0),
                    DashCollisionResults::ResIgnore => ()
                }
                return true;
            }
        }

        if self.speed.y > 0. {
            if self.dash_dir.x != 0. && self.dash_dir.y > 0. && self.speed.y > 0. {
                self.dash_dir.x = self.dash_dir.x.signum();
                self.dash_dir.y = 0.;
                self.speed.y = 0.;
                self.speed.x *= DODGE_SLIDE_SPEED_MULT;
                self.set_ducking(true);
            }


            if self.state != State::StClimb {                    
                self.rumble(RumbleStrength::StrLight, RumbleLength::LenShort);
            }
        } else if self.speed.y < 0. {
            if self.speed.x <= 0. {
                for i in 1..UPWARD_CORNER_CORRECTION {
                    if !self.collide(self.position + Vector2::new(-i as f32, -1.)) {
                        self.position += Vector2::new(-i  as f32, -1.);
                        return true;
                    }
                }
            }

            if self.speed.x >= 0. {
                for i in 1..UPWARD_CORNER_CORRECTION {
                    if !self.collide(self.position + Vector2::new(i as f32, -1.)) {
                        self.position += Vector2::new(i  as f32, -1.);
                        return true;
                    }
                }
            }

            if self.var_jump_timer < VAR_JUMP_TIME - CEILING_VAR_JUMP_GRACE {
                self.var_jump_timer = 0.;
            }
        }

        false
    }

    fn check_stamina(&self) -> f32 {
        if self.wall_boost_timer > 0. {
            self.stamina + CLIMB_JUMP_COST
        } else {
            self.stamina
        }
    }

    fn swim_check(&self) -> bool {
        self.collide_water(self.position + Vector2::UNIT_Y * -8.0)
        && self.collide_water(self.position)
    }

    fn swim_jump_check(&self) -> bool {
        self.collide_water(self.position + Vector2::UNIT_Y * -14.0)
    }

    fn swim_rise_check(&self) -> bool {
        self.collide_water(self.position + Vector2::UNIT_Y * -18.0)
    }

    fn swim_underwater_check(&self) -> bool {
        self.collide_water(self.position + Vector2::UNIT_Y * -9.0)
    }

    fn can_climb_at(&self, position: Vector2) -> bool {
        self.can_climb_callback
            .map_or(
                true,
                |func| func(self, position)
            )
    }

    fn can_unduck(&mut self) -> bool {
        if !self.ducking() {
            true
        } else {
            let prev = self.collider;
            self.collider = NORMAL_HITBOX;
            let res = self.collide(self.position);
            self.collider = prev;
            res
        }
    }

    fn can_unduck_at(&mut self, pos: Vector2) -> bool {
        let prev = self.position;
        self.position = pos;
        let res = self.can_unduck();
        self.position = prev;
        res
    }

    fn set_ducking(&mut self, ducking: bool) {
        if ducking {
            self.collider = DUCK_HITBOX;
            self.hurtbox = DUCK_HURTBOX;
        } else {
            self.collider = NORMAL_HITBOX;
            self.hurtbox = NORMAL_HURTBOX;
        }
    }
    
    fn play_sound(&self, name: &'static ffi::CStr) {
        if let Some(cb) = self.sound_callback {
            cb(name.as_ptr())
        }
    }

    fn jump(&mut self, _particles: bool, play_sfx: bool) {
        self.input.jump_consumed = true;
        self.jump_grace_timer = 0.;
        self.var_jump_timer = VAR_JUMP_TIME;
        self.auto_jump = false;
        self.dash_attack_timer = 0.;
        self.wall_slide_timer = WALL_SLIDE_TIME;
        self.wall_boost_timer = 0.;

        self.speed.x += JUMP_H_BOOST * self.move_x as f32;
        self.speed.y = JUMP_SPEED;
        self.speed += self.lift_boost();
        self.var_jump_speed = self.speed.y;

        if play_sfx {
            self.play_sound(c"char_mad_jump");
        }
    }

    fn super_jump(&mut self) {
        self.input.jump_consumed = true;
        self.jump_grace_timer = 0.;
        self.var_jump_timer = VAR_JUMP_TIME;
        self.auto_jump = false;
        self.dash_attack_timer = 0.;
        self.wall_slide_timer = WALL_SLIDE_TIME;
        self.wall_boost_timer = 0.;

        self.speed.x = SUPER_JUMP_H * self.facing as i8 as f32;
        self.speed.y = JUMP_SPEED;
        self.speed += self.lift_boost();

        self.play_sound(c"char_mad_jump");

        if self.ducking() {
            self.set_ducking(false);
            self.speed.x *= DUCK_SUPER_JUMP_X_MULT;
            self.speed.y *= DUCK_SUPER_JUMP_Y_MULT;
        }

        self.var_jump_speed = self.speed.y;
    }

    fn wall_jump_check(&self, dir: i8) -> bool {
        self.collide(self.position + Vector2::UNIT_X * dir as f32 * WALL_JUMP_CHECK_DIST)
    }

    fn wall_jump(&mut self, dir: i8) {
        self.set_ducking(false);
        self.input.jump_consumed = true;
        self.jump_grace_timer = 0.;
        self.var_jump_timer = VAR_JUMP_TIME;
        self.auto_jump = false;
        self.dash_attack_timer = 0.;
        self.wall_slide_timer = WALL_SLIDE_TIME;
        self.wall_boost_timer = 0.;
        if self.move_x != 0 {
            self.force_move_x = dir;
            self.force_move_x_timer = WALL_JUMP_FORCE_TIME;
        }

        self.speed.x = WALL_JUMP_H_SPEED * dir as f32;
        self.speed.y = JUMP_SPEED;
        self.speed += self.lift_boost();
        self.var_jump_speed = self.speed.y;

        self.play_sound(c"char_mad_jump_wall")
    }

    fn super_wall_jump(&mut self, dir: i8) {
        self.set_ducking(false);
        self.input.jump_consumed = true;
        self.jump_grace_timer = 0.;
        self.var_jump_timer = SUPER_WALL_JUMP_VAR_TIME;
        self.auto_jump = false;
        self.dash_attack_timer = 0.;
        self.wall_slide_timer = WALL_SLIDE_TIME;
        self.wall_boost_timer = 0.;

        
        self.speed.x = SUPER_WALL_JUMP_H * dir as f32;
        self.speed.y = SUPER_WALL_JUMP_SPEED;
        self.speed += self.lift_boost();
        self.var_jump_speed = self.speed.y;

        self.play_sound(c"char_mad_jump_superwall")
    }

    fn climb_hop(&mut self) {
        // fuck you i'm not implementing climb hopping
        if !self.on_ground {
            self.stamina += CLIMB_JUMP_COST;
        }
        self.climb_jump();
    }

    fn climb_jump(&mut self) {
        if !self.on_ground {
            self.stamina -= CLIMB_JUMP_COST;
            self.rumble(RumbleStrength::StrLight, RumbleLength::LenMedium);
        }

        self.jump(false, false);

        if self.move_x == 0 {
            self.wall_boost_dir = -(self.facing as i8);
            self.wall_boost_timer = CLIMB_JUMP_BOOST_TIME;
        }

        self.play_sound(c"char_mad_jump_climc")
    }

    fn rebound(&mut self, dir: i8) {
        self.speed.x = REBOUND_SPEED_X * dir as f32;
        self.speed.y = REBOUND_SPEED_Y;
        
        self.var_jump_speed = self.speed.y;
        self.var_jump_timer = REBOUND_VAR_JUMP_TIME;
        self.auto_jump = true;
        self.auto_jump_timer = 0.;
        self.dash_attack_timer = 0.;
        self.wall_slide_timer = WALL_SLIDE_TIME;
        self.wall_boost_timer = 0.;
        self.force_move_x_timer = 0.;

        self.CLST_SetState(State::StNormal);
    }

    fn reflect_bounce(&mut self, dir: Vector2) {
        if dir.x != 0. {
            self.speed.x = dir.x * REFLECT_BOUND_SPEED;
        }
        if dir.y != 0. {
            self.speed.y = dir.y * REFLECT_BOUND_SPEED;
        }

        self.auto_jump_timer = 0.;
        self.dash_attack_timer = 0.;
        self.wall_slide_timer = WALL_SLIDE_TIME;
        self.wall_boost_timer = 0.;
        self.dash_attack_timer = 0.;
        self.force_move_x_timer = 0.;

        self.CLST_SetState(State::StNormal);
    }
    
    fn lift_boost(&self) -> Vector2 {
        let mut val = self.lift_speed;
        if val.x.abs() > LIFT_X_CAP {
            val.x = LIFT_X_CAP.copysign(val.x);
        }

        if val.y > 0. {
            val.y = 0.;
        } else if val.y < LIFT_Y_CAP {
            val.y = LIFT_Y_CAP;
        }

        val
    }
    
    fn rumble(&self, strength: RumbleStrength, length: RumbleLength) {
        if let Some(cb) = self.rumble_callback {
            cb(strength, length);
        }
    }

    #[no_mangle]
    /// Sets the state of Madeline's state machine,
    /// respecting the state's methods for beginning and ending.
    /// 
    /// You should probably always use this instead of setting state directly.
    pub extern "C" fn CLST_SetState(&mut self, state: State) {
        if self.state == state { return }

        self.state.end(self);

        self.state = state;

        self.state.begin(self);
    }

    fn climb_check(&self, add_y: f32) -> bool {
        self.can_climb_at(
            Vector2::new(self.facing as i8 as f32 * CLIMB_CHECK_DIST, add_y)
        ) && self.collide(
            self.position + Vector2::new(
                self.facing as i8 as f32 * CLIMB_CHECK_DIST,
                add_y
            )
        )
    }

    fn can_dash(&self) -> bool {
        self.input.dashing() && self.dash_cooldown_timer <= 0. && self.dashes > 0
    }


    fn advance_dash(&mut self) -> Option<f32> {
        // This is manually implemented. ;-;
        use DashCoroutineBreakpoint as Brk;

        let res = match self.dash_coroutine_breakpoint {
            Brk::BreakpointStart => {
                self.dash_coroutine_breakpoint = Brk::Breakpoint1;
                None
            },
            Brk::Breakpoint1 => {
                let dir = self.last_aim;

                let mut new_speed = dir * DASH_SPEED;
                if self.before_dash_speed.x.signum() == new_speed.x.signum()
                    && self.before_dash_speed.x.abs() > new_speed.x.abs()
                {
                    new_speed.x = self.before_dash_speed.x;
                }

                self.speed = new_speed;

                if self.collide_water(self.position) {
                    self.speed *= SWIM_DASH_SPEED_MULT;
                }

                self.dash_dir = dir;
                self.facing = match self.dash_dir.x.partial_cmp(&0.) {
                    Some(Ordering::Greater) => Facings::FacingRight,
                    Some(Ordering::Less) => Facings::FacingLeft,
                    _ => self.facing
                };

                if self.on_ground
                    && self.dash_dir.x != 0.
                    && self.dash_dir.y > 0.
                    && self.speed.y > 0.
                {
                    self.dash_dir.x = self.dash_dir.x.signum();
                    self.dash_dir.y = 0.;
                    self.speed.y = 0.;
                    self.speed.x *= DODGE_SLIDE_SPEED_MULT;
                    self.set_ducking(true);
                }

                self.dash_coroutine_breakpoint = Brk::Breakpoint2;
                Some(DASH_TIME)
            },
            Brk::Breakpoint2 => {
                self.auto_jump = true;
                self.auto_jump_timer = 0.;
                if self.dash_dir.y <= 0. {
                    self.speed = self.dash_dir * END_DASH_SPEED;
                }
                if self.speed.y < 0. {
                    self.speed.y *= END_DASH_UP_MULT;
                }
                self.CLST_SetState(State::StNormal);
                None
            }
        };

        res
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[repr(C)]
pub enum State {
    #[default]
    StNormal,
    StClimb,
    StDash,
    StSwim,
}

impl State {
    fn begin(self, maddy: &mut Madeline) {
        match self {
            State::StNormal => 
                maddy.max_fall = MAX_FALL,
            State::StClimb => {
                maddy.auto_jump = false;
                maddy.speed.x = 0.;
                maddy.speed.y *= CLIMB_GRAB_Y_MULT;
                maddy.wall_slide_timer = WALL_SLIDE_TIME;
                maddy.climb_no_move_timer = CLIMB_NO_MOVE_TIME;
                maddy.wall_boost_timer = 0.;
                maddy.last_climb_move = 0;

                maddy.rumble(RumbleStrength::StrMedium, RumbleLength::LenShort);

                for _ in 0..(CLIMB_CHECK_DIST as i32) {
                    if !maddy.collide(maddy.position + Vector2::UNIT_X * maddy.facing as i8 as f32) {
                        maddy.position.x += maddy.facing as i8 as f32;
                    } else {
                        break;
                    }
                }
            },
            State::StDash => {
                maddy.dash_started_on_ground = maddy.on_ground;
                maddy.dash_cooldown_timer = DASH_COOLDOWN;
                maddy.dash_refill_cooldown_timer = DASH_REFILL_COOLDOWN;
                maddy.started_dashing = true;
                maddy.wall_slide_timer = WALL_SLIDE_TIME;
                maddy.dash_coroutine_breakpoint = DashCoroutineBreakpoint::BreakpointStart;
                
                maddy.rumble(RumbleStrength::StrStrong, RumbleLength::LenMedium);

                maddy.dash_attack_timer = DASH_ATTACK_TIME;
                maddy.before_dash_speed = maddy.speed;
                maddy.speed = Vector2::ZERO;
                maddy.dash_dir = Vector2::ZERO;

                if !maddy.on_ground && maddy.ducking() && maddy.can_unduck() {
                    maddy.set_ducking(false);
                }
            }
            State::StSwim => {
                if maddy.speed.y > 0. {
                    maddy.speed.y *= SWIM_Y_SPEED_MULT;
                }
                maddy.stamina = CLIMB_MAX_STAMINA;
            }
        }
    }

    fn update(self, maddy: &mut Madeline, delta_time: f32) {
        let new_state = match self {
            State::StNormal => update_normal(maddy, delta_time),
            State::StClimb => update_climb(maddy, delta_time),
            State::StDash => update_dash(maddy, delta_time),
            State::StSwim => update_swim(maddy, delta_time)
        };

        maddy.CLST_SetState(new_state);
    }

    fn end(self, maddy: &mut Madeline) {
        match self {
            State::StNormal => {
                maddy.wall_boost_timer = 0.;
                maddy.wall_speed_retention_timer = 0.;
                maddy.hop_wait_x = 0;
            },
            State::StClimb =>
                maddy.wall_speed_retention_timer = 0.,
            State::StDash => {
                maddy.dash_coroutine_breakpoint = DashCoroutineBreakpoint::BreakpointStart;
            }
            State::StSwim => {}
        }
    }
}

fn update_normal(maddy: &mut Madeline, delta_time: f32) -> State {
        
    if maddy.lift_boost().y < 0. && maddy.was_on_ground && !maddy.on_ground && maddy.speed.y >= 0. {
        maddy.speed.y = maddy.lift_boost().y;
    }

    if maddy.input.grabbing() && !maddy.is_tired() && !maddy.ducking()
        && maddy.speed.y >= 0. && maddy.speed.x.signum() as i8 != -(maddy.facing as i8)
    {
        if maddy.climb_check(0.) {
            maddy.set_ducking(false);
            return State::StClimb;
        }

        if maddy.input.aim.y < maddy.input.deadzone.y {
            for i in 1..=CLIMB_UP_CHECK_DIST {
                if !maddy.collide(maddy.position + Vector2::UNIT_Y * -i as f32)
                    && maddy.climb_check(-i as f32)
                {
                    maddy.CLST_MoveVExact(-i, false);
                    maddy.set_ducking(false);
                    return State::StClimb;
                }
            }
        }
    }

    if maddy.can_dash() {
        maddy.speed += maddy.lift_boost();
        maddy.dashes = maddy.dashes.saturating_sub(1);
        maddy.input.dash_consumed = true;
        return State::StDash;
    }

    if maddy.ducking() {
        if maddy.on_ground && maddy.input.aim.y < maddy.input.deadzone.y {
            if maddy.can_unduck() {
                maddy.set_ducking(false);
            } else if maddy.speed.x == 0. {
                for i in (1..=DUCK_CORRECT_CHECK).rev() {
                    if maddy.can_unduck_at(maddy.position + Vector2::UNIT_X * i as f32) {
                        maddy.CLST_MoveH(DUCK_CORRECT_SLIDE, false);
                        break;
                    } else if maddy.can_unduck_at(maddy.position - Vector2::UNIT_X * i as f32) {
                        maddy.CLST_MoveH(-DUCK_CORRECT_SLIDE, false);
                        break;
                    }
                }
            }
        }
    } else if maddy.on_ground && maddy.input.aim.y >= maddy.input.deadzone.y && maddy.speed.y >= 0. {
        maddy.set_ducking(true);
    }

    if maddy.ducking() && maddy.on_ground {
        maddy.speed.x = maddy.speed.x.approach(0., DUCK_FRICTION * delta_time);
    } else {
        let mut friction_mult = if maddy.on_ground { 1. } else { AIR_MULT };
        friction_mult *= maddy.friction_mult();

        if maddy.speed.x.abs() > MAX_RUN && maddy.speed.x.signum() == maddy.move_x as f32 {
            maddy.speed.x = maddy.speed.x.approach(MAX_RUN * maddy.move_x as f32, RUN_REDUCE * friction_mult * delta_time)
        } else {
            maddy.speed.x = maddy.speed.x.approach(MAX_RUN * maddy.move_x as f32, RUN_ACCEL * friction_mult * delta_time)
        }
    }

    if maddy.input.aim.y >= maddy.input.deadzone.y && maddy.speed.y >= MAX_FALL {
        maddy.max_fall = maddy.max_fall.approach(FAST_MAX_FALL, FAST_MAX_ACCEL * delta_time);
    } else {
        maddy.max_fall = maddy.max_fall.approach(MAX_FALL, FAST_MAX_ACCEL);
    }

    if !maddy.on_ground {
        let mut max_fall = maddy.max_fall;

        if (
            maddy.move_x == maddy.facing as i8 ||
            (maddy.move_x == 0 && maddy.input.grabbing())
        ) && maddy.input.aim.y <= maddy.input.deadzone.y {
            if
                maddy.speed.y >= 0. && maddy.wall_slide_timer > 0.
                && maddy.collide(maddy.position + Vector2::UNIT_X * maddy.facing as i8 as f32)
                && maddy.can_unduck()
            {
                maddy.set_ducking(false);
                maddy.wall_slide_dir = maddy.facing as i8;
            }

            if maddy.wall_slide_dir != 0 {
                if maddy.wall_slide_timer > WALL_SLIDE_TIME * 0.5 && maddy.can_climb_at(maddy.position + Vector2::UNIT_X * maddy.wall_slide_dir as f32) {
                    maddy.wall_slide_timer = WALL_SLIDE_TIME * 0.5;
                }

                max_fall = MAX_FALL.lerp(WALL_SLIDE_START_MAX, maddy.wall_slide_timer / WALL_SLIDE_TIME);
            }
        }

        let mult = 
            if maddy.speed.y.abs() < HALF_GRAV_THRESHOLD && (maddy.input.jump || maddy.auto_jump)
                { 0.5 } else { 1.0 }
            * maddy.inventory.gravity_mult;

        maddy.speed.y = maddy.speed.y.approach(max_fall, GRAVITY * mult * delta_time);
    }

    if maddy.var_jump_timer > 0. {
        if maddy.auto_jump || maddy.input.jumping() {
            maddy.speed.y = maddy.speed.y
                .min(maddy.var_jump_speed);
        } else {
            maddy.var_jump_timer = 0.
        }
    }

    if maddy.input.jumping() {
        if maddy.jump_grace_timer > 0. {
            maddy.jump(true, true)
        } else if maddy.can_unduck() {
            if maddy.wall_jump_check(1) {
                if maddy.facing == Facings::FacingRight
                    && maddy.input.grabbing()
                    && maddy.stamina > 0.
                    && maddy.can_climb_at(
                        maddy.position + Vector2::UNIT_X * WALL_JUMP_CHECK_DIST
                    )
                {
                    maddy.climb_jump();
                } else if maddy.dash_attacking()
                    && maddy.dash_dir.x == 0.
                    && maddy.dash_dir.y == -1.
                {
                    maddy.super_wall_jump(-1);
                } else {
                    maddy.wall_jump(-1);
                }
            } else if maddy.wall_jump_check(-1) {
                if maddy.facing == Facings::FacingLeft
                    && maddy.input.grabbing()
                    && maddy.stamina > 0.
                    && maddy.can_climb_at(
                        maddy.position - Vector2::UNIT_X * WALL_JUMP_CHECK_DIST
                    )
                {
                    maddy.climb_jump();
                } else if maddy.dash_attacking()
                    && maddy.dash_dir.x == 0.
                    && maddy.dash_dir.y == -1.
                {
                    maddy.super_wall_jump(1);
                } else {
                    maddy.wall_jump(1);
                }
            } else if maddy.collide_water(maddy.position + Vector2::UNIT_Y * 2.) {
                maddy.jump(true, true);
            }
        }
    }

    State::StNormal
}

fn update_climb(maddy: &mut Madeline, delta_time: f32) -> State {
    maddy.climb_no_move_timer -= delta_time;

    if maddy.on_ground {
        maddy.stamina = CLIMB_MAX_STAMINA;
    }

    if maddy.input.jumping() && (!maddy.ducking() || maddy.can_unduck()) {
        if maddy.move_x == -(maddy.facing as i8) {
            maddy.wall_jump(-(maddy.facing as i8))
        } else {
            maddy.climb_jump()
        }

        return State::StNormal;
    }

    if maddy.can_dash() {
        maddy.speed += maddy.lift_boost();
        maddy.dashes = maddy.dashes.saturating_sub(1);
        maddy.input.dash_consumed = true;
        return State::StDash;
    }

    if !maddy.input.grabbing() {
        maddy.speed += maddy.lift_boost();
        return State::StNormal;
    }

    if !maddy.collide(maddy.position + Vector2::UNIT_X * maddy.facing as i8 as f32) {
        if maddy.speed.y < 0. {
            maddy.climb_hop();
        }

        return State::StNormal;
    }

    let mut target = 0.;
    let mut try_slip = false;
    
    if maddy.climb_no_move_timer <= 0. {
        if !maddy.can_climb_at(maddy.position + Vector2::UNIT_X * maddy.facing as i8 as f32) {
            try_slip = true;
        } else if maddy.input.aim.y <= -maddy.input.deadzone.y {
            target = CLIMB_UP_SPEED;

            if maddy.collide(maddy.position - Vector2::UNIT_Y)
                || maddy.slip_check(-1.)
            {
                maddy.speed.y = maddy.speed.y.max(0.);
                target = 0.;
                try_slip = true;
            } else if maddy.slip_check(0.) {
                maddy.climb_hop();
                return State::StNormal;
            }
        } else if maddy.input.aim.y >= maddy.input.deadzone.y {
            target = CLIMB_DOWN_SPEED;

            if maddy.on_ground {
                maddy.speed.y = maddy.speed.y.min(0.);
                target = 0.;
            }
        } else {
            try_slip = true;
        }
    } else {
        try_slip = true;
    }

    maddy.last_climb_move = target.signum() as i8;

    if try_slip && maddy.slip_check(0.) {
        target = CLIMB_SLIP_SPEED;
    }

    maddy.speed.y = maddy.speed.y.approach(target, CLIMB_ACCEL * delta_time);

    if 
        maddy.input.aim.y < maddy.input.deadzone.y
        && maddy.speed.y > 0.
        && !maddy.collide(maddy.position + Vector2::new(maddy.facing as i8 as f32, 1.))
    {
        maddy.speed.y = 0.;
    }

    if maddy.climb_no_move_timer <= 0. {
        if maddy.last_climb_move == -1 {
            maddy.stamina -= CLIMB_UP_COST * delta_time;

            if maddy.on_interval(0.2, delta_time) {
                maddy.rumble(RumbleStrength::StrLight, RumbleLength::LenShort)
            }
        } else {
            if maddy.last_climb_move == 0 {
                maddy.stamina -= CLIMB_STILL_COST * delta_time;
            }

            if !maddy.on_ground && maddy.on_interval(0.8, delta_time) {
                maddy.rumble(RumbleStrength::StrLight, RumbleLength::LenShort);
            }
        }
    }

    if maddy.stamina <= 0. {
        maddy.speed += maddy.lift_boost();
        return State::StNormal;
    }

    State::StClimb
}

fn update_dash(maddy: &mut Madeline, delta_time: f32) -> State {
    // Coroutine stuff

    if maddy.coroutine_timer > 0. {
        maddy.coroutine_timer -= delta_time;
    } else {
        if let Some(res) = maddy.advance_dash() {
            maddy.coroutine_timer = res;
        }
        if maddy.state != State::StDash {
            return maddy.state;
        }
    }

    // Actual update function
    maddy.started_dashing = false;

    // Superjumping
    if maddy.dash_dir.y == 0.
        && maddy.can_unduck()
        && maddy.input.jumping()
        && maddy.jump_grace_timer > 0.
    {
        maddy.super_jump();
        return State::StNormal;
    }

    // Wouncing :3
    if maddy.dash_dir.x == 0. && maddy.dash_dir.y == -1. {
        if maddy.input.jumping() && maddy.can_unduck() {
            if maddy.wall_jump_check(1) {
                maddy.super_wall_jump(-1);
                return State::StNormal;
            } else if maddy.wall_jump_check(-1) {
                maddy.super_wall_jump(1);
                return State::StNormal;
            }
        }
    } else {
        if maddy.input.jumping() && maddy.can_unduck() {
            if maddy.wall_jump_check(1) {
                maddy.wall_jump(-1);
                return State::StNormal;
            } else if maddy.wall_jump_check(-1) {
                maddy.wall_jump(1);
                return State::StNormal;
            }
        }
    }
    
    State::StDash
}

fn update_swim(maddy: &mut Madeline, delta_time: f32) -> State {
    if !maddy.swim_check() {
        return State::StNormal;
    }

    if maddy.can_unduck() {
        maddy.set_ducking(false);
    }

    if maddy.can_dash() {
        maddy.input.dash_consumed = true;
        return State::StDash;
    }

    let underwater = maddy.swim_underwater_check();

    if !underwater && maddy.speed.y >= 0. && maddy.input.grabbing()
        && !maddy.is_tired() && maddy.can_unduck()
        && maddy.speed.x.signum() != maddy.facing as i8 as f32
        && maddy.climb_check(0.)
        && !maddy.CLST_MoveVExact(-1, false)
    {
        maddy.set_ducking(false);
        return State::StClimb;
    }

    let move_vec = maddy.input.get_aim_vector(maddy.facing as i8, false);

    let max_x = if underwater { SWIM_UNDERWATER_MAX } else { SWIM_MAX };
    let max_y = SWIM_MAX;

    if maddy.speed.x > SWIM_MAX && maddy.speed.x.signum() == move_vec.x.signum() {
        maddy.speed.x = maddy.speed.x.approach(max_x * move_vec.x, SWIM_REDUCE * delta_time);
    } else {
        maddy.speed.x = maddy.speed.x.approach(max_x * move_vec.x, SWIM_ACCEL * delta_time);
    }

    if move_vec.y == 0. && maddy.swim_rise_check() {
        maddy.speed.y = maddy.speed.y.approach(SWIM_MAX_RISE, SWIM_ACCEL * delta_time);
    } else if move_vec.y >= 0. && maddy.swim_underwater_check() {
        if maddy.speed.y.abs() > SWIM_MAX && maddy.speed.y.signum() == move_vec.y.signum() {
            maddy.speed.y = maddy.speed.y.approach(max_y * move_vec.y, SWIM_REDUCE * delta_time);
        } else {
            maddy.speed.y = maddy.speed.y.approach(max_y * move_vec.y, SWIM_ACCEL * delta_time);
        }
    }

    if !underwater && maddy.move_x != 0
        && maddy.collide(maddy.position + Vector2::UNIT_X * maddy.move_x as f32)
        && maddy.collide(maddy.position + Vector2::new(maddy.move_x as f32, -3.))
    {
        maddy.climb_hop();
    }

    if maddy.input.jumping() && maddy.swim_jump_check() {
        maddy.jump(false, false);
        return State::StNormal;
    }

    State::StSwim
}