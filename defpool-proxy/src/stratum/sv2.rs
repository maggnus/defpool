// Stratum V2 message handling
// This module provides utilities for working with SV2 messages

use stratum_core::parsers_sv2::AnyMessage;

/// Extract message type from SV2 message
#[allow(dead_code)]
pub fn get_message_type(msg: &AnyMessage) -> &'static str {
    match msg {
        AnyMessage::Mining(mining_msg) => {
            use stratum_core::parsers_sv2::Mining;
            match mining_msg {
                Mining::OpenStandardMiningChannel(_) => "OpenStandardMiningChannel",
                Mining::OpenStandardMiningChannelSuccess(_) => "OpenStandardMiningChannelSuccess",
                Mining::NewMiningJob(_) => "NewMiningJob",
                Mining::SetNewPrevHash(_) => "SetNewPrevHash",
                Mining::SubmitSharesStandard(_) => "SubmitSharesStandard",
                Mining::SubmitSharesSuccess(_) => "SubmitSharesSuccess",
                Mining::SubmitSharesError(_) => "SubmitSharesError",
                Mining::SetTarget(_) => "SetTarget",
                _ => "Unknown Mining Message",
            }
        }
        AnyMessage::Common(common_msg) => {
            use stratum_core::parsers_sv2::CommonMessages;
            match common_msg {
                CommonMessages::SetupConnection(_) => "SetupConnection",
                CommonMessages::SetupConnectionSuccess(_) => "SetupConnectionSuccess",
                CommonMessages::SetupConnectionError(_) => "SetupConnectionError",
                CommonMessages::ChannelEndpointChanged(_) => "ChannelEndpointChanged",
                _ => "Unknown Common Message",
            }
        }
        _ => "Unknown Message Type",
    }
}

/// Check if message is a share submission
#[allow(dead_code)]
pub fn is_share_submission(msg: &AnyMessage) -> bool {
    matches!(
        msg,
        AnyMessage::Mining(stratum_core::parsers_sv2::Mining::SubmitSharesStandard(_))
    )
}

/// Check if message is a new job
#[allow(dead_code)]
pub fn is_new_job(msg: &AnyMessage) -> bool {
    matches!(
        msg,
        AnyMessage::Mining(stratum_core::parsers_sv2::Mining::NewMiningJob(_))
    )
}
