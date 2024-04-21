use once_cell::sync::Lazy;
use rand::Rng;
use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacAddress {
    bytes: [u8; 6],
}

impl MacAddress {
    pub fn new(address: String) -> Result<Self, String> {
        static MAC_ADDRESS_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$")
                .expect("Failed to compile regex")
        });

        if MAC_ADDRESS_REGEX.is_match(&address) {
            let bytes: Result<Vec<u8>, _> = address
                .split(|c| c == ':' || c == '-')
                .map(|byte| u8::from_str_radix(byte, 16))
                .collect();
            match bytes {
                Ok(bytes) => Ok(Self {
                    bytes: [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]],
                }),
                Err(_) => Err("Invalid MAC address format".to_string()),
            }
        } else {
            Err("Invalid MAC address format".to_string())
        }
    }

    pub fn random() -> Self {
        let fixed_oid = "00:11:22";
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<String> =
            (0..3).map(|_| format!("{:02X}", rng.gen::<u8>())).collect();
        let mac_address = format!("{}:{}", fixed_oid, random_bytes.join(":"));
        Self::new(mac_address).unwrap()
    }

    pub fn is_broadcast(&self) -> bool {
        self.bytes.iter().all(|&byte| byte == 0xFF)
    }

    pub fn broadcast() -> Self {
        Self {
            bytes: [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
        }
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        self.bytes
    }
}
