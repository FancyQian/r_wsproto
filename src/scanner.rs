#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EspBleCtlMessage {
    #[prost(bool, tag="1")]
    pub on: bool,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScannerMessage {
    #[prost(enumeration="ScannerMessageType", tag="1")]
    pub r#type: i32,
    /// Type Data
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Type EspCtl
    #[prost(enumeration="EspCtlCmd", tag="3")]
    pub esp_cmd: i32,
    #[prost(string, tag="4")]
    pub ota_url: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub voice_url: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub blemesg: ::core::option::Option<EspBleCtlMessage>,
    /// Type ScannerCtl
    #[prost(enumeration="ScannerCtlCmd", tag="7")]
    pub s_cmd: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScannerMessages {
    #[prost(message, repeated, tag="1")]
    pub mesgs: ::prost::alloc::vec::Vec<ScannerMessage>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScannerMessageType {
    Data = 0,
    ScannerCtl = 1,
    EspCtl = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScannerCtlCmd {
    FactoryReset = 0,
    ManualMode = 1,
    AutoMode = 2,
    AutoFast = 3,
    TiggerScanner = 4,
    ScanOff = 5,
    ScanTimeoutAlwaysOpen = 6,
    EnableScanFailTips = 7,
    DisableScanFailTips = 8,
    TailCrlf = 9,
    Version = 10,
    IsAlive = 11,
    Reboot = 12,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EspCtlCmd {
    EspFactoryReset = 0,
    Ota = 1,
    VoiceUpdate = 2,
    Ble = 3,
}
