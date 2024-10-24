use crate::logic::player::Player;
use crate::logic::role::{Role, RoleFormation};
use shorekeeper_protocol::{
    ClientCurrentRoleReportRequest, ClientCurrentRoleReportResponse, ERemoveEntityType, ErrorCode,
    FormationAttrRequest, FormationAttrResponse, RoleFavorListRequest, RoleFavorListResponse,
    RoleShowListUpdateRequest, RoleShowListUpdateResponse, UpdateFormationRequest,
    UpdateFormationResponse,
};
use std::collections::HashSet;

pub fn on_role_show_list_update_request(
    player: &mut Player,
    request: RoleShowListUpdateRequest,
    response: &mut RoleShowListUpdateResponse,
) {
    let role_ids: HashSet<i32> = player.role_list.keys().cloned().collect();
    let all_exist = request.role_list.iter().all(|id| role_ids.contains(id));

    if all_exist {
        player.basic_info.role_show_list = request.role_list;
        response.error_code = ErrorCode::Success.into();
    } else {
        response.error_code = ErrorCode::InvalidRequest.into(); // TODO: replace with appropriate error code
    }
}

pub fn on_client_current_role_report_request(
    _player: &Player,
    request: ClientCurrentRoleReportRequest,
    response: &mut ClientCurrentRoleReportResponse,
) {
    response.current_entity_id = request.current_entity_id;
    response.player_id = request.player_id;
}

pub fn on_role_favor_list_request(
    _player: &Player,
    _request: RoleFavorListRequest,
    response: &mut RoleFavorListResponse,
) {
    response.favor_list = vec![]; // TODO: add favor
    response.error_code = ErrorCode::Success.into();
}

pub fn on_formation_attr_request(
    _player: &Player,
    _request: FormationAttrRequest,
    response: &mut FormationAttrResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_update_formation_request(
    player: &mut Player,
    request: UpdateFormationRequest,
    response: &mut UpdateFormationResponse,
) {
    let mut world_ref = player.world.borrow_mut();
    let world = world_ref.get_mut_world_entity();

    for formation in request.formations {
        let formation_id = formation.formation_id;
        let cur_role = formation.cur_role;
        let is_current = formation.is_current;

        if is_current {
            // update player current formation id
            player.cur_formation_id = formation_id;

            // search old formation id and set real_formation_id, set is_current to false
            let mut real_formation_id = formation_id;
            if let Some(rf) = player
                .formation_list
                .values_mut()
                .find(|rf| rf.is_current && rf.id != formation_id)
            {
                real_formation_id = rf.id;
                rf.is_current = false;
            }

            if let Some(old_formation) = player.formation_list.get(&real_formation_id) {
                let removed_entities: Vec<i64> = old_formation
                    .role_ids
                    .iter()
                    .map(|&role_id| world.get_entity_id(role_id))
                    .collect();
                removed_entities.iter().for_each(|&entity_id| {
                    world.remove_entity(entity_id as i32);
                });
                player.notify(player.build_player_entity_remove_notify(
                    removed_entities,
                    ERemoveEntityType::RemoveTypeNormal,
                ));
            }

            let added_roles: Vec<Role> = formation
                .role_ids
                .iter()
                .map(|&role_id| Role::new(role_id))
                .collect();

            if !added_roles.is_empty() {
                // add new roles
                player.notify(player.build_player_entity_add_notify(added_roles, world));
            }

            // send update group formation notify
            player.notify(player.build_update_group_formation_notify(
                RoleFormation {
                    id: formation_id,
                    cur_role,
                    role_ids: formation.role_ids.clone(),
                    is_current,
                },
                world,
            ));

            response.formation = Some(formation.clone());
        }

        // update all formation and check formation_list
        player
            .formation_list
            .entry(formation_id)
            .and_modify(|r| {
                r.cur_role = formation.cur_role;
                r.role_ids = formation.role_ids.clone();
                r.is_current = is_current;
            })
            .or_insert(RoleFormation {
                id: formation_id,
                cur_role: formation.cur_role,
                role_ids: formation.role_ids,
                is_current,
            });
    }

    player.notify(player.build_update_formation_notify());

    response.error_code = ErrorCode::Success.into();
}
