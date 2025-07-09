//! response code enum

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Code {
    #[default]
    Positive = 0x00,

    GeneralReject = 0x10,
    ServiceNotSupported = 0x11,
    SubFunctionNotSupported = 0x12,
    IncorrectMessageLengthOrInvalidFormat = 0x13,
    ResponseTooLong = 0x14,

    BusyRepeatRequest = 0x21,
    ConditionsNotCorrect = 0x22,

    RequestSequenceError = 0x24,
    NoResponseFromSubnetComponent = 0x25,
    FailurePreventsExecutionOfRequestedAction = 0x26,

    RequestOutOfRange = 0x31,

    SecurityAccessDenied = 0x33,
    AuthenticationRequired = 0x34,
    InvalidKey = 0x35,
    ExceedNumberOfAttempts = 0x36,
    RequiredTimeDelayNotExpired = 0x37,
    SecureDataTransmissionRequired = 0x38,
    SecureDataTransmissionNotAllowed = 0x39,
    SecureDataVerificationFailed = 0x3A,

    CertificateVerificationFailedInvalidTimePeriod = 0x50,
    CertificateVerificationFailedInvalidSignature = 0x51,
    CertificateVerificationFailedInvalidChainOfTrust = 0x52,
    CertificateVerificationFailedInvalidType = 0x53,
    CertificateVerificationFailedInvalidFormat = 0x54,
    CertificateVerificationFailedInvalidContent = 0x55,
    CertificateVerificationFailedInvalidScope = 0x56,
    CertificateVerificationFailedInvalidCertificate = 0x57,
    OwnershipVerificationFailed = 0x58,
    ChallengeCalculationFailed = 0x59,
    SettingAccessRightsFailed = 0x5A,
    SessionKeyCreationDerivationFailed = 0x5B,
    ConfigurationDataUsageFailed = 0x5C,
    DeAuthenticationFailed = 0x5D,

    UploadDownloadNotAccepted = 0x70,
    TransferDataSuspended = 0x71,
    GeneralProgrammingFailure = 0x72,
    WrongBlockSequenceCounter = 0x73,

    RequestCorrectlyReceivedResponsePending = 0x78,

    SubFunctionNotSupportedInActiveSession = 0x7E,
    ServiceNotSupportedInActiveSession = 0x7F,

    RpmTooHigh = 0x81,
    RpmTooLow = 0x82,
    EngineIsRunning = 0x83,
    EngineIsNotRunning = 0x84,
    EngineRunTimeTooLow = 0x85,
    TemperatureTooHigh = 0x86,
    TemperatureTooLow = 0x87,
    VehicleSpeedTooHigh = 0x88,
    VehicleSpeedTooLow = 0x89,
    ThrottlePedalTooHigh = 0x8A,
    ThrottlePedalTooLow = 0x8B,
    TransmissionRangeNotInNeutral = 0x8C,
    TransmissionRangeNotInGear = 0x8D,
    BrakeSwitchNotClosed = 0x8F,
    ShifterLeverNotInPark = 0x90,
    TorqueConverterClutchLocked = 0x91,
    VoltageTooHigh = 0x92,
    VoltageTooLow = 0x93,
    ResourceTemporarilyNotAvailable = 0x94,
    VehicleManufacturerSpecific(u8), // 0xF0~0xFE
    Reserved(u8),
}

impl From<u8> for Code {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::Positive,

            0x10 => Self::GeneralReject,
            0x11 => Self::ServiceNotSupported,
            0x12 => Self::SubFunctionNotSupported,
            0x13 => Self::IncorrectMessageLengthOrInvalidFormat,
            0x14 => Self::ResponseTooLong,

            0x21 => Self::BusyRepeatRequest,

            0x22 => Self::ConditionsNotCorrect,
            0x24 => Self::RequestSequenceError,
            0x25 => Self::NoResponseFromSubnetComponent,
            0x26 => Self::FailurePreventsExecutionOfRequestedAction,

            0x31 => Self::RequestOutOfRange,
            0x33 => Self::SecurityAccessDenied,
            0x34 => Self::AuthenticationRequired,
            0x35 => Self::InvalidKey,
            0x36 => Self::ExceedNumberOfAttempts,
            0x37 => Self::RequiredTimeDelayNotExpired,
            0x38 => Self::SecureDataTransmissionRequired,
            0x39 => Self::SecureDataTransmissionNotAllowed,
            0x3A => Self::SecureDataVerificationFailed,

            0x50 => Self::CertificateVerificationFailedInvalidTimePeriod,
            0x51 => Self::CertificateVerificationFailedInvalidSignature,
            0x52 => Self::CertificateVerificationFailedInvalidChainOfTrust,
            0x53 => Self::CertificateVerificationFailedInvalidType,
            0x54 => Self::CertificateVerificationFailedInvalidFormat,
            0x55 => Self::CertificateVerificationFailedInvalidContent,
            0x56 => Self::CertificateVerificationFailedInvalidScope,
            0x57 => Self::CertificateVerificationFailedInvalidCertificate,
            0x58 => Self::OwnershipVerificationFailed,
            0x59 => Self::ChallengeCalculationFailed,
            0x5A => Self::SettingAccessRightsFailed,
            0x5B => Self::SessionKeyCreationDerivationFailed,
            0x5C => Self::ConfigurationDataUsageFailed,
            0x5D => Self::DeAuthenticationFailed,

            0x70 => Self::UploadDownloadNotAccepted,
            0x71 => Self::TransferDataSuspended,
            0x72 => Self::GeneralProgrammingFailure,
            0x73 => Self::WrongBlockSequenceCounter,

            0x78 => Self::RequestCorrectlyReceivedResponsePending,

            0x7E => Self::SubFunctionNotSupportedInActiveSession,
            0x7F => Self::ServiceNotSupportedInActiveSession,

            0x81 => Self::RpmTooHigh,
            0x82 => Self::RpmTooLow,
            0x83 => Self::EngineIsRunning,
            0x84 => Self::EngineIsNotRunning,
            0x85 => Self::EngineRunTimeTooLow,
            0x86 => Self::TemperatureTooHigh,
            0x87 => Self::TemperatureTooLow,
            0x88 => Self::VehicleSpeedTooHigh,
            0x89 => Self::VehicleSpeedTooLow,
            0x8A => Self::ThrottlePedalTooHigh,
            0x8B => Self::ThrottlePedalTooLow,
            0x8C => Self::TransmissionRangeNotInNeutral,
            0x8D => Self::TransmissionRangeNotInGear,
            0x8F => Self::BrakeSwitchNotClosed,
            0x90 => Self::ShifterLeverNotInPark,
            0x91 => Self::TorqueConverterClutchLocked,
            0x92 => Self::VoltageTooHigh,
            0x93 => Self::VoltageTooLow,
            0x94 => Self::ResourceTemporarilyNotAvailable,
            0xF0..=0xFE => Self::VehicleManufacturerSpecific(v),
            _ => Self::Reserved(v),
        }
    }
}

impl From<Code> for u8 {
    fn from(val: Code) -> Self {
        match val {
            Code::Positive => 0x00,

            Code::GeneralReject => 0x10,
            Code::ServiceNotSupported => 0x11,
            Code::SubFunctionNotSupported => 0x12,
            Code::IncorrectMessageLengthOrInvalidFormat => 0x13,
            Code::ResponseTooLong => 0x14,

            Code::BusyRepeatRequest => 0x21,
            Code::ConditionsNotCorrect => 0x22,

            Code::RequestSequenceError => 0x24,
            Code::NoResponseFromSubnetComponent => 0x25,
            Code::FailurePreventsExecutionOfRequestedAction => 0x26,

            Code::RequestOutOfRange => 0x31,

            Code::SecurityAccessDenied => 0x33,
            Code::AuthenticationRequired => 0x34,
            Code::InvalidKey => 0x35,
            Code::ExceedNumberOfAttempts => 0x36,
            Code::RequiredTimeDelayNotExpired => 0x37,
            Code::SecureDataTransmissionRequired => 0x38,
            Code::SecureDataTransmissionNotAllowed => 0x39,
            Code::SecureDataVerificationFailed => 0x3A,

            Code::CertificateVerificationFailedInvalidTimePeriod => 0x50,
            Code::CertificateVerificationFailedInvalidSignature => 0x51,
            Code::CertificateVerificationFailedInvalidChainOfTrust => 0x52,
            Code::CertificateVerificationFailedInvalidType => 0x53,
            Code::CertificateVerificationFailedInvalidFormat => 0x54,
            Code::CertificateVerificationFailedInvalidContent => 0x55,
            Code::CertificateVerificationFailedInvalidScope => 0x56,
            Code::CertificateVerificationFailedInvalidCertificate => 0x57,
            Code::OwnershipVerificationFailed => 0x58,
            Code::ChallengeCalculationFailed => 0x59,
            Code::SettingAccessRightsFailed => 0x5A,
            Code::SessionKeyCreationDerivationFailed => 0x5B,
            Code::ConfigurationDataUsageFailed => 0x5C,
            Code::DeAuthenticationFailed => 0x5D,

            Code::UploadDownloadNotAccepted => 0x70,
            Code::TransferDataSuspended => 0x71,
            Code::GeneralProgrammingFailure => 0x72,
            Code::WrongBlockSequenceCounter => 0x73,

            Code::RequestCorrectlyReceivedResponsePending => 0x78,

            Code::SubFunctionNotSupportedInActiveSession => 0x7E,
            Code::ServiceNotSupportedInActiveSession => 0x7F,

            Code::RpmTooHigh => 0x81,
            Code::RpmTooLow => 0x82,
            Code::EngineIsRunning => 0x83,
            Code::EngineIsNotRunning => 0x84,
            Code::EngineRunTimeTooLow => 0x85,
            Code::TemperatureTooHigh => 0x86,
            Code::TemperatureTooLow => 0x87,
            Code::VehicleSpeedTooHigh => 0x88,
            Code::VehicleSpeedTooLow => 0x89,
            Code::ThrottlePedalTooHigh => 0x8A,
            Code::ThrottlePedalTooLow => 0x8B,
            Code::TransmissionRangeNotInNeutral => 0x8C,
            Code::TransmissionRangeNotInGear => 0x8D,
            Code::BrakeSwitchNotClosed => 0x8F,
            Code::ShifterLeverNotInPark => 0x90,
            Code::TorqueConverterClutchLocked => 0x91,
            Code::VoltageTooHigh => 0x92,
            Code::VoltageTooLow => 0x93,
            Code::ResourceTemporarilyNotAvailable => 0x94,
            Code::VehicleManufacturerSpecific(v) => v,
            Code::Reserved(v) => v,
        }
    }
}
