use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BasePropertyData {
    pub id: i32,
    pub lv: i32,
    pub life_max: i32,
    pub life: i32,
    pub sheild: i32,
    pub sheild_damage_change: i32,
    pub sheild_damage_reduce: i32,
    pub atk: i32,
    pub crit: i32,
    pub crit_damage: i32,
    pub def: i32,
    pub energy_efficiency: i32,
    pub cd_reduse: i32,
    pub reaction_efficiency: i32,
    pub damage_change_normal_skill: i32,
    pub damage_change: i32,
    pub damage_reduce: i32,
    pub damage_change_auto: i32,
    pub damage_change_cast: i32,
    pub damage_change_ultra: i32,
    pub damage_change_qte: i32,
    pub damage_change_phys: i32,
    pub damage_change_element1: i32,
    pub damage_change_element2: i32,
    pub damage_change_element3: i32,
    pub damage_change_element4: i32,
    pub damage_change_element5: i32,
    pub damage_change_element6: i32,
    pub damage_resistance_phys: i32,
    pub damage_resistance_element1: i32,
    pub damage_resistance_element2: i32,
    pub damage_resistance_element3: i32,
    pub damage_resistance_element4: i32,
    pub damage_resistance_element5: i32,
    pub damage_resistance_element6: i32,
    pub heal_change: i32,
    pub healed_change: i32,
    pub damage_reduce_phys: i32,
    pub damage_reduce_element1: i32,
    pub damage_reduce_element2: i32,
    pub damage_reduce_element3: i32,
    pub damage_reduce_element4: i32,
    pub damage_reduce_element5: i32,
    pub damage_reduce_element6: i32,
    pub reaction_change1: i32,
    pub reaction_change2: i32,
    pub reaction_change3: i32,
    pub reaction_change4: i32,
    pub reaction_change5: i32,
    pub reaction_change6: i32,
    pub reaction_change7: i32,
    pub reaction_change8: i32,
    pub reaction_change9: i32,
    pub reaction_change10: i32,
    pub reaction_change11: i32,
    pub reaction_change12: i32,
    pub reaction_change13: i32,
    pub reaction_change14: i32,
    pub reaction_change15: i32,
    pub energy_max: i32,
    pub energy: i32,
    pub special_energy_1_max: i32,
    pub special_energy_1: i32,
    pub special_energy_2_max: i32,
    pub special_energy_2: i32,
    pub special_energy_3_max: i32,
    pub special_energy_3: i32,
    pub special_energy_4_max: i32,
    pub special_energy_4: i32,
    pub strength_max: i32,
    pub strength: i32,
    pub strength_recover: i32,
    pub strength_punish_time: i32,
    pub strength_run: i32,
    pub strength_swim: i32,
    pub strength_fast_swim: i32,
    pub hardness_max: i32,
    pub hardness: i32,
    pub hardness_recover: i32,
    pub hardness_punish_time: i32,
    pub hardness_change: i32,
    pub hardness_reduce: i32,
    pub rage_max: i32,
    pub rage: i32,
    pub rage_recover: i32,
    pub rage_punish_time: i32,
    pub rage_change: i32,
    pub rage_reduce: i32,
    pub tough_max: i32,
    pub tough: i32,
    pub tough_recover: i32,
    pub tough_change: i32,
    pub tough_reduce: i32,
    pub tough_recover_delay_time: i32,
    pub element_power1: i32,
    pub element_power2: i32,
    pub element_power3: i32,
    pub element_power4: i32,
    pub element_power5: i32,
    pub element_power6: i32,
    pub special_damage_change: i32,
    pub strength_fast_climb_cost: i32,
    pub element_property_type: i32,
    pub weak_time: i32,
    pub ignore_def_rate: i32,
    pub ignore_damage_resistance_phys: i32,
    pub ignore_damage_resistance_element1: i32,
    pub ignore_damage_resistance_element2: i32,
    pub ignore_damage_resistance_element3: i32,
    pub ignore_damage_resistance_element4: i32,
    pub ignore_damage_resistance_element5: i32,
    pub ignore_damage_resistance_element6: i32,
    pub skill_tough_ratio: i32,
    pub strength_climb_jump: i32,
    pub strength_gliding: i32,
    pub mass: i32,
    pub braking_friction_factor: i32,
    pub gravity_scale: i32,
    pub speed_ratio: i32,
    pub damage_change_phantom: i32,
    pub auto_attack_speed: i32,
    pub cast_attack_speed: i32,
    pub status_build_up_1_max: i32,
    pub status_build_up_1: i32,
    pub status_build_up_2_max: i32,
    pub status_build_up_2: i32,
    pub status_build_up_3_max: i32,
    pub status_build_up_3: i32,
    pub status_build_up_4_max: i32,
    pub status_build_up_4: i32,
    pub status_build_up_5_max: i32,
    pub status_build_up_5: i32,
    pub paralysis_time_max: i32,
    pub paralysis_time: i32,
    pub paralysis_time_recover: i32,
    pub element_energy_max: i32,
    pub element_energy: i32,
}