use shorekeeper_protocol::{ErrorCode, SceneLoadingFinishRequest, SceneLoadingFinishResponse,
                           SceneTraceRequest, SceneTraceResponse, UpdateSceneDateRequest,
                           UpdateSceneDateResponse,
};

use crate::logic::player::Player;

pub fn on_scene_trace_request(
    _player: &Player,
    request: SceneTraceRequest,
    _: &mut SceneTraceResponse,
) {
    tracing::debug!("SceneTraceRequest: trace id {}", request.scene_trace_id);
}

pub fn on_scene_loading_finish_request(
    _player: &Player,
    request: SceneLoadingFinishRequest,
    response: &mut SceneLoadingFinishResponse,
) {
    // TODO: Implement this if needed
    response.error_code = ErrorCode::Success.into();
}

pub fn on_update_scene_date_request(
    _player: &Player,
    _request: UpdateSceneDateRequest,
    response: &mut UpdateSceneDateResponse,
) {
    response.error_code = ErrorCode::Success.into();
}
