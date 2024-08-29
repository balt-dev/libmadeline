use crate::Vector2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum RumbleStrength {
    #[default]
    Light,
    Medium,
    Strong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum RumbleLength {
    #[default]
    Short,
    Medium,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Input {
    /// The aim direction of the movement.
    pub aim: Vector2,
    /// The leniency per axis for having the stick point in an exact direction.
    pub deadzone: Vector2,
    pub jump: bool,
    pub grab: bool,
    pub dash: bool,
    pub talk: bool,
    pub jump_consumed: bool,
    pub grab_consumed: bool,
    pub dash_consumed: bool,
    pub talk_consumed: bool,
}

impl Input {
    pub fn refresh(&mut self) {
        self.jump_consumed &= self.jump;
        self.grab_consumed &= self.grab;
        self.dash_consumed &= self.dash;
        self.talk_consumed &= self.talk;
    }

    pub fn jumping(&self) -> bool {
        self.jump && !self.jump_consumed
    }

    pub fn dashing(&self) -> bool {
        self.dash && !self.dash_consumed
    }

    pub fn grabbing(&self) -> bool {
        self.grab && !self.grab_consumed
    }

    pub fn talking(&self) -> bool {
        self.talk && !self.talk_consumed
    }

    pub fn get_aim_vector(&self, facing: i8, snap: bool) -> Vector2 {
        // This is heuristic and based entirely on my experience in the game :P
        if self.aim == Vector2::ZERO {
            return Vector2::new(facing as f32, 0.);
        }
        Vector2::new(
            if self.aim.x.abs() < self.deadzone.x {
                if snap {
                    self.aim.x.signum()
                } else {
                    self.aim.x
                }
            } else {
                0.
            },
            if self.aim.y.abs() < self.deadzone.y {
                if snap {
                    self.aim.y.signum()
                } else {
                    self.aim.y
                }
            } else {
                0.
            },
        )
        .normalized()
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            aim: Vector2::ZERO,
            // Just a guess
            deadzone: Vector2::new(0.15, 0.15),
            jump: false,
            grab: false,
            dash: false,
            talk: false,
            jump_consumed: false,
            grab_consumed: false,
            dash_consumed: false,
            talk_consumed: false,
        }
    }
}
