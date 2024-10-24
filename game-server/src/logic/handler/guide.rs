use shorekeeper_protocol::{GuideInfoRequest, GuideInfoResponse};

use crate::logic::player::Player;

pub fn on_guide_info_request(
    _player: &Player,
    _request: GuideInfoRequest,
    response: &mut GuideInfoResponse,
) {
    // TODO: Implement this
    response.guide_group_finish_list = Vec::new();
}
