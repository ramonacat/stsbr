use crate::data_source::{DataSource, DataSourceState, BlockError};
use nix::sys::socket::{AddressFamily, SockAddr};

pub struct NetworkInterface {}

impl DataSource for NetworkInterface {
    fn current_state(&self) -> Result<DataSourceState, BlockError> {
        let mut addrs = nix::ifaddrs::getifaddrs().map_err(|_| BlockError::new("Failed to get interface addresses".to_string()))?;
        let iface = addrs.find(|a| a.interface_name == "eno1" && a.address.map_or(false, |a| a.family() == AddressFamily::Inet));

        match iface {
            Some(i) => {
                match i.address {
                    Some(SockAddr::Inet(address)) => Ok(DataSourceState::new(address.ip().to_string())),
                    Some(_) => Err(BlockError::new("Wrong address type".to_string())),
                    None => Err(BlockError::new("No address".to_string()))
                }
            },
            None => Err(BlockError::new("Failed to find interface".to_string()))
        }
    }
}

impl NetworkInterface {
    pub fn new() -> Self {
        NetworkInterface {}
    }
}