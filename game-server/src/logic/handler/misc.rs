use shorekeeper_protocol::{IYa, InputSettingRequest, InputSettingResponse};

use crate::logic::player::Player;

pub fn on_input_setting_request(
    _: &Player,
    _: InputSettingRequest,
    response: &mut InputSettingResponse,
) {
    response.i_ya = Some(IYa::default());
}
