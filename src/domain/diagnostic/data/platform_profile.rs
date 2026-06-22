/// Delivery target profile. Rules severity changes depending on which platform
/// the video is being prepared for.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum PlatformProfile {
    /// Web / YouTube / streaming — H.264, AAC stereo, mp4 container
    #[default]
    Web,
    /// Broadcast / TV — mono tracks, PCM 16-bit, 48 kHz, mov/mxf
    Broadcast,
    /// Mobile — H.264 Baseline/Main, AAC, max 1080p, no 10-bit
    Mobile,
}

impl PlatformProfile {
    pub fn label(&self) -> &'static str {
        match self {
            PlatformProfile::Web => "Web",
            PlatformProfile::Broadcast => "Broadcast",
            PlatformProfile::Mobile => "Mobile",
        }
    }
}
