// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn new(level: u32) -> Self {
        Self {
            health: 100,
            level,
            mana: if level >= 10 { Some(100) } else { None },
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(m) if m >= mana_cost => {
                self.mana = Some(m - mana_cost);
                mana_cost * 2
            }
            None => {
                self.health -= std::cmp::min(self.health, mana_cost);
                0
            }
            _ => 0,
        }
    }
}
