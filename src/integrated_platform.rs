use std::ffi::c_void;

use eos_sys::{
    EOS_EIntegratedPlatformManagementFlags, EOS_IntegratedPlatform_Options,
    EOS_IntegratedPlatform_Steam_Options, EOS_INTEGRATEDPLATFORM_STEAM_OPTIONS_API_LATEST,
};

pub struct EosIntegratedPlatformOptions {
    platform_type: String,
    flags: EosIntegratedPlatformManagementFlag,
    steam_options: Option<EosIntegratedPlatformSteamOptions>,
}

impl Into<EOS_IntegratedPlatform_Options> for EosIntegratedPlatformOptions {
    fn into(self) -> EOS_IntegratedPlatform_Options {
        let steam_options: *const EOS_IntegratedPlatform_Steam_Options;
        if let Some(optional_steam_options) = self.steam_options {
            steam_options =
                &Into::<EOS_IntegratedPlatform_Steam_Options>::into(optional_steam_options)
                    as *const _;
        } else {
            steam_options = std::ptr::null()
        };
        EOS_IntegratedPlatform_Options {
            ApiVersion: EOS_INTEGRATEDPLATFORM_STEAM_OPTIONS_API_LATEST as i32,
            Type: self.platform_type.as_ptr() as *const i8,
            Flags: self.flags.into(),
            InitOptions: &steam_options as *const _ as *const c_void,
        }
    }
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EosIntegratedPlatformManagementFlag {
    /// The integrated platform library should be disabled. This is equivalent to providing no flags.
    Disabled = 1,
    /// The integrated platform library is managed by the calling application. EOS SDK should only hook into an existing instance of the integrated platform library.
    LibraryManagedByApplication = 2,
    /// EOS SDK should fully manage the integrated platform library. It will do this by performing the load, initialize, tick and unload operations as necessary.
    LibraryManagedBySDK = 4,
    /// The EOS SDK should not mirror the EOS rich presence with the Integrated Platform.
    /// The default behavior is for EOS SDK to share local presence with the Integrated Platform.
    DisablePresenceMirroring = 8,
    /// EOS SDK should not perform any sessions management through the Integrated Platform.
    /// The default behavior is for EOS SDK to perform sessions management through the Integrated Platform.
    /// Sessions management includes:
    ///    - sharing the lobby and session presence enabled games with the Integrated Platform.
    ///    - handling Social Overlay join button events which cannot be handled by normal processing of Epic Services.
    ///    - handling Social Overlay invite button events which cannot be handled by normal processing of Epic Services.
    ///    - handling startup requests from the Integrated Platform to immediately join a game due to in invite while offline.
    /// @see EOS_Lobby_AddNotifySendLobbyNativeInviteRequested
    DisableSDKManagedSessions = 16,
    /// Some features within the EOS SDK may wish to know a preference of Integrated Platform versus EOS.
    /// When determining an absolute platform preference those with this flag will be skipped.
    /// The IntegratedPlatforms list is provided via the EOS_Platform_Options during EOS_Platform_Create.
    /// The primary usage of the EOS_IPMF_PreferEOSIdentity and EOS_IPMF_PreferIntegratedIdentity flags is with game invites from the Social Overlay.
    /// For game invites from the Social Overlay the EOS SDK will follow these rules:
    ///     - If the only account ID we can determine for the target player is an EAS ID then the EOS system will be used.
    ///     - If the only account ID we can determine for the target player is an integrated platform ID then the integrated platform system will be used.
    ///     - If both are available then the EOS SDK will operate in 1 of 3 modes:
    ///         - no preference identified: use both the EOS and integrated platform systems.
    ///         - PreferEOS: Use EOS if the target is an EAS friend and is either online in EAS or not online for the integrated platform.
    ///         - PreferIntegrated: Use integrated platform if the target is an integrated platform friend and is either online in the integrated platform or not online for EAS.     
    ///     - If the integrated platform fails to send then try EAS if was not already used.
    PreferEOSIdentity = 32,
    /// Some features within the EOS SDK may wish to know a preference of Integrated Platform versus EOS.
    /// For further explanation see EOS_IPMF_PreferEOSIdentity.
    /// @see EOS_IPMF_PreferEOSIdentity
    PreferIntegratedIdentity = 64,
    /// By default the EOS SDK will attempt to detect the login/logout events of local users and update local states accordingly.
    /// Setting this flag will disable this functionality, relying on the application to process login/logout events and notify EOS SDK.
    /// It is not possible for the EOS SDK to do this on all platforms, making this flag not always optional.
    /// This flag must be set to use the manual platform user login/logout functions, even on platforms where it is not possible for the EOS SDK to detect login/logout events, making this a required flag for correct Integrated Platform behavior on those platforms.
    ApplicationManagedIdentityLogin = 128,
}

impl Into<EOS_EIntegratedPlatformManagementFlags> for EosIntegratedPlatformManagementFlag {
    fn into(self) -> EOS_EIntegratedPlatformManagementFlags {
        match self {
            EosIntegratedPlatformManagementFlag::Disabled => {
                EOS_EIntegratedPlatformManagementFlags::EOS_IPMF_Disabled
            }
            EosIntegratedPlatformManagementFlag::LibraryManagedByApplication => {
                EOS_EIntegratedPlatformManagementFlags::EOS_IPMF_LibraryManagedByApplication
            }
            EosIntegratedPlatformManagementFlag::LibraryManagedBySDK => {
                EOS_EIntegratedPlatformManagementFlags::EOS_IPMF_LibraryManagedBySDK
            }
            EosIntegratedPlatformManagementFlag::DisablePresenceMirroring => {
                EOS_EIntegratedPlatformManagementFlags::EOS_IPMF_DisablePresenceMirroring
            }
            EosIntegratedPlatformManagementFlag::DisableSDKManagedSessions => {
                EOS_EIntegratedPlatformManagementFlags::EOS_IPMF_DisableSDKManagedSessions
            }
            EosIntegratedPlatformManagementFlag::PreferEOSIdentity => {
                EOS_EIntegratedPlatformManagementFlags::EOS_IPMF_PreferEOSIdentity
            }
            EosIntegratedPlatformManagementFlag::PreferIntegratedIdentity => {
                EOS_EIntegratedPlatformManagementFlags::EOS_IPMF_PreferIntegratedIdentity
            }
            EosIntegratedPlatformManagementFlag::ApplicationManagedIdentityLogin => {
                EOS_EIntegratedPlatformManagementFlags::EOS_IPMF_ApplicationManagedIdentityLogin
            }
        }
    }
}

pub struct EosIntegratedPlatformSteamOptions {
    /// Usage of this parameter is dependent on the specified EOS_EIntegratedPlatformManagementFlags.
    /// Optional with EOS_IPMF_LibraryManagedByApplication.
    /// Set to override the loaded library basename, or use None to assume the default basename by platform:
    /// - Linux: libsteam_api.so,
    /// - macOS: libsteam_api.dylib,
    /// - Windows 32-bit: steam_api.dll,
    /// - Windows 64-bit: steam_api64.dll.
    /// Required with EOS_IPMF_LibraryManagedBySDK.
    /// Set to a fully qualified file path to the Steamworks SDK runtime library on disk.
    pub override_library_path: Option<String>,
    /// Used to specify the major version of the Steam SDK your game is compiled against, e.g.: steam_major_version = 1;
    pub steam_major_version: u32,
    // Used to specify the minor version of the Steam SDK your game is compiled against, e.g.: steam_minor_version = 57;
    pub steam_minor_version: u32,
}

impl Into<EOS_IntegratedPlatform_Steam_Options> for EosIntegratedPlatformSteamOptions {
    fn into(self) -> EOS_IntegratedPlatform_Steam_Options {
        EOS_IntegratedPlatform_Steam_Options {
            ApiVersion: EOS_INTEGRATEDPLATFORM_STEAM_OPTIONS_API_LATEST as i32,
            OverrideLibraryPath: self
                .override_library_path
                .map(|s| s.as_ptr() as *const i8)
                .unwrap_or(std::ptr::null()),
            SteamMajorVersion: self.steam_major_version,
            SteamMinorVersion: self.steam_minor_version,
        }
    }
}
