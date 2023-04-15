//! Wrapper for IOCTL requests

use crate::{dynamic_diag::DynamicDiagSession, DiagServerResult};
use auto_uds::kwp2k::{KwpCommand, KwpSessionType};

/// Handler for Input output control by local identifier requests (IOCTL)
/// This allows for short term or long term actuation's of components an ECU controls,
/// or reporting a components current state.
///
/// USE WITH CAUTION!
#[derive(Debug)]
pub struct KwpIOCTLManager<'a> {
    server: &'a mut DynamicDiagSession,
    identifier: u8,
}

impl<'a> KwpIOCTLManager<'a> {
    /// Creates an IOCTL manager
    ///
    /// ## Parameters
    /// * identifier - A identifier for the component or function to control. Valid ranges are
    ///     * 0x10-0xF9 - Input output local Identifier
    ///     * 0xFA-0xFE - System supplier specific
    ///     * 0xFF - Input output local identifier
    /// Other values may result in an ECU rejecting the request.
    /// * server - KWP2000 server reference
    pub fn new(identifier: u8, server: &'a mut DynamicDiagSession) -> DiagServerResult<Self> {
        // We need to be in extended mode for this SID to work, so try now
        server.kwp_set_session(KwpSessionType::ExtendedDiagnostics.into())?;
        Ok(Self { identifier, server })
    }

    /// Asks the ECU to take back control of the identifier.
    pub fn return_control_to_ecu(&mut self) -> DiagServerResult<()> {
        self.server.send_command_with_response(
            KwpCommand::InputOutputControlByLocalIdentifier,
            &[self.identifier, 0x00],
        )?;
        Ok(())
    }

    /// Asks the ECU to report the current state of the identifier.
    pub fn report_current_state(&mut self) -> DiagServerResult<Vec<u8>> {
        self.server.send_command_with_response(
            KwpCommand::InputOutputControlByLocalIdentifier,
            &[self.identifier, 0x01],
        )
    }

    /// Asks the ECU to return the component identifier back to its default (Factory) state
    pub fn reset_to_default_state(&mut self) -> DiagServerResult<()> {
        self.server.send_command_with_response(
            KwpCommand::InputOutputControlByLocalIdentifier,
            &[self.identifier, 0x04],
        )?;
        Ok(())
    }

    /// Asks the ECU to freeze the current state of the identifier
    pub fn freeze_current_state(&mut self) -> DiagServerResult<()> {
        self.server.send_command_with_response(
            KwpCommand::InputOutputControlByLocalIdentifier,
            &[self.identifier, 0x05],
        )?;
        Ok(())
    }

    /// Actuates the component at the provided identifier. This is a short term actuation.
    /// Once the ECU looses power or returns to its default session state, the component will
    /// be controlled by the ECU normally
    pub fn short_term_actuate(&mut self, args: &[u8]) -> DiagServerResult<()> {
        let mut a = vec![self.identifier, 0x07];
        a.extend_from_slice(args);
        self.server
            .send_command_with_response(KwpCommand::InputOutputControlByLocalIdentifier, &a)?;
        Ok(())
    }

    /// Adjusts the component's value. This is an optional command and is NOT supported by all ECUs.
    /// This allows for long-term adjustments (Such as fuel trims) to be made to the ECU. The ECU
    /// will retain the values even after a power reset.
    pub fn long_term_adjust(&mut self, args: &[u8]) -> DiagServerResult<()> {
        let mut a = vec![self.identifier, 0x08];
        a.extend_from_slice(args);
        self.server
            .send_command_with_response(KwpCommand::InputOutputControlByLocalIdentifier, &a)?;
        Ok(())
    }
}
