use iso13400_2::{response::*, *};

#[test]
fn test_header_negative() -> anyhow::Result<()> {
    let source = hex::decode(
        "02FD0000\
    00000001\
    00",
    )?;

    let payload = HeaderNegative::new(HeaderNegativeCode::IncorrectPatternFormat);
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::RespHeaderNegative(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_vehicle_id() -> anyhow::Result<()> {
    let source = hex::decode(
        "02FD0004\
    00000021\
    2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D\
    0E00\
    001100110011\
    110011001100\
    1000",
    )?;

    let payload = VehicleID::new(
        "-".repeat(17),
        LogicAddress::from(0x0E00),
        Eid::new(0x001100110011)?,
        Gid::new(0x110011001100)?,
        FurtherAction::CentralSecurity,
        Some(SyncStatus::VINorGIDSync),
    )?;
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::RespVehicleId(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_entity_status() -> anyhow::Result<()> {
    let source = hex::decode(
        "02FD4002\
    00000007\
    00FF0155AA55AA",
    )?;
    let payload = EntityStatus::new(NodeType::Gateway, 0xFF, 0x01, Some(0x55aa55aa));

    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::RespEntityStatus(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_diag_power_mode() -> anyhow::Result<()> {
    let source = hex::decode(
        "02FD4004\
    00000001\
    01",
    )?;

    let payload = DiagnosticPowerMode::new(PowerMode::Ready);
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::RespDiagPowerMode(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_routing_activation() -> anyhow::Result<()> {
    let source = hex::decode(
        "02FD0006\
    0000000D\
    0E00\
    0DFF\
    10\
    00000000\
    00000000",
    )?;

    let payload = RoutingActive::new(
        LogicAddress::from(0x0E00),
        LogicAddress::from(0x0DFF),
        ActiveCode::Success,
        Some(0x00),
    );
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::RespRoutingActive(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_alive_check() -> anyhow::Result<()> {
    let source = hex::decode(
        "02FD0008\
    00000002\
    0E00",
    )?;

    let payload = AliveCheck::new(LogicAddress::from(0x0E00));
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::RespAliveCheck(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_diag_positive() -> anyhow::Result<()> {
    let source = hex::decode(
        "02FD8002\
    00000008\
    0E00\
    0DFF\
    00\
    021001",
    )?;
    let payload = DiagnosticPositive::new(
        LogicAddress::from(0x0E00),
        LogicAddress::from(0x0DFF),
        DiagnosticPositiveCode::Confirm,
        vec![0x02, 0x10, 0x01],
    );
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::RespDiagPositive(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_diag_negative() -> anyhow::Result<()> {
    let source = hex::decode(
        "02FD8003\
    00000008\
    0E00\
    0DFF\
    05\
    021001",
    )?;
    let payload = DiagnosticNegative::new(
        LogicAddress::from(0x0E00),
        LogicAddress::from(0x0DFF),
        DiagnosticNegativeCode::OutOfMemory,
        vec![0x02, 0x10, 0x01],
    );
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::RespDiagNegative(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}
