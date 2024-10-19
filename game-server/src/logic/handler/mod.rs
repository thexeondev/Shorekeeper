mod combat;
mod entity;
mod guide;
mod mail;
mod misc;
mod scene;
mod skill;

pub use combat::*;
pub use entity::*;
pub use guide::*;
pub use mail::*;
pub use misc::*;
pub use scene::*;
pub use skill::*;

use shorekeeper_protocol::message::Message;

macro_rules! handle_request {
    ($($name:ident $(, $inner_package:ident)?;)*) => {
        fn handle_request(player: &mut super::player::Player, mut msg: Message) {
            use ::shorekeeper_protocol::{MessageID, Protobuf};

            ::paste::paste! {
                match msg.get_message_id() {
                    $(
                        ::shorekeeper_protocol::$($inner_package::)?[<$name Request>]::MESSAGE_ID => {
                            let Ok(request) = ::shorekeeper_protocol::$($inner_package::)?[<$name Request>]::decode(&*msg.remove_payload()) else {
                                tracing::debug!("failed to decode {}, player_id: {}", stringify!($($inner_package::)?[<$name Request>]), player.basic_info.id);
                                return;
                            };

                            tracing::debug!("logic: processing request {}", stringify!($($inner_package::)?[<$name Request>]));

                            let mut response = ::shorekeeper_protocol::$($inner_package::)?[<$name Response>]::default();
                            [<on_ $($inner_package:snake _)? $name:snake _request>](player, request, &mut response);

                            player.respond(response, msg.get_rpc_id());
                        },
                    )*
                    unhandled => {
                         ::tracing::warn!("can't find handler for request with message_id={unhandled}");
                         let tmp = &*msg.remove_payload();
                         let (name, value) = shorekeeper_protocol::proto_dumper::get_debug_info(
                             unhandled, tmp,
                         ).unwrap_or_else(|err| ("Error", err.to_string()));
                        tracing::debug!("trying to log unhandled data for message {name} with:\n{value}")
                    }
                }
            }
        }
    };
}

macro_rules! handle_push {
    ($($name:ident $(, $inner_package:ident)?;)*) => {
        fn handle_push(player: &mut super::player::Player, mut msg: Message) {
            use ::shorekeeper_protocol::{MessageID, Protobuf};

            ::paste::paste! {
                match msg.get_message_id() {
                    $(
                        ::shorekeeper_protocol::$($inner_package::)?[<$name Push>]::MESSAGE_ID => {
                            let Ok(push) = ::shorekeeper_protocol::$($inner_package::)?[<$name Push>]::decode(&*msg.remove_payload()) else {
                                tracing::debug!("failed to decode {}, player_id: {}", stringify!($($inner_package::)?[<$name Push>]), player.basic_info.id);
                                return;
                            };

                            tracing::debug!("logic: processing push {}", stringify!($($inner_package::)?[<$name Push>]));

                            [<on_ $($inner_package:snake _)? $name:snake _push>](player, push);
                        },
                    )*
                    unhandled => {
                         ::tracing::warn!("can't find handler for push with message_id={unhandled}");
                         let tmp = &*msg.remove_payload();
                         let (name, value) = shorekeeper_protocol::proto_dumper::get_debug_info(
                             unhandled, tmp,
                         ).unwrap_or_else(|err| ("Error", err.to_string()));
                        tracing::debug!("trying to log unhandled data for message {name} with:\n{value}")
                    }
                }
            }
        }
    };
}

handle_request! {
    // Combat
    CombatSendPack, combat_message;

    // Entity
    EntityActive;
    EntityOnLanded;
    EntityPosition;
    EntityLoadComplete;

    // Guide
    GuideInfo;

    // Mail
    MailBindInfo;

    // Misc
    InputSetting;
    InputSettingUpdate;
    LanguageSettingUpdate;
    ServerPlayStationPlayOnlyState;

    // Scene
    SceneTrace;
    SceneLoadingFinish;
    UpdateSceneDate;

    // Skill
    VisionExploreSkillSet;
}

handle_push! {
    // Entity
    MovePackage;

    // Misc
    VersionInfo;
}

pub fn handle_logic_message(player: &mut super::player::Player, msg: Message) {
    match msg {
        Message::Request { .. } => handle_request(player, msg),
        Message::Push { .. } => handle_push(player, msg),
        _ => tracing::warn!(
            "handle_logic_message: wrong message type: {}, message_id: {}, player_id: {}",
            msg.get_message_type(),
            msg.get_message_id(),
            player.basic_info.id,
        ),
    }
}
