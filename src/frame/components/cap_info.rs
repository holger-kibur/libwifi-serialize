use bitflags::bitflags;

bitflags! {
    pub struct CapabilityInfo: u16 {
        const ESS               = 0x0001;
        const IBSS              = 0x0002;
        const CF_POLLABLE       = 0x0004;
        const CF_REQUEST_POLL   = 0x0008;
        const PRIVACY           = 0x0010;
        const SHORT_PREAMBLE    = 0x0020;
        const PBCC              = 0x0040;
        const CHANNEL_AGILITY   = 0x0080;
        const SPECTRUM_MGMT     = 0x0100;
        const QOS               = 0x0200;
        const SHORT_TIME_SLOT   = 0x0400;
        const AUTO_PWR_SAVE     = 0x0800;
        const MEASURE_RADIO     = 0x1000;
        const DSSS_OFDM         = 0x2000;
        const DELAY_BLOCK_ACK   = 0x4000;
        const IMM_BLOCK_ACK     = 0x8000;
    }
}
