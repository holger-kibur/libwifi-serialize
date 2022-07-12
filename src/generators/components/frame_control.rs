use cookie_factory::{do_gen, gen_le_u8};

use crate::generators::{GenCursor, GenResult, SerializationError};
use crate::{frame::components::FrameControl, FrameSubType, FrameType};

/// Readability macro to panic with a helpful message in case of an illegal
/// frame subtype.
macro_rules! illegal_subtype {
    ($subtype:ident) => {
        match $subtype {
            FrameSubType::Reserved => Err(SerializationError::FCtrlReservedSubtype),
            FrameSubType::Unhandled => Err(SerializationError::FCtrlUnhandledSubtype),
            _ => Err(SerializationError::FCtrlInvalidSubtype),
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
        FrameType::Unknown => {
            unimplemented!("Serialization of extended frame types not supported!")
        }
    }?;
    let fctrl_byte_1 = ((ser_frame_subtype & 0x0F) << 4)
        | ((ser_frame_type & 0x03) << 2)
        | (frame_ctrl.protocol_version & 0x03);

    // Second byte is just the flags field, which is conveniently already in the
    // right format :).
    Ok(do_gen!(
        cursor,
        gen_le_u8!(fctrl_byte_1) >> gen_le_u8!(frame_ctrl.flags)
    )?)
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
fn gen_mgmt_subtype(frame_subtype: FrameSubType) -> Result<u8, SerializationError> {
    match frame_subtype {
        FrameSubType::AssociationRequest => Ok(0),
        FrameSubType::AssociationResponse => Ok(1),
        FrameSubType::ReassociationRequest => Ok(2),
        FrameSubType::ReassociationResponse => Ok(3),
        FrameSubType::ProbeRequest => Ok(4),
        FrameSubType::ProbeResponse => Ok(5),
        FrameSubType::TimingAdvertisement => Ok(6),
        FrameSubType::Beacon => Ok(8),
        FrameSubType::Atim => Ok(9),
        FrameSubType::Disassociation => Ok(10),
        FrameSubType::Authentication => Ok(11),
        FrameSubType::Deauthentication => Ok(12),
        FrameSubType::Action => Ok(13),
        FrameSubType::ActionNoAck => Ok(14),
        _ => illegal_subtype!(frame_subtype),
    }
}

/// Convert [FrameSubType] variant to corresponding header field value given
/// that this is a control frame.
fn gen_ctrl_subtype(frame_subtype: FrameSubType) -> Result<u8, SerializationError> {
    match frame_subtype {
        FrameSubType::Trigger => Ok(2),
        FrameSubType::Tack => Ok(3),
        FrameSubType::BeamformingReportPoll => Ok(4),
        FrameSubType::NdpAnnouncement => Ok(5),
        FrameSubType::ControlFrameExtension => Ok(6),
        FrameSubType::ControlWrapper => Ok(7),
        FrameSubType::BlockAckRequest => Ok(8),
        FrameSubType::BlockAck => Ok(9),
        FrameSubType::PsPoll => Ok(10),
        FrameSubType::Rts => Ok(11),
        FrameSubType::Cts => Ok(12),
        FrameSubType::Ack => Ok(13),
        FrameSubType::CfEnd => Ok(14),
        FrameSubType::CfEndCfAck => Ok(15),
        _ => illegal_subtype!(frame_subtype),
    }
}

/// Convert [FrameSubType] variant to corresponding header field value given
/// that this is a data frame.
fn gen_data_subtype(frame_subtype: FrameSubType) -> Result<u8, SerializationError> {
    match frame_subtype {
        FrameSubType::Data => Ok(0),
        FrameSubType::DataCfAck => Ok(1),
        FrameSubType::DataCfPoll => Ok(2),
        FrameSubType::DataCfAckCfPoll => Ok(3),
        FrameSubType::NullData => Ok(4),
        FrameSubType::CfAck => Ok(5),
        FrameSubType::CfPoll => Ok(6),
        FrameSubType::CfAckCfPoll => Ok(7),
        FrameSubType::QosData => Ok(8),
        FrameSubType::QosDataCfAck => Ok(9),
        FrameSubType::QosDataCfPoll => Ok(10),
        FrameSubType::QosDataCfAckCfPoll => Ok(11),
        FrameSubType::QosNull => Ok(12),
        FrameSubType::QosCfPoll => Ok(14),
        FrameSubType::QosCfAckCfPoll => Ok(15),
        _ => illegal_subtype!(frame_subtype),
    }
}
