extern crate openvr_sys;
use openvr_sys::Enum_EVRInitError::*;
use openvr_sys::Enum_EVRApplicationType::*;

pub mod common;
pub mod error;
pub mod tracking;
pub mod system;
pub mod extended_display;
pub mod compositor;
pub mod render_models;
pub mod subsystems;

pub use system::IVRSystem;
pub use extended_display::IVRExtendedDisplay;
pub use compositor::IVRCompositor;
pub use render_models::IVRRenderModels;

pub use subsystems::*;
pub use error::*;

pub use common::Eye;

/// Inits the open vr interface and returns the system
pub fn init() ->  Result<system::IVRSystem, Error<openvr_sys::Enum_EVRInitError>> {
    let mut err = EVRInitError_VRInitError_None;
    let app_type = EVRApplicationType_VRApplication_Scene;

    // try to initialize base vr eco
    unsafe {
        openvr_sys::VR_InitInternal(&mut err, app_type);
    };

    // check for errors
    match err {
        EVRInitError_VRInitError_None => {
            // get system
            let result = system();
            match result {
                Ok(sys) => {
                    return Ok(sys);
                },
                Err(err) => {
                    return Err(err);
                }
            }
        },
        _ => {
            return Err(Error::from_raw(err));
        }
    };
}

/// Shutdowns all openvr related systems
pub fn shutdown() {
    unsafe {
        openvr_sys::VR_ShutdownInternal();
    }
}
