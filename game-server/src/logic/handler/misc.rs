use shorekeeper_protocol::{ErrorCode, Hih, InputSettingRequest, InputSettingResponse, InputSettingUpdateRequest, InputSettingUpdateResponse, LanguageSettingUpdateRequest, LanguageSettingUpdateResponse, ServerPlayStationPlayOnlyStateRequest, ServerPlayStationPlayOnlyStateResponse, VersionInfoPush};

use crate::logic::player::Player;

pub fn on_input_setting_request(
    _: &Player,
    _: InputSettingRequest,
    response: &mut InputSettingResponse,
) {
    response.hih = Some(Hih::default());
}

pub fn on_input_setting_update_request(
    _: &Player,
    _: InputSettingUpdateRequest,
    response: &mut InputSettingUpdateResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_language_setting_update_request(
    _: &Player,
    _: LanguageSettingUpdateRequest,
    response: &mut LanguageSettingUpdateResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_server_play_station_play_only_state_request(
    _: &Player,
    _: ServerPlayStationPlayOnlyStateRequest,
    response: &mut ServerPlayStationPlayOnlyStateResponse,
) {
    response.play_station_play_only_state = false;
}

pub fn on_version_info_push(player: &Player, push: VersionInfoPush) {
    // TODO: Shall we do safety check and ensure we have compatible versions?
    tracing::debug!(
        "Client versions: launcher: {}, app: {}, resources: {}",
        push.launcher_version,
        push.app_version,
        push.resource_version
    );
}
