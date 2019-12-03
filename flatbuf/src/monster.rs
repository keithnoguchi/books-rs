// SPDX-License-Identifier: GPL-2.0
use crate::model::my_game::sample;
use crate::model::my_game::sample::{Color, Equipment, MonsterArgs, Vec3, Weapon, WeaponArgs};
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct Monster;

impl Monster {
    #[allow(dead_code)]
    fn create<'b>(b: &mut FlatBufferBuilder<'b>, name: &str) -> WIPOffset<sample::Monster<'b>> {
        let name1 = b.create_string("Axe");
        let name2 = b.create_string("Sword");
        println!("axe name: {:?}", name1);
        let axe = Weapon::create(
            b,
            &WeaponArgs {
                name: Some(name1),
                damage: 5,
            },
        );
        println!("axe: {:?}", axe);
        println!("sword name: {:?}", name2);
        let sword = Weapon::create(
            b,
            &WeaponArgs {
                name: Some(name2),
                damage: 3,
            },
        );
        println!("sword: {:?}", sword);
        let weapons = b.create_vector(&[axe, sword]);
        println!("weapons: {:?}", weapons);
        let name = b.create_string(name);
        println!("name: {:?}", name);
        let inventory = b.create_vector(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        println!("inventory: {:?}", inventory);
        let x = Vec3::new(1.0, 2.0, 3.0);
        println!("x: {:?}", x);
        let y = Vec3::new(4.0, 5.0, 6.0);
        println!("x: {:?}", y);
        let path = b.create_vector(&[x, y]);
        println!("path: {:?}", path);
        let orc = sample::Monster::create(
            b,
            &MonsterArgs {
                pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
                //mana: 150, // It's a default value which is filled in below
                hp: 80,
                name: Some(name),
                inventory: Some(inventory),
                color: Color::Red,
                weapons: Some(weapons),
                equipped_type: Equipment::Weapon,
                equipped: Some(axe.as_union_value()),
                path: Some(path),
                ..Default::default()
            },
        );
        println!("monster: {:?}", orc);
        orc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builder_with_different_capacities() {
        let capacities = [1usize, 16, 32, 64, 128, 256, 1024, 2048, 4096];
        for &t in &capacities {
            let _builder = FlatBufferBuilder::new_with_capacity(t);
        }
    }
    #[test]
    fn serialize_sword_and_axe() {
        let mut b = FlatBufferBuilder::new();
        let name = b.create_string("Sword");
        let _sword = Weapon::create(
            &mut b,
            &WeaponArgs {
                name: Some(name),
                damage: 3,
            },
        );
        let name = b.create_string("Axe");
        let _axe = Weapon::create(
            &mut b,
            &WeaponArgs {
                name: Some(name),
                damage: 5,
            },
        );
    }
    #[test]
    fn serialize_weapons() {
        let mut b = FlatBufferBuilder::new_with_capacity(1);
        let name = b.create_string("Sword");
        let sword = Weapon::create(
            &mut b,
            &WeaponArgs {
                name: Some(name),
                damage: 3,
            },
        );
        let name = b.create_string("Axe");
        let axe = Weapon::create(
            &mut b,
            &WeaponArgs {
                name: Some(name),
                damage: 5,
            },
        );
        let _weapons = b.create_vector(&[sword, axe]);
    }
    #[test]
    fn serialize_monster() {
        let mut builder = FlatBufferBuilder::new_with_capacity(1);
        let orc = super::Monster::create(&mut builder, "ore");
        builder.finish(orc, None);
    }
    #[test]
    fn serialize_and_deserialize_monster() {
        use super::sample::get_root_as_monster;
        let mut builder = FlatBufferBuilder::new();
        let godzilla = super::Monster::create(&mut builder, "godzilla");
        builder.finish(godzilla, None);
        let buf = builder.finished_data(); // Of type `&[u8]`
        let monster = get_root_as_monster(buf);
        match monster.name() {
            Some(got) => assert_eq!("godzilla", got),
            _ => panic!("unexpected None"),
        };
    }
    #[test]
    fn multiple_monsters() {
        use super::sample::get_root_as_monster;
        let monsters = ["godzilla", "minilla", "ore"];
        for name in &monsters {
            let mut builder = FlatBufferBuilder::new();
            let monster = super::Monster::create(&mut builder, name);
            builder.finish(monster, None);
            let buf = builder.finished_data();
            let monster = get_root_as_monster(buf);
            match monster.name() {
                Some(got) => assert_eq!(name, &got),
                _ => panic!("unexpected None"),
            }
        }
    }
}
