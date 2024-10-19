use shorekeeper_protocol::combat_message::{CombatSendPackRequest, CombatSendPackResponse};
use shorekeeper_protocol::ErrorCode;

use crate::logic::player::Player;

pub fn on_combat_message_combat_send_pack_request(
    _player: &Player,
    request: CombatSendPackRequest,
    response: &mut CombatSendPackResponse,
) {
    tracing::debug!("CombatSendPackRequest: for {:?}", request);
    // TODO: Implement this
    response.error_code = ErrorCode::Success.into();
}
