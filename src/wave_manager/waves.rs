use crate::wave_manager::WaveDefinition;

pub(crate) const DEFINED_WAVES: [WaveDefinition; 2] = [
    WaveDefinition {
        // name: ("Beginning"),
        start_delay: 2.0,
        spawn_rate: 0.0,

        enemy_count: 5,
        shrimp_count: 0,

        drop_item_count: 10,
    },
    WaveDefinition {
        // name: ("Gamer Mode"),
        start_delay: 3.0,
        spawn_rate: 0.25,

        enemy_count: 10,
        shrimp_count: 0,

        drop_item_count: 10,
    },
];
