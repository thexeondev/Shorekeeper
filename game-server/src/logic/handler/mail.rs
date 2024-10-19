use shorekeeper_protocol::{MailBind, MailBindInfoRequest, MailBindInfoResponse};

use crate::logic::player::Player;

pub fn on_mail_bind_info_request(
    _: &Player,
    _: MailBindInfoRequest,
    response: &mut MailBindInfoResponse,
) {
    // TODO: Implement this
    response.mail_bind = Some(MailBind {
        is_bind: true,
        is_reward: true,
        close_time: -1,
    });
}
