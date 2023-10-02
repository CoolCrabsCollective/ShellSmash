use crate::wave_manager::WaveDefinition;

pub(crate) const DEFINED_WAVES: [WaveDefinition; 2] = [
    WaveDefinition {
        // name: ("Beginning"),
        start_delay: 2.0,
        spawn_rate: 0.0,

        jellyfish_count: 5,
        urchin_count: 2,
        shrimp_count: 0,

        drop_item_count: 10,
    },
    WaveDefinition {
        // name: ("Gamer Mode"),
        start_delay: 3.0,
        spawn_rate: 0.25,

        jellyfish_count: 10,
        urchin_count: 5,
        shrimp_count: 1,

        drop_item_count: 10,
    },
];
