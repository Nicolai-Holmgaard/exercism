// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            Some(Self{
                health: 100,
                mana: if self.level < 10 { None } else { Some(100) },
                level: self.level
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana < mana_cost {
                    return 0;
                }
                self.mana = Some(mana - mana_cost);
                2 * mana_cost
            },
            None => {
                self.health = self.health - if self.health < mana_cost { self.health } else { mana_cost };
                0
            }
        }
    }
}
