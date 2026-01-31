use crate::error::Error;

use uefi::runtime::{ResetType, VariableVendor, VariableAttributes};
use uefi::{println, runtime, cstr16, Status};


const EFI_OS_INDICATIONS_BOOT_TO_FW_UI: u64 = 0x0000000000000001;


#[derive(Debug, PartialEq)]
pub enum Reset {
    Cold,
    Warm,
    Shutdown,
    Firmware,
}

impl TryFrom<&str> for Reset {
    type Error = Error;

    fn try_from(from: &str) -> Result<Reset, Error> {
        let reset = from.to_ascii_lowercase();

        match reset.as_str() {
            "cold" => Ok(Reset::Cold),
            "warm" => Ok(Reset::Warm),
            "shutdown" => Ok(Reset::Shutdown),
            "firmware" => Ok(Reset::Firmware),
            &_ => Err(Error::InvalidResetType(reset)),
        }
    }
}

impl Reset {
    fn reset_type(&self) -> ResetType {
        match self {
            Reset::Cold | Reset::Firmware => ResetType::COLD,
            Reset::Warm => ResetType::WARM,
            Reset::Shutdown => ResetType::SHUTDOWN,
        }
    }

    pub fn reset(&self) -> Result<(), Error> {
        if *self == Reset::Firmware {
            println!("info: set variable 'OsIndications': 0x0000000000000001");

            let attributes = VariableAttributes::NON_VOLATILE | VariableAttributes::BOOTSERVICE_ACCESS | VariableAttributes::RUNTIME_ACCESS;

            runtime::set_variable(cstr16!("OsIndications"), &VariableVendor::GLOBAL_VARIABLE, attributes, &EFI_OS_INDICATIONS_BOOT_TO_FW_UI.to_le_bytes())?;
        }

        println!("info: reset: {:#x?}", self.reset_type().0);

        runtime::reset(self.reset_type(), Status::SUCCESS, None);
    }
}


