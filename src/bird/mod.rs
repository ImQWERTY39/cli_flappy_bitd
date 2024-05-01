pub const BIRD_WIDTH: u16 = 6;
pub const BIRD_HEIGHT: u16 = 2;
pub const JUMP_STRENGTH: u16 = 4;
pub const GRAVITY: u16 = 1;

pub struct Bird {
    pub x: u16,
    pub y: u16,
    pub y_vel: u16,
}

impl Bird {
    pub fn update_height(&mut self, window_height: u16) -> Result<(), BirdTouchedGround> {
        if self.y + GRAVITY > window_height - BIRD_HEIGHT {
            return Err(BirdTouchedGround);
        }

        self.y = (self.y + GRAVITY)
            .checked_sub(self.y_vel)
            .unwrap_or_default();
        self.y_vel = self.y_vel.checked_sub(1).unwrap_or_default();

        Ok(())
    }
}

pub struct BirdTouchedGround;
