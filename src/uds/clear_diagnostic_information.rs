//!  Provides methods to clear diagnostic trouble codes from the ECU

use automotive_diag::uds::UdsCommand;

use crate::{dynamic_diag::DynamicDiagSession, DiagServerResult};

impl DynamicDiagSession {
    /// Clears diagnostic information (DTCs) from the ECU.
    ///
    /// ## Parameters
    /// * server - The UDS Diagnostic server
    /// * dtc_mask - Mask of DTCs to clear. Only the lower 3 bytes are used (from 0x00000000 - 0x00FFFFFF)
    pub fn uds_clear_diagnostic_information(&self, dtc_mask: u32) -> DiagServerResult<()> {
        self.send_command_with_response(
            UdsCommand::ClearDiagnosticInformation,
            &[
                (dtc_mask >> 16) as u8,
                (dtc_mask >> 8) as u8,
                dtc_mask as u8,
            ],
        )?;
        Ok(())
    }
}
