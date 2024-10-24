use crate::logic::player::Player;
use shorekeeper_protocol::combat_message::{
    combat_receive_data, combat_request_data, combat_response_data, combat_send_data,
    CombatReceiveData, CombatRequestData, CombatResponseData, CombatSendPackRequest,
    CombatSendPackResponse,
};
use shorekeeper_protocol::{ErrorCode, SwitchRoleRequest, SwitchRoleResponse};

#[inline(always)]
fn create_combat_response(
    combat_request: &CombatRequestData,
    message: combat_response_data::Message,
) -> CombatReceiveData {
    CombatReceiveData {
        message: Some(combat_receive_data::Message::CombatResponseData(
            CombatResponseData {
                combat_common: combat_request.combat_common,
                request_id: combat_request.request_id,
                message: Some(message),
            },
        )),
    }
}

pub fn on_combat_message_combat_send_pack_request(
    player: &mut Player,
    request: CombatSendPackRequest,
    response: &mut CombatSendPackResponse,
) {
    for data in request.data.iter() {
        if let Some(combat_send_data::Message::Request(ref request_data)) = data.message {
            if let Some(ref request_message) = request_data.message {
                match request_message {
                    combat_request_data::Message::SwitchRoleRequest(ref request) => {
                        handle_switch_role_request(player, request_data, request, response);
                    }
                    _ => {}
                }
            }
        }
    }
    response.error_code = ErrorCode::Success.into();
}

fn handle_switch_role_request(
    player: &mut Player,
    combat_request: &CombatRequestData,
    request: &SwitchRoleRequest,
    response: &mut CombatSendPackResponse,
) {
    // Find current formation and update current role
    if let Some(formation) = player.formation_list.values_mut().find(|f| f.is_current) {
        formation.cur_role = request.role_id;

        let receive_pack = response
            .receive_pack_notify
            .get_or_insert_with(Default::default);

        receive_pack.data.push(create_combat_response(
            combat_request,
            combat_response_data::Message::SwitchRoleResponse(SwitchRoleResponse {
                error_code: ErrorCode::Success.into(),
                role_id: request.role_id,
            }),
        ));
    } else {
        tracing::error!("Role with id {} not found", request.role_id);
        response.error_code = ErrorCode::ErrSwitchRoleEntityNotExist.into();
        return;
    }

    response.error_code = ErrorCode::Success.into();
}
