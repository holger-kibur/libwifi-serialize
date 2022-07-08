use cookie_factory::{do_gen, gen_le_u8};

use crate::{frame::components::FrameControl, FrameType, FrameSubType};
use crate::generators::{GenCursor, GenResult};

/// Readability macro to panic with a helpful message in case of an illegal
/// frame subtype.
macro_rules! illegal_subtype {
    ($subtype:ident) => {
        match $subtype {
            FrameSubType::Reserved => panic!("Can't serialize frame with reserved subtype!"),
            FrameSubType::Unhandled => panic!("Can't serialize frame with unhandled subtype!"),
            _ => panic!("Unsuitable subframe type for frame type: {:?}!", $subtype),
        }
    };
}

/// Serializer for the two byte [FrameControl] header common to all 802.11 frames.
pub fn gen_frame_control<'a>(cursor: GenCursor<'a>, frame_ctrl: &FrameControl) -> GenResult<'a> {
    // Compose first byte of frame control header
    let ser_frame_type = gen_frame_type(frame_ctrl.frame_type);
    let ser_frame_subtype = match frame_ctrl.frame_type {
        FrameType::Management => gen_mgmt_subtype(frame_ctrl.frame_subtype),
        FrameType::Control => gen_ctrl_subtype(frame_ctrl.frame_subtype),
        FrameType::Data => gen_data_subtype(frame_ctrl.frame_subtype),
        FrameType::Unknown => unimplemented!("Serialization of extended frame types not supported!"),
    };
    let fctrl_byte_1 = (frame_ctrl.protocol_version << 6) | ((ser_frame_type & 0x03) << 4) | (ser_frame_subtype & 0x0F);

    // Second byte is just the flags field, which is conveniently already in the
    // right format :).
    do_gen!(cursor, gen_le_u8!(fctrl_byte_1) >> gen_le_u8!(frame_ctrl.flags))
}

/// Convert [FrameType] variant to corresponding header field value.
fn gen_frame_type(frame_type: FrameType) -> u8 {
    match frame_type {
        FrameType::Management => 0,
        FrameType::Control => 1,
        FrameType::Data => 2,
        FrameType::Unknown => 3,
    }
}

/// Convert [FrameSubType] variant to corresponding header field value given
/// that this is a management frame.
fn gen_mgmt_subtype(frame_subtype: FrameSubType) -> u8 {
    match frame_subtype {
        FrameSubType::AssociationRequest => 0,
        FrameSubType::AssociationResponse => 1,
        FrameSubType::ReassociationRequest => 2,
        FrameSubType::ReassociationResponse => 3,
        FrameSubType::ProbeRequest => 4,
        FrameSubType::ProbeResponse => 5,
        FrameSubType::TimingAdvertisement => 6,
        FrameSubType::Beacon => 8,
        FrameSubType::Atim => 9,
        FrameSubType::Disassociation => 10,
        FrameSubType::Authentication => 11,
        FrameSubType::Deauthentication => 12,
        FrameSubType::Action => 13,
        FrameSubType::ActionNoAck => 14,
        _ => illegal_subtype!(frame_subtype),
    }
}

/// Convert [FrameSubType] variant to corresponding header field value given
/// that this is a control frame.
fn gen_ctrl_subtype(frame_subtype: FrameSubType) -> u8 {
    match frame_subtype {
        FrameSubType::Trigger => 2,
        FrameSubType::Tack => 3,
        FrameSubType::BeamformingReportPoll => 4,
        FrameSubType::NdpAnnouncement => 5,
        FrameSubType::ControlFrameExtension => 6,
        FrameSubType::ControlWrapper => 7,
        FrameSubType::BlockAckRequest => 8,
        FrameSubType::BlockAck => 9,
        FrameSubType::PsPoll => 10,
        FrameSubType::Rts => 11,
        FrameSubType::Cts => 12,
        FrameSubType::Ack => 13,
        FrameSubType::CfEnd => 14,
        FrameSubType::CfEndCfAck => 15,
        _ => illegal_subtype!(frame_subtype),
    }
}

/// Convert [FrameSubType] variant to corresponding header field value given
/// that this is a data frame.
fn gen_data_subtype(frame_subtype: FrameSubType) -> u8 {
    match frame_subtype {
        FrameSubType::Data => 0,
        FrameSubType::DataCfAck => 1,
        FrameSubType::DataCfPoll => 2,
        FrameSubType::DataCfAckCfPoll => 3,
        FrameSubType::NullData => 4,
        FrameSubType::CfAck => 5,
        FrameSubType::CfPoll => 6,
        FrameSubType::CfAckCfPoll => 7,
        FrameSubType::QosData => 8,
        FrameSubType::QosDataCfAck => 9,
        FrameSubType::QosDataCfPoll => 10,
        FrameSubType::QosDataCfAckCfPoll => 11,
        FrameSubType::QosNull => 12,
        FrameSubType::QosCfPoll => 14,
        FrameSubType::QosCfAckCfPoll => 15,
        _ => illegal_subtype!(frame_subtype),
    }
}