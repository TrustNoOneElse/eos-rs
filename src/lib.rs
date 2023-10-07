use bitflags::bitflags;
use eos_sys::{
    EOS_ERTCBackgroundMode,
    EOS_HIntegratedPlatformOptionsContainer, EOS_HPlatform, EOS_Initialize, EOS_InitializeOptions,
    EOS_IntegratedPlatformOptionsContainer_Release,
    EOS_IntegratedPlatform_CreateIntegratedPlatformOptionsContainer,
    EOS_IntegratedPlatform_CreateIntegratedPlatformOptionsContainerOptions,
    EOS_Platform_ClientCredentials, EOS_Platform_Create, EOS_Platform_Options,
    EOS_Platform_RTCOptions,
    EOS_INITIALIZE_API_LATEST,
    EOS_INTEGRATEDPLATFORM_CREATEINTEGRATEDPLATFORMOPTIONSCONTAINER_API_LATEST,
    EOS_PF_CONSOLE_ENABLE_OVERLAY_AUTOMATIC_UNLOADING, EOS_PF_DISABLE_OVERLAY,
    EOS_PF_DISABLE_SOCIAL_OVERLAY, EOS_PF_LOADING_IN_EDITOR, EOS_PF_RESERVED1,
    EOS_PF_WINDOWS_ENABLE_OVERLAY_D3D10, EOS_PF_WINDOWS_ENABLE_OVERLAY_D3D9,
    EOS_PF_WINDOWS_ENABLE_OVERLAY_OPENGL, EOS_PLATFORM_RTCOPTIONS_API_LATEST, EOS_Platform_Release,
};
use error::EosError;
use std::{path::PathBuf, sync::Arc};

mod error;
#[cfg(feature = "integrated_platform")]
mod integrated_platform;

type Result<T> = std::result::Result<T, EosError>;

static LATEST_API_VERSION: i32 = EOS_INITIALIZE_API_LATEST as i32;

// EOS is not considered yet thead-safe
// see https://dev.epicgames.com/docs/epic-online-services/platforms/guidelines-and-references
#[derive(Clone)]
pub struct EosPlatform {
    inner: Arc<Inner>,
}

struct Inner {
    handle: EOS_HPlatform,
}

impl EosPlatform {
    pub fn init(
        init_options: EosInitializeOptions,
        platform_options: EosPlatformOptions,
    ) -> Result<Self> {
        let init_successful: EosError = unsafe { EOS_Initialize(&init_options.into()) }.into();
        if !init_successful.was_successful() {
            return Err(init_successful);
        }

        let (handle, result): (_, EosError) = unsafe {
            let options = EOS_IntegratedPlatform_CreateIntegratedPlatformOptionsContainerOptions {
                ApiVersion:
                    EOS_INTEGRATEDPLATFORM_CREATEINTEGRATEDPLATFORMOPTIONSCONTAINER_API_LATEST
                        as i32,
            };
            let handle: EOS_HIntegratedPlatformOptionsContainer = std::mem::zeroed();
            let result = EOS_IntegratedPlatform_CreateIntegratedPlatformOptionsContainer(
                &options as *const _,
                &handle as *const _ as *mut _,
            );
            (handle, result.into())
        };

        if !result.was_successful() {
            return Err(result);
        }

        let platform_handle = unsafe {
            let mut platform_options: eos_sys::_tagEOS_Platform_Options = platform_options.into();
            platform_options.IntegratedPlatformOptionsContainerHandle = handle;
            let result = EOS_Platform_Create(&platform_options);
            EOS_IntegratedPlatformOptionsContainer_Release(handle);
            result
        };

        Ok(Self {
            inner: Arc::new(Inner {
                handle: platform_handle,
            }),
        })
    }
}

impl Drop for EosPlatform {
    fn drop(&mut self) {
        unsafe {
            EOS_Platform_Release(self.inner.handle);
        }
    }
}

pub enum EosPlatformRtcBackgroundMode {
    /// Upon entering a background application status, all logged in users leave any RTC rooms. All subsequent attemps to join any RTC rooms will be rejected.
    /// Upon returning to a foreground application status, all subsequent attemps to join any RTC rooms will be allowed.
    LeaveRooms = 0,
    /// Application status has no effect on RTC rooms. Audio is captured from input devices and is played to output devices.
    /// Games should obtain consent from users and otherwise make users aware this is occurring.
    KeepRoomsAlive = 1,
}

impl Into<EOS_ERTCBackgroundMode> for EosPlatformRtcBackgroundMode {
    fn into(self) -> EOS_ERTCBackgroundMode {
        match self {
            EosPlatformRtcBackgroundMode::LeaveRooms => {
                EOS_ERTCBackgroundMode::EOS_RTCBM_LeaveRooms
            }
            EosPlatformRtcBackgroundMode::KeepRoomsAlive => {
                EOS_ERTCBackgroundMode::EOS_RTCBM_KeepRoomsAlive
            }
        }
    }
}

pub struct EeoPlatformClientCredentials {
    client_id: String,
    client_secret: String,
}

impl Into<EOS_Platform_ClientCredentials> for EeoPlatformClientCredentials {
    fn into(self) -> EOS_Platform_ClientCredentials {
        EOS_Platform_ClientCredentials {
            ClientId: self.client_id.as_ptr() as *const i8,
            ClientSecret: self.client_secret.as_ptr() as *const i8,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EosPlatformFlags(u64);

bitflags! {
    impl EosPlatformFlags: u64 {
        const LOADING_IN_EDITOR = EOS_PF_LOADING_IN_EDITOR as u64;
        const DISABLE_OVERLAY = EOS_PF_DISABLE_OVERLAY as u64;
        const DISABLE_SOCIAL_OVERLAY = EOS_PF_DISABLE_SOCIAL_OVERLAY as u64;
        const RESERVED1 = EOS_PF_RESERVED1 as u64;
        const WINDOWS_ENABLE_OVERLAY_D3D9 = EOS_PF_WINDOWS_ENABLE_OVERLAY_D3D9 as u64;
        const WINDOWS_ENABLE_OVERLAY_D3D10 = EOS_PF_WINDOWS_ENABLE_OVERLAY_D3D10 as u64;
        const WINDOWS_ENABLE_OVERLAY_OPENGL = EOS_PF_WINDOWS_ENABLE_OVERLAY_OPENGL as u64;
        const CONSOLE_ENABLE_OVERLAY_AUTOMATIC_UNLOADING = EOS_PF_CONSOLE_ENABLE_OVERLAY_AUTOMATIC_UNLOADING as u64;
    }
}

pub struct EosPlatformOptions {
    /// Set this to false if the application is running as a client with a local user, otherwise set to true (e.g. for a dedicated game server)
    pub is_server: bool,
    /// The product ID for the running application, found on the dev portal. Max length is 64.
    pub product_id: String,
    /// The sandbox ID for the running application, found on the dev portal. Max length is 64.
    pub sandbox_id: String,
    /// The deployment ID for the running application, found on the dev portal. Max length is 64.
    pub deployment_id: String,
    /// Set of service permissions associated with the running application
    pub client_credentials: EeoPlatformClientCredentials,
    /// Platform creation flags, e.g. EosPlatformFlags::EOS_PF_LOADING_IN_EDITOR. This is a bitwise-or union of the defined flags.
    pub flags: EosPlatformFlags,
    /// Used by Player Data Storage and Title Storage. Must be None initialized if unused. 256-bit Encryption Key for file encryption in hexadecimal format; 64 hex chars
    pub encryption_key: Option<String>,
    /// The override country code to use for the logged in user. Max length is 4.
    /// If not specified, the country code will be determined automatically.
    pub override_country_code: Option<String>,
    /// The override locale code to use for the logged in user. This follows ISO 639. Max length is 9.
    /// If not specified, the locale code will be determined automatically.
    pub override_locale_code: Option<String>,
    /// Used by Player Data Storage and Title Storage. Must be None initialized if unused. Cache directory path. Absolute path to the folder that is going to be used for caching temporary data. The path is created if it's missing.
    /// The Path must conatain UTF-8 valid characters only.
    pub cache_directory: Option<PathBuf>,
    /// A budget, measured in milliseconds, for EOS_Platform_Tick to do its work. When the budget is met or exceeded (or if no work is available), EOS_Platform_Tick will return.
    /// This allows your game to amortize the cost of SDK work across multiple frames in the event that a lot of work is queued for processing.
    /// Zero is interpreted as \"perform all available work\".
    pub tick_budget_in_milliseconds: u32,
    /// RTC options. Setting to None will disable RTC features (e.g. voice)
    pub rtc_options: Option<EosPlatformRtcBackgroundMode>,
}

impl Into<EOS_Platform_Options> for EosPlatformOptions {
    fn into(self) -> EOS_Platform_Options {
        EOS_Platform_Options {
            ApiVersion: LATEST_API_VERSION,
            bIsServer: self.is_server as i32,
            ProductId: self.product_id.as_ptr() as *const i8,
            SandboxId: self.sandbox_id.as_ptr() as *const i8,
            DeploymentId: self.deployment_id.as_ptr() as *const i8,
            ClientCredentials: self.client_credentials.into(),
            Flags: self.flags.bits(),
            EncryptionKey: match self.encryption_key {
                Some(encryption_key) => encryption_key.as_ptr() as *const i8,
                None => std::ptr::null(),
            },
            OverrideCountryCode: match self.override_country_code {
                Some(override_country_code) => override_country_code.as_ptr() as *const i8,
                None => std::ptr::null(),
            },
            OverrideLocaleCode: match self.override_locale_code {
                Some(override_locale_code) => override_locale_code.as_ptr() as *const i8,
                None => std::ptr::null(),
            },
            CacheDirectory: match self.cache_directory {
                Some(cache_directory) => cache_directory.as_path().to_str().unwrap().as_ptr() as *const i8,
                None => std::ptr::null(),
            },
            TickBudgetInMilliseconds: self.tick_budget_in_milliseconds,
            RTCOptions: match self.rtc_options {
                Some(rtc_options) => &EOS_Platform_RTCOptions {
                    ApiVersion: EOS_PLATFORM_RTCOPTIONS_API_LATEST as i32,
                    BackgroundMode: rtc_options.into(),
                    PlatformSpecificOptions: std::ptr::null_mut(),
                } as *const _ as *mut _,
                None => std::ptr::null(),
            },
            IntegratedPlatformOptionsContainerHandle: std::ptr::null_mut(),
            Reserved: std::ptr::null_mut(),
            SystemSpecificOptions: std::ptr::null_mut(),
        }
        
    }
}

pub struct EosInitializeOptions {
    /// The name of the product using the Epic Online Services SDK.
    /// The name string is required to be non-empty and at maximum of 64 bytes long. The string buffer can consist of the following characters:
    /// A-Z, a-z, 0-9, dot, underscore, space, exclamation mark, question mark, and sign, hyphen, parenthesis, plus, minus, colon.
    product_name: String,
    /// Product version of the running application.
    /// The version string is required to be non-empty and at maximum of 64 bytes long. The string buffer can consist of the following characters:
    /// A-Z, a-z, 0-9, dot, underscore, space, exclamation mark, question mark, and sign, hyphen, parenthesis, plus, minus, colon.
    product_version: String,
}

impl Into<EOS_InitializeOptions> for EosInitializeOptions {
    fn into(self) -> EOS_InitializeOptions {
        EOS_InitializeOptions {
            ApiVersion: LATEST_API_VERSION,
            ProductName: self.product_name.as_ptr() as *const i8,
            ProductVersion: self.product_version.as_ptr() as *const i8,
            /// TODO does it make sense to use here a the allocator from the stdlib?
            AllocateMemoryFunction: None,
            ReallocateMemoryFunction: None,
            ReleaseMemoryFunction: None,
            Reserved: std::ptr::null_mut(),
            OverrideThreadAffinity: std::ptr::null_mut(),
            SystemInitializeOptions: std::ptr::null_mut(),
        }
    }
}
