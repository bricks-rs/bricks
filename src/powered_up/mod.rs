pub mod scan;

//todo replace the below with references to lego_powered_up crate once it
// has been rewritten

#[allow(unused)]
pub mod bleservice {
    use lazy_static::lazy_static;
    use uuid::Uuid;

    pub const WEDO2_SMART_HUB_2: &str = "00004f0e-1212-efde-1523-785feabcd123";
    pub const WEDO2_SMART_HUB_3: &str = "2a19";
    pub const WEDO2_SMART_HUB_4: &str = "180f";
    pub const WEDO2_SMART_HUB_5: &str = "180a";
    lazy_static! {
        pub static ref WEDO2_SMART_HUB: Uuid =
            Uuid::parse_str("00001523-1212-efde-1523-785feabcd123").unwrap();
        pub static ref LPF2_HUB: Uuid =
            Uuid::parse_str("00001623-1212-efde-1623-785feabcd123").unwrap();
    }
}

#[allow(unused)]
pub mod blecharacteristic {
    use lazy_static::lazy_static;
    use uuid::Uuid;

    pub const WEDO2_BATTERY: &str = "2a19";
    pub const WEDO2_FIRMWARE_REVISION: &str = "2a26";
    pub const WEDO2_BUTTON: &str = "00001526-1212-efde-1523-785feabcd123"; // "1526"
    pub const WEDO2_PORT_TYPE: &str = "00001527-1212-efde-1523-785feabcd123"; // "1527" // Handles plugging and unplugging of devices on WeDo 2.0 Smart Hub
    pub const WEDO2_LOW_VOLTAGE_ALERT: &str =
        "00001528-1212-efde-1523-785feabcd123"; // "1528"
    pub const WEDO2_HIGH_CURRENT_ALERT: &str =
        "00001529-1212-efde-1523-785feabcd123"; // "1529"
    pub const WEDO2_LOW_SIGNAL_ALERT: &str =
        "0000152a-1212-efde-1523-785feabcd123"; // "152a",
    pub const WEDO2_DISCONNECT: &str = "0000152b-1212-efde-1523-785feabcd123"; // "152b"
    pub const WEDO2_SENSOR_VALUE: &str = "00001560-1212-efde-1523-785feabcd123"; // "1560"
    pub const WEDO2_VALUE_FORMAT: &str = "00001561-1212-efde-1523-785feabcd123"; // "1561"
    pub const WEDO2_PORT_TYPE_WRITE: &str =
        "00001563-1212-efde-1523-785feabcd123"; // "1563"
    pub const WEDO2_MOTOR_VALUE_WRITE: &str =
        "00001565-1212-efde-1523-785feabcd123"; // "1565"
    pub const WEDO2_NAME_ID: &str = "00001524-1212-efde-1523-785feabcd123"; // "1524"
    lazy_static! {
        pub static ref LPF2_ALL: Uuid =
            Uuid::parse_str("00001624-1212-efde-1623-785feabcd123").unwrap();
    }
}
