pub mod member;
pub mod boss;

use std::collections::{HashMap, HashSet};

use member::*;
use boss::*;

pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(name.to_string(), Member {role: Role::Associate, age});
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let self_power: u32 = self.members.values().map(|m| m.role.power()).sum();
        let other_power: u32 = other.members.values().map(|m| m.role.power()).sum();

        let self_loses = self_power <= other_power;
        let (loser, winner) = if self_loses { (self, other) } else { (other, self) };

        if let Some(&youngest_age) = loser.members.values().map(|m| m.age).collect::<Vec<_>>().iter().min() {
            loser.members.retain(|_, member| member.age != youngest_age);
        }

        if loser.members.is_empty() {
            winner.cities.extend(loser.cities.drain());
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, value: u64) {
        let amount = value.min(target.wealth);
        self.wealth += amount;
        target.wealth -= amount;
    }

    pub fn conquer_city(&mut self, mobs: &[&Mob], city: String) {
        for mob in mobs {
            if mob.cities.contains(&city) {
                break;
            } else {
                self.cities.insert(city.clone())
            };
        }
    }
}