use crate::wave_manager::WaveDefinition;
use rand::Rng;

pub(crate) const DEFINED_WAVES: [WaveDefinition; 5] = [
    WaveDefinition {
        // name: ("Beginning"),
        start_delay: 5.0,
        spawn_rate: 0.0,

        jellyfish_count: 5,
        urchin_count: 2,
        shrimp_count: 0,

        luck: 1,

        drop_item_count: 10,
    },
    WaveDefinition {
        // name: ("Gamer Mode"),
        start_delay: 3.0,
        spawn_rate: 0.25,

        jellyfish_count: 7,
        urchin_count: 5,
        shrimp_count: 0,

        luck: 0,

        drop_item_count: 10,
    },
    WaveDefinition {
        // name: ("Gamer Mode"),
        start_delay: 7.0,
        spawn_rate: 1.5,

        jellyfish_count: 20,
        urchin_count: 5,
        shrimp_count: 0,

        luck: 2,

        drop_item_count: 10,
    },
    WaveDefinition {
        // name: ("Gamer Mode"),
        start_delay: 7.0,
        spawn_rate: 1.25,

        jellyfish_count: 15,
        urchin_count: 20,
        shrimp_count: 3,

        luck: 0,

        drop_item_count: 10,
    },
    WaveDefinition {
        // name: ("Gamer Mode"),
        start_delay: 7.0,
        spawn_rate: 0.25,

        jellyfish_count: 20,
        urchin_count: 30,
        shrimp_count: 0,

        luck: 0,

        drop_item_count: 10,
    },
];

pub fn wave_generation(wave_count: i32) -> WaveDefinition {
    let mut rng = rand::thread_rng();

    let mut total_enemy_count = wave_count * 4;

    let mut jellyfish = 0;
    let mut urchin = 0;
    let mut shrimp = 0;

    while total_enemy_count > 0 {
        let rand_enemy = rng.gen_range(0..20);

        match rand_enemy {
            0..=6 => {
                urchin += 1;
            }
            6..=10 => {
                shrimp += 1;
            }
            _ => {
                jellyfish += 1;
            }
        }

        total_enemy_count -= 1;
    }

    let mut luck_value: i32 = 0;
    match wave_count {
        10 => {
            luck_value = 2;
        }
        _ => {}
    }

    return WaveDefinition {
        // name: ("Gamer Mode"),
        start_delay: 1.5,
        spawn_rate: 0.75,

        jellyfish_count: jellyfish,
        urchin_count: urchin,
        shrimp_count: shrimp,

        luck: luck_value,

        drop_item_count: 10,
    };
}
