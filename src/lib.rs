#[cxx::bridge]
pub mod ffi {
    enum ETrackedPropertyError {
        TrackedProp_Success = 0,
        TrackedProp_WrongDataType = 1,
        TrackedProp_WrongDeviceClass = 2,
        TrackedProp_BufferTooSmall = 3,
        TrackedProp_UnknownProperty = 4, // Driver has not set the property (and may not ever).
        TrackedProp_InvalidDevice = 5,
        TrackedProp_CouldNotContactServer = 6,
        TrackedProp_ValueNotProvidedByDevice = 7,
        TrackedProp_StringExceedsMaximumLength = 8,
        TrackedProp_NotYetAvailable = 9, // The property value isn't known yet, but is expected soon. Call again later.
        TrackedProp_PermissionDenied = 10,
        TrackedProp_InvalidOperation = 11,
        TrackedProp_CannotWriteToWildcards = 12,
        TrackedProp_IPCReadFailure = 13,
        TrackedProp_OutOfMemory = 14,
        TrackedProp_InvalidContainer = 15,
    }

    enum EVRSettingsError {
        VRSettingsError_None = 0,
        VRSettingsError_IPCFailed = 1,
        VRSettingsError_WriteFailed = 2,
        VRSettingsError_ReadFailed = 3,
        VRSettingsError_JsonParseFailed = 4,
        VRSettingsError_UnsetSettingHasNoDefault = 5, // This will be returned if the setting does not appear in the appropriate default file and has not been set
        VRSettingsError_AccessDenied = 6,
    }

    unsafe extern "C++" {
        include!("openvr/openvr_driver.h");

        type ETrackedPropertyError;

        #[namespace = "vr"]
        type IVRSettings;

        #[namespace = "vr"]
        type EVRSettingsError;

        fn GetSettingsErrorNameFromEnum(self: &IVRSettings, eError: EVRSettingsError) -> *const i8;
    }
}
