pub fn run() {
    let boss = Fighter { health: 104, damage: 8, armor: 1 };

    let mut weapons: Weapons = Vec::new();
    weapons.push(Item { name: String::from("Dagger"), stats: ItemStats { cost: 8, damage: 4, armor: 0 }});
    weapons.push(Item { name: String::from("ShortSword"), stats: ItemStats { cost: 10, damage: 5, armor: 0 }});
    weapons.push(Item { name: String::from("Warhammer"), stats: ItemStats { cost: 25, damage: 6, armor: 0 }});
    weapons.push(Item { name: String::from("Longsword"), stats: ItemStats { cost: 40, damage: 7, armor: 0 }});
    weapons.push(Item { name: String::from("Greataxe"), stats: ItemStats { cost: 74, damage: 8, armor: 0 }});

    let mut armor: Armor = Vec::new();
    armor.push(Item { name: String::from("None"), stats: ItemStats { cost: 0, damage: 0, armor: 0 }});
    armor.push(Item { name: String::from("Leather"), stats: ItemStats { cost: 13, damage: 0, armor: 1 }});
    armor.push(Item { name: String::from("Chainmail"), stats: ItemStats { cost: 31, damage: 0, armor: 2 }});
    armor.push(Item { name: String::from("Splintmail"), stats: ItemStats { cost: 53, damage: 0, armor: 3 }});
    armor.push(Item { name: String::from("Bandedmail"), stats: ItemStats { cost: 75, damage: 0, armor: 4 }});
    armor.push(Item { name: String::from("Platemail"), stats: ItemStats { cost: 102, damage: 0, armor: 5 }});

    let mut rings: Rings = Vec::new();
    rings.push(Item { name: String::from("None 1"), stats: ItemStats { cost: 0, damage: 0, armor: 0 }});
    rings.push(Item { name: String::from("None 2"), stats: ItemStats { cost: 0, damage: 0, armor: 0 }});
    rings.push(Item { name: String::from("Damage +1"), stats: ItemStats { cost: 25, damage: 1, armor: 0 }});
    rings.push(Item { name: String::from("Damage +2"), stats: ItemStats { cost: 50, damage: 2, armor: 0 }});
    rings.push(Item { name: String::from("Damage +3"), stats: ItemStats { cost: 100, damage: 3, armor: 0 }});
    rings.push(Item { name: String::from("Defense +1"), stats: ItemStats { cost: 20, damage: 0, armor: 1 }});
    rings.push(Item { name: String::from("Defense +2"), stats: ItemStats { cost: 40, damage: 0, armor: 2 }});
    rings.push(Item { name: String::from("Defense +3"), stats: ItemStats { cost: 80, damage: 0, armor: 3 }});

    let shop = Shop { weapons, armor, rings };

    p1(&boss, &shop);
    p2(&boss, &shop);
}

fn p1(boss: &Fighter, shop: &Shop) {
    let health = 100;
    let mut lowest_cost = i32::MAX;

    for weapon in &shop.weapons {
        for armor in &shop.armor {
            for ring1 in &shop.rings {
                for ring2 in &shop.rings {
                    if ring1.name == ring2.name {
                        continue;
                    }

                    let p_damage = weapon.stats.damage + armor.stats.damage + ring1.stats.damage + ring2.stats.damage;
                    let p_armor = weapon.stats.armor + armor.stats.armor + ring1.stats.armor + ring2.stats.armor;
                    let player = Fighter { health, damage: p_damage, armor: p_armor };

                    if will_player_win(boss, &player) {
                        let cost = weapon.stats.cost + armor.stats.cost + ring1.stats.cost + ring2.stats.cost;
                        if cost < lowest_cost {
                            lowest_cost = cost;
                        }
                    }
                }
            }
        }
    }

    println!("{lowest_cost}");
}

fn p2(boss: &Fighter, shop: &Shop) {
    let health = 100;
    let mut highest_cost = i32::MIN;

    for weapon in &shop.weapons {
        for armor in &shop.armor {
            for ring1 in &shop.rings {
                for ring2 in &shop.rings {
                    if ring1.name == ring2.name {
                        continue;
                    }

                    let p_damage = weapon.stats.damage + armor.stats.damage + ring1.stats.damage + ring2.stats.damage;
                    let p_armor = weapon.stats.armor + armor.stats.armor + ring1.stats.armor + ring2.stats.armor;
                    let player = Fighter { health, damage: p_damage, armor: p_armor };

                    if !will_player_win(boss, &player) {
                        let cost = weapon.stats.cost + armor.stats.cost + ring1.stats.cost + ring2.stats.cost;
                        if cost > highest_cost {
                            highest_cost = cost;
                        }
                    }
                }
            }
        }
    }

    println!("{highest_cost}");
}

fn will_player_win(boss: &Fighter, player: &Fighter) -> bool {
    let mut boss = boss.clone();
    let mut player = player.clone();
    let player_damage_per_turn = player.damage - boss.armor;
    if player_damage_per_turn <= 0 {
        return false;
    }

    let boss_damage_per_turn = boss.damage - player.armor;
    if boss_damage_per_turn <= 0 {
        return true;
    }

    while player.health > 0 && boss.health > 0 {
        boss.health -= player_damage_per_turn;
        player.health -= boss_damage_per_turn;
    }

    if boss.health <= 0 {
        return true;
    }

    false
}

struct ItemStats {
    cost: i32,
    damage: i32,
    armor: i32,
}

struct Item {
    name: String,
    stats: ItemStats,
}

type Weapons = Vec<Item>;
type Armor = Vec<Item>;
type Rings = Vec<Item>;

struct Shop {
    weapons: Weapons,
    armor: Armor,
    rings: Rings,
}

#[derive(Clone)]
struct Fighter {
    health: i32,
    damage: i32,
    armor: i32,
}
