use eos_sys::EOS_EResult;

/// General Errors
pub enum EosError {
    Success,
    NoConnection,
    InvalidCredentials,
    InvalidUser,
    InvalidAuth,
    AccessDenied,
    MissingPermissions,
    TokenNotAccount,
    TooManyRequests,
    AlreadyPending,
    InvalidParameters,
    InvalidRequest,
    UnrecognizedResponse,
    IncompatibleVersion,
    NotConfigured,
    AlreadyConfigured,
    NotImplemented,
    Canceled,
    NotFound,
    OperationWillRetry,
    NoChange,
    VersionMismatch,
    LimitExceeded,
    Disabled,
    DuplicateNotAllowed,
    MissingParametersDepracted,
    InvalidSandboxId,
    TimedOut,
    PartialResult,
    MissingRole,
    MissingFeature,
    InvalidSandbox,
    InvalidDeployment,
    InvalidProduct,
    InvalidProductUserID,
    ServiceFailure,
    CacheDirectoryMissing,
    CacheDirectoryInvalid,
    InvalidState,
    RequestInProgress,
    ApplicationSuspended,
    NetworkDisconnected,
    AuthAccountLocked,
    AuthAccountLockedForUpdate,
    AuthInvalidRefreshToken,
    AuthInvalidToken,
    AuthAuthenticationFailure,
    AuthInvalidPlatformToken,
    AuthWrongAccount,
    AuthWrongClient,
    AuthFullAccountRequired,
    AuthHeadlessAccountRequired,
    AuthPasswordResetRequired,
    AuthPasswordCannotBeReused,
    AuthExpired,
    AuthScopeConsentRequired,
    AuthApplicationNotFound,
    AuthScopeNotFound,
    AuthAccountFeatureRestricted,
    AuthAccountPortalLoadError,
    AuthCorrectiveActionRequired,
    AuthPinGrantCode,
    AuthPinGrantExpired,
    AuthPinGrantPending,
    AuthExternalAuthNotLinked,
    AuthExternalAuthRevoked,
    AuthExternalAuthInvalid,
    AuthExternalAuthRestricted,
    AuthExternalAuthCannotLogin,
    AuthExternalAuthExpired,
    AuthExternalAuthIsLastLoginType,
    AuthExchangeCodeNotFound,
    AuthOriginatingExchangeCodeSessionExpired,
    AuthAccountNotActive,
    AuthMFARequired,
    AuthParentalControls,
    AuthNoRealId,
    AuthUserInterfaceRequired,
    FriendsInviteAwaitingAcceptance,
    FriendsNoInvitation,
    FriendsAlreadyFriends,
    FriendsNotFriends,
    FriendsTargetUserTooManyInvites,
    FriendsLocalUserTooManyInvites,
    FriendsTargetUserFriendLimitExceeded,
    FriendsLocalUserFriendLimitExceeded,
    PresenceDataInvalid,
    PresenceDataLengthInvalid,
    PresenceDataKeyInvalid,
    PresenceDataKeyLengthInvalid,
    PresenceDataValueInvalid,
    PresenceDataValueLengthInvalid,
    PresenceRichTextInvalid,
    PresenceRichTextLengthInvalid,
    PresenceStatusInvalid,
    EcomEntitlementStale,
    EcomCatalogOfferStale,
    EcomCatalogItemStale,
    EcomCatalogOfferPriceInvalid,
    EcomCheckoutLoadError,
    EcomPurchaseProcessing,
    SessionsSessionInProgress,
    SessionsTooManyPlayers,
    SessionsNoPermission,
    SessionsSessionAlreadyExists,
    SessionsInvalidLock,
    SessionsInvalidSession,
    SessionsSandboxNotAllowed,
    SessionsInviteFailed,
    SessionsInviteNotFound,
    SessionsUpsertNotAllowed,
    SessionsAggregationFailed,
    SessionsHostAtCapacity,
    SessionsSandboxAtCapacity,
    SessionsSessionNotAnonymous,
    SessionsOutOfSync,
    SessionsTooManyInvites,
    SessionsPresenceSessionExists,
    SessionsDeploymentAtCapacity,
    SessionsNotAllowed,
    SessionsPlayerSanctioned,
    PlayerDataStorageFilenameInvalid,
    PlayerDataStorageFilenameLengthInvalid,
    PlayerDataStorageFilenameInvalidChars,
    PlayerDataStorageFileSizeTooLarge,
    PlayerDataStorageFileSizeInvalid,
    PlayerDataStorageFileHandleInvalid,
    PlayerDataStorageDataInvalid,
    PlayerDataStorageDataLengthInvalid,
    PlayerDataStorageStartIndexInvalid,
    PlayerDataStorageRequestInProgress,
    PlayerDataStorageUserThrottled,
    PlayerDataStorageEncryptionKeyNotSet,
    PlayerDataStorageUserErrorFromDataCallback,
    PlayerDataStorageFileHeaderHasNewerVersion,
    PlayerDataStorageFileCorrupted,
    ConnectExternalTokenValidationFailed,
    ConnectUserAlreadyExists,
    ConnectAuthExpired,
    ConnectInvalidToken,
    ConnectUnsupportedTokenType,
    ConnectLinkAccountFailed,
    ConnectExternalServiceUnavailable,
    ConnectExternalServiceConfigurationFailure,
    ConnectLinkAccountFailedMissingNintendoIdAccountDeprecated,
    UISocialOverlayLoadError,
    LobbyNotOwner,
    LobbyInvalidLock,
    LobbyLobbyAlreadyExists,
    LobbySessionInProgress,
    LobbyTooManyPlayers,
    LobbyNoPermission,
    LobbyInvalidSession,
    LobbySandboxNotAllowed,
    LobbyInviteFailed,
    LobbyInviteNotFound,
    LobbyUpsertNotAllowed,
    LobbyAggregationFailed,
    LobbyHostAtCapacity,
    LobbySandboxAtCapacity,
    LobbyTooManyInvites,
    LobbyDeploymentAtCapacity,
    LobbyNotAllowed,
    LobbyMemberUpdateOnly,
    LobbyPresenceLobbyExists,
    LobbyVoiceNotEnabled,
    LobbyPlatformNotAllowed,
    TitleStorageUserErrorFromDataCallback,
    TitleStorageEncryptionKeyNotSet,
    TitleStorageFileCorrupted,
    TitleStorageFileHeaderHasNewerVersion,
    ModsModSdkProcessIsAlreadyRunning,
    ModsModSdkCommandIsEmpty,
    ModsModSdkProcessCreationFailed,
    ModsCriticalError,
    ModsToolInternalError,
    ModsIPCFailure,
    ModsInvalidIPCResponse,
    ModsURILaunchFailure,
    ModsModIsNotInstalled,
    ModsUserDoesNotOwnTheGame,
    ModsOfferRequestByIdInvalidResult,
    ModsCouldNotFindOffer,
    ModsOfferRequestByIdFailure,
    ModsPurchaseFailure,
    ModsInvalidGameInstallInfo,
    ModsCannotGetManifestLocation,
    ModsUnsupportedOS,
    AntiCheatClientProtectionNotAvailable,
    AntiCheatInvalidMode,
    AntiCheatClientProductIdMismatch,
    AntiCheatClientSandboxIdMismatch,
    AntiCheatProtectMessageSessionKeyRequired,
    AntiCheatProtectMessageValidationFailed,
    AntiCheatProtectMessageInitializationFailed,
    AntiCheatPeerAlreadyRegistered,
    AntiCheatPeerNotFound,
    AntiCheatPeerNotProtected,
    AntiCheatClientDeploymentIdMismatch,
    AntiCheatDeviceIdAuthIsNotSupported,
    RTCTooManyParticipants,
    RTCRoomAlreadyExists,
    RTCUserKicked,
    RTCUserBanned,
    RTCRoomWasLeft,
    RTCReconnectionTimegateExpired,
    RTCShutdownInvoked,
    RTCUserIsInBlocklist,
    ProgressionSnapshotSnapshotIdUnavailable,
    KWSParentEmailMissing,
    KWSUserGraduated,
    AndroidJavaVMNotStored,
    PermissionRequiredPatchAvailable,
    PermissionRequiredSystemUpdate,
    PermissionAgeRestrictionFailure,
    PermissionAccountTypeFailure,
    PermissionChatRestriction,
    PermissionUGCRestriction,
    PermissionOnlinePlayRestricted,
    DesktopCrossplayApplicationNotBootstrapped,
    DesktopCrossplayServiceNotInstalled,
    DesktopCrossplayServiceStartFailed,
    DesktopCrossplayServiceNotRunning,
    CustomInvitesInviteFailed,
    UserInfoBestDisplayNameIndeterminate,
    UnexpectedError,
}

impl EosError {
    pub fn was_successful(&self) -> bool {
        match self {
            EosError::Success => true,
            _ => false,
        }
    }
}

impl From<EOS_EResult> for EosError {
    fn from(value: EOS_EResult) -> Self {
        match value {
            EOS_EResult::EOS_Success => EosError::Success,
            EOS_EResult::EOS_NoConnection => EosError::NoConnection,
            EOS_EResult::EOS_InvalidCredentials => EosError::InvalidCredentials,
            EOS_EResult::EOS_InvalidUser => EosError::InvalidUser,
            EOS_EResult::EOS_InvalidAuth => EosError::InvalidAuth,
            EOS_EResult::EOS_AccessDenied => EosError::AccessDenied,
            EOS_EResult::EOS_MissingPermissions => EosError::MissingPermissions,
            EOS_EResult::EOS_Token_Not_Account => EosError::TokenNotAccount,
            EOS_EResult::EOS_TooManyRequests => EosError::TooManyRequests,
            EOS_EResult::EOS_AlreadyPending => EosError::AlreadyPending,
            EOS_EResult::EOS_InvalidParameters => EosError::InvalidParameters,
            EOS_EResult::EOS_InvalidRequest => EosError::InvalidRequest,
            EOS_EResult::EOS_UnrecognizedResponse => EosError::UnrecognizedResponse,
            EOS_EResult::EOS_IncompatibleVersion => EosError::IncompatibleVersion,
            EOS_EResult::EOS_NotConfigured => EosError::NotConfigured,
            EOS_EResult::EOS_AlreadyConfigured => EosError::AlreadyConfigured,
            EOS_EResult::EOS_NotImplemented => EosError::NotImplemented,
            EOS_EResult::EOS_Canceled => EosError::Canceled,
            EOS_EResult::EOS_NotFound => EosError::NotFound,
            EOS_EResult::EOS_OperationWillRetry => EosError::OperationWillRetry,
            EOS_EResult::EOS_NoChange => EosError::NoChange,
            EOS_EResult::EOS_VersionMismatch => EosError::VersionMismatch,
            EOS_EResult::EOS_LimitExceeded => EosError::LimitExceeded,
            EOS_EResult::EOS_Disabled => EosError::Disabled,
            EOS_EResult::EOS_DuplicateNotAllowed => EosError::DuplicateNotAllowed,
            EOS_EResult::EOS_MissingParameters_DEPRECATED => EosError::MissingParametersDepracted,
            EOS_EResult::EOS_InvalidSandboxId => EosError::InvalidSandboxId,
            EOS_EResult::EOS_TimedOut => EosError::TimedOut,
            EOS_EResult::EOS_PartialResult => EosError::PartialResult,
            EOS_EResult::EOS_Missing_Role => EosError::MissingRole,
            EOS_EResult::EOS_Missing_Feature => EosError::MissingFeature,
            EOS_EResult::EOS_Invalid_Sandbox => EosError::InvalidSandbox,
            EOS_EResult::EOS_Invalid_Deployment => EosError::InvalidDeployment,
            EOS_EResult::EOS_Invalid_Product => EosError::InvalidProduct,
            EOS_EResult::EOS_Invalid_ProductUserID => EosError::InvalidProductUserID,
            EOS_EResult::EOS_ServiceFailure => EosError::ServiceFailure,
            EOS_EResult::EOS_CacheDirectoryMissing => EosError::CacheDirectoryMissing,
            EOS_EResult::EOS_CacheDirectoryInvalid => EosError::CacheDirectoryInvalid,
            EOS_EResult::EOS_InvalidState => EosError::InvalidState,
            EOS_EResult::EOS_RequestInProgress => EosError::RequestInProgress,
            EOS_EResult::EOS_ApplicationSuspended => EosError::ApplicationSuspended,
            EOS_EResult::EOS_NetworkDisconnected => EosError::NetworkDisconnected,
            EOS_EResult::EOS_Auth_AccountLocked => EosError::AuthAccountLocked,
            EOS_EResult::EOS_Auth_AccountLockedForUpdate => EosError::AuthAccountLockedForUpdate,
            EOS_EResult::EOS_Auth_InvalidRefreshToken => EosError::AuthInvalidRefreshToken,
            EOS_EResult::EOS_Auth_InvalidToken => EosError::AuthInvalidToken,
            EOS_EResult::EOS_Auth_AuthenticationFailure => EosError::AuthAuthenticationFailure,
            EOS_EResult::EOS_Auth_InvalidPlatformToken => EosError::AuthInvalidPlatformToken,
            EOS_EResult::EOS_Auth_WrongAccount => EosError::AuthWrongAccount,
            EOS_EResult::EOS_Auth_WrongClient => EosError::AuthWrongClient,
            EOS_EResult::EOS_Auth_FullAccountRequired => EosError::AuthFullAccountRequired,
            EOS_EResult::EOS_Auth_HeadlessAccountRequired => EosError::AuthHeadlessAccountRequired,
            EOS_EResult::EOS_Auth_PasswordResetRequired => EosError::AuthPasswordResetRequired,
            EOS_EResult::EOS_Auth_PasswordCannotBeReused => EosError::AuthPasswordCannotBeReused,
            EOS_EResult::EOS_Auth_Expired => EosError::AuthExpired,
            EOS_EResult::EOS_Auth_ScopeConsentRequired => EosError::AuthScopeConsentRequired,
            EOS_EResult::EOS_Auth_ApplicationNotFound => EosError::AuthApplicationNotFound,
            EOS_EResult::EOS_Auth_ScopeNotFound => EosError::AuthScopeNotFound,
            EOS_EResult::EOS_Auth_AccountFeatureRestricted => {
                EosError::AuthAccountFeatureRestricted
            }
            EOS_EResult::EOS_Auth_AccountPortalLoadError => EosError::AuthAccountPortalLoadError,
            EOS_EResult::EOS_Auth_CorrectiveActionRequired => {
                EosError::AuthCorrectiveActionRequired
            }
            EOS_EResult::EOS_Auth_PinGrantCode => EosError::AuthPinGrantCode,
            EOS_EResult::EOS_Auth_PinGrantExpired => EosError::AuthPinGrantExpired,
            EOS_EResult::EOS_Auth_PinGrantPending => EosError::AuthPinGrantPending,
            EOS_EResult::EOS_Auth_ExternalAuthNotLinked => EosError::AuthExternalAuthNotLinked,
            EOS_EResult::EOS_Auth_ExternalAuthRevoked => EosError::AuthExternalAuthRevoked,
            EOS_EResult::EOS_Auth_ExternalAuthInvalid => EosError::AuthExternalAuthInvalid,
            EOS_EResult::EOS_Auth_ExternalAuthRestricted => EosError::AuthExternalAuthRestricted,
            EOS_EResult::EOS_Auth_ExternalAuthCannotLogin => EosError::AuthExternalAuthCannotLogin,
            EOS_EResult::EOS_Auth_ExternalAuthExpired => EosError::AuthExternalAuthExpired,
            EOS_EResult::EOS_Auth_ExternalAuthIsLastLoginType => {
                EosError::AuthExternalAuthIsLastLoginType
            }
            EOS_EResult::EOS_Auth_ExchangeCodeNotFound => EosError::AuthExchangeCodeNotFound,
            EOS_EResult::EOS_Auth_OriginatingExchangeCodeSessionExpired => {
                EosError::AuthOriginatingExchangeCodeSessionExpired
            }
            EOS_EResult::EOS_Auth_AccountNotActive => EosError::AuthAccountNotActive,
            EOS_EResult::EOS_Auth_MFARequired => EosError::AuthMFARequired,
            EOS_EResult::EOS_Auth_ParentalControls => EosError::AuthParentalControls,
            EOS_EResult::EOS_Auth_NoRealId => EosError::AuthNoRealId,
            EOS_EResult::EOS_Auth_UserInterfaceRequired => EosError::AuthUserInterfaceRequired,
            EOS_EResult::EOS_Friends_InviteAwaitingAcceptance => {
                EosError::FriendsInviteAwaitingAcceptance
            }
            EOS_EResult::EOS_Friends_NoInvitation => EosError::FriendsNoInvitation,
            EOS_EResult::EOS_Friends_AlreadyFriends => EosError::FriendsAlreadyFriends,
            EOS_EResult::EOS_Friends_NotFriends => EosError::FriendsNotFriends,
            EOS_EResult::EOS_Friends_TargetUserTooManyInvites => {
                EosError::FriendsTargetUserTooManyInvites
            }
            EOS_EResult::EOS_Friends_LocalUserTooManyInvites => {
                EosError::FriendsLocalUserTooManyInvites
            }
            EOS_EResult::EOS_Friends_TargetUserFriendLimitExceeded => {
                EosError::FriendsTargetUserFriendLimitExceeded
            }
            EOS_EResult::EOS_Friends_LocalUserFriendLimitExceeded => {
                EosError::FriendsLocalUserFriendLimitExceeded
            }
            EOS_EResult::EOS_Presence_DataInvalid => EosError::PresenceDataInvalid,
            EOS_EResult::EOS_Presence_DataLengthInvalid => EosError::PresenceDataLengthInvalid,
            EOS_EResult::EOS_Presence_DataKeyInvalid => EosError::PresenceDataKeyInvalid,
            EOS_EResult::EOS_Presence_DataKeyLengthInvalid => {
                EosError::PresenceDataKeyLengthInvalid
            }
            EOS_EResult::EOS_Presence_DataValueInvalid => EosError::PresenceDataValueInvalid,
            EOS_EResult::EOS_Presence_DataValueLengthInvalid => {
                EosError::PresenceDataValueLengthInvalid
            }
            EOS_EResult::EOS_Presence_RichTextInvalid => EosError::PresenceRichTextInvalid,
            EOS_EResult::EOS_Presence_RichTextLengthInvalid => {
                EosError::PresenceRichTextLengthInvalid
            }
            EOS_EResult::EOS_Presence_StatusInvalid => EosError::PresenceStatusInvalid,
            EOS_EResult::EOS_Ecom_EntitlementStale => EosError::EcomEntitlementStale,
            EOS_EResult::EOS_Ecom_CatalogOfferStale => EosError::EcomCatalogOfferStale,
            EOS_EResult::EOS_Ecom_CatalogItemStale => EosError::EcomCatalogItemStale,
            EOS_EResult::EOS_Ecom_CatalogOfferPriceInvalid => {
                EosError::EcomCatalogOfferPriceInvalid
            }
            EOS_EResult::EOS_Ecom_CheckoutLoadError => EosError::EcomCheckoutLoadError,
            EOS_EResult::EOS_Ecom_PurchaseProcessing => EosError::EcomPurchaseProcessing,
            EOS_EResult::EOS_Sessions_SessionInProgress => EosError::SessionsSessionInProgress,
            EOS_EResult::EOS_Sessions_TooManyPlayers => EosError::SessionsTooManyPlayers,
            EOS_EResult::EOS_Sessions_NoPermission => EosError::SessionsNoPermission,
            EOS_EResult::EOS_Sessions_SessionAlreadyExists => {
                EosError::SessionsSessionAlreadyExists
            }
            EOS_EResult::EOS_Sessions_InvalidLock => EosError::SessionsInvalidLock,
            EOS_EResult::EOS_Sessions_InvalidSession => EosError::SessionsInvalidSession,
            EOS_EResult::EOS_Sessions_SandboxNotAllowed => EosError::SessionsSandboxNotAllowed,
            EOS_EResult::EOS_Sessions_InviteFailed => EosError::SessionsInviteFailed,
            EOS_EResult::EOS_Sessions_InviteNotFound => EosError::SessionsInviteNotFound,
            EOS_EResult::EOS_Sessions_UpsertNotAllowed => EosError::SessionsUpsertNotAllowed,
            EOS_EResult::EOS_Sessions_AggregationFailed => EosError::SessionsAggregationFailed,
            EOS_EResult::EOS_Sessions_HostAtCapacity => EosError::SessionsHostAtCapacity,
            EOS_EResult::EOS_Sessions_SandboxAtCapacity => EosError::SessionsSandboxAtCapacity,
            EOS_EResult::EOS_Sessions_SessionNotAnonymous => EosError::SessionsSessionNotAnonymous,
            EOS_EResult::EOS_Sessions_OutOfSync => EosError::SessionsOutOfSync,
            EOS_EResult::EOS_Sessions_TooManyInvites => EosError::SessionsTooManyInvites,
            EOS_EResult::EOS_Sessions_PresenceSessionExists => {
                EosError::SessionsPresenceSessionExists
            }
            EOS_EResult::EOS_Sessions_DeploymentAtCapacity => {
                EosError::SessionsDeploymentAtCapacity
            }
            EOS_EResult::EOS_Sessions_NotAllowed => EosError::SessionsNotAllowed,
            EOS_EResult::EOS_Sessions_PlayerSanctioned => EosError::SessionsPlayerSanctioned,
            EOS_EResult::EOS_PlayerDataStorage_FilenameInvalid => {
                EosError::PlayerDataStorageFilenameInvalid
            }
            EOS_EResult::EOS_PlayerDataStorage_FilenameLengthInvalid => {
                EosError::PlayerDataStorageFilenameLengthInvalid
            }
            EOS_EResult::EOS_PlayerDataStorage_FilenameInvalidChars => {
                EosError::PlayerDataStorageFilenameInvalidChars
            }
            EOS_EResult::EOS_PlayerDataStorage_FileSizeTooLarge => {
                EosError::PlayerDataStorageFileSizeTooLarge
            }
            EOS_EResult::EOS_PlayerDataStorage_FileSizeInvalid => {
                EosError::PlayerDataStorageFileSizeInvalid
            }
            EOS_EResult::EOS_PlayerDataStorage_FileHandleInvalid => {
                EosError::PlayerDataStorageFileHandleInvalid
            }
            EOS_EResult::EOS_PlayerDataStorage_DataInvalid => {
                EosError::PlayerDataStorageDataInvalid
            }
            EOS_EResult::EOS_PlayerDataStorage_DataLengthInvalid => {
                EosError::PlayerDataStorageDataLengthInvalid
            }
            EOS_EResult::EOS_PlayerDataStorage_StartIndexInvalid => {
                EosError::PlayerDataStorageStartIndexInvalid
            }
            EOS_EResult::EOS_PlayerDataStorage_RequestInProgress => {
                EosError::PlayerDataStorageRequestInProgress
            }
            EOS_EResult::EOS_PlayerDataStorage_UserThrottled => {
                EosError::PlayerDataStorageUserThrottled
            }
            EOS_EResult::EOS_PlayerDataStorage_EncryptionKeyNotSet => {
                EosError::PlayerDataStorageEncryptionKeyNotSet
            }
            EOS_EResult::EOS_PlayerDataStorage_UserErrorFromDataCallback => {
                EosError::PlayerDataStorageUserErrorFromDataCallback
            }
            EOS_EResult::EOS_PlayerDataStorage_FileHeaderHasNewerVersion => {
                EosError::PlayerDataStorageFileHeaderHasNewerVersion
            }
            EOS_EResult::EOS_PlayerDataStorage_FileCorrupted => {
                EosError::PlayerDataStorageFileCorrupted
            }
            EOS_EResult::EOS_Connect_ExternalTokenValidationFailed => {
                EosError::ConnectExternalTokenValidationFailed
            }
            EOS_EResult::EOS_Connect_UserAlreadyExists => EosError::ConnectUserAlreadyExists,
            EOS_EResult::EOS_Connect_AuthExpired => EosError::ConnectAuthExpired,
            EOS_EResult::EOS_Connect_InvalidToken => EosError::ConnectInvalidToken,
            EOS_EResult::EOS_Connect_UnsupportedTokenType => EosError::ConnectUnsupportedTokenType,
            EOS_EResult::EOS_Connect_LinkAccountFailed => EosError::ConnectLinkAccountFailed,
            EOS_EResult::EOS_Connect_ExternalServiceUnavailable => {
                EosError::ConnectExternalServiceUnavailable
            }
            EOS_EResult::EOS_Connect_ExternalServiceConfigurationFailure => {
                EosError::ConnectExternalServiceConfigurationFailure
            }
            EOS_EResult::EOS_Connect_LinkAccountFailedMissingNintendoIdAccount_DEPRECATED => {
                EosError::ConnectLinkAccountFailedMissingNintendoIdAccountDeprecated
            }
            EOS_EResult::EOS_UI_SocialOverlayLoadError => EosError::UISocialOverlayLoadError,
            EOS_EResult::EOS_Lobby_NotOwner => EosError::LobbyNotOwner,
            EOS_EResult::EOS_Lobby_InvalidLock => EosError::LobbyInvalidLock,
            EOS_EResult::EOS_Lobby_LobbyAlreadyExists => EosError::LobbyLobbyAlreadyExists,
            EOS_EResult::EOS_Lobby_SessionInProgress => EosError::LobbySessionInProgress,
            EOS_EResult::EOS_Lobby_TooManyPlayers => EosError::LobbyTooManyPlayers,
            EOS_EResult::EOS_Lobby_NoPermission => EosError::LobbyNoPermission,
            EOS_EResult::EOS_Lobby_InvalidSession => EosError::LobbyInvalidSession,
            EOS_EResult::EOS_Lobby_SandboxNotAllowed => EosError::LobbySandboxNotAllowed,
            EOS_EResult::EOS_Lobby_InviteFailed => EosError::LobbyInviteFailed,
            EOS_EResult::EOS_Lobby_InviteNotFound => EosError::LobbyInviteNotFound,
            EOS_EResult::EOS_Lobby_UpsertNotAllowed => EosError::LobbyUpsertNotAllowed,
            EOS_EResult::EOS_Lobby_AggregationFailed => EosError::LobbyAggregationFailed,
            EOS_EResult::EOS_Lobby_HostAtCapacity => EosError::LobbyHostAtCapacity,
            EOS_EResult::EOS_Lobby_SandboxAtCapacity => EosError::LobbySandboxAtCapacity,
            EOS_EResult::EOS_Lobby_TooManyInvites => EosError::LobbyTooManyInvites,
            EOS_EResult::EOS_Lobby_DeploymentAtCapacity => EosError::LobbyDeploymentAtCapacity,
            EOS_EResult::EOS_Lobby_NotAllowed => EosError::LobbyNotAllowed,
            EOS_EResult::EOS_Lobby_MemberUpdateOnly => EosError::LobbyMemberUpdateOnly,
            EOS_EResult::EOS_Lobby_PresenceLobbyExists => EosError::LobbyPresenceLobbyExists,
            EOS_EResult::EOS_Lobby_VoiceNotEnabled => EosError::LobbyVoiceNotEnabled,
            EOS_EResult::EOS_Lobby_PlatformNotAllowed => EosError::LobbyPlatformNotAllowed,
            EOS_EResult::EOS_TitleStorage_UserErrorFromDataCallback => {
                EosError::TitleStorageUserErrorFromDataCallback
            }
            EOS_EResult::EOS_TitleStorage_EncryptionKeyNotSet => {
                EosError::TitleStorageEncryptionKeyNotSet
            }
            EOS_EResult::EOS_TitleStorage_FileCorrupted => EosError::TitleStorageFileCorrupted,
            EOS_EResult::EOS_TitleStorage_FileHeaderHasNewerVersion => {
                EosError::TitleStorageFileHeaderHasNewerVersion
            }
            EOS_EResult::EOS_Mods_ModSdkProcessIsAlreadyRunning => {
                EosError::ModsModSdkProcessIsAlreadyRunning
            }
            EOS_EResult::EOS_Mods_ModSdkCommandIsEmpty => EosError::ModsModSdkCommandIsEmpty,
            EOS_EResult::EOS_Mods_ModSdkProcessCreationFailed => {
                EosError::ModsModSdkProcessCreationFailed
            }
            EOS_EResult::EOS_Mods_CriticalError => EosError::ModsCriticalError,
            EOS_EResult::EOS_Mods_ToolInternalError => EosError::ModsToolInternalError,
            EOS_EResult::EOS_Mods_IPCFailure => EosError::ModsIPCFailure,
            EOS_EResult::EOS_Mods_InvalidIPCResponse => EosError::ModsInvalidIPCResponse,
            EOS_EResult::EOS_Mods_URILaunchFailure => EosError::ModsURILaunchFailure,
            EOS_EResult::EOS_Mods_ModIsNotInstalled => EosError::ModsModIsNotInstalled,
            EOS_EResult::EOS_Mods_UserDoesNotOwnTheGame => EosError::ModsUserDoesNotOwnTheGame,
            EOS_EResult::EOS_Mods_OfferRequestByIdInvalidResult => {
                EosError::ModsOfferRequestByIdInvalidResult
            }
            EOS_EResult::EOS_Mods_CouldNotFindOffer => EosError::ModsCouldNotFindOffer,
            EOS_EResult::EOS_Mods_OfferRequestByIdFailure => EosError::ModsOfferRequestByIdFailure,
            EOS_EResult::EOS_Mods_PurchaseFailure => EosError::ModsPurchaseFailure,
            EOS_EResult::EOS_Mods_InvalidGameInstallInfo => EosError::ModsInvalidGameInstallInfo,
            EOS_EResult::EOS_Mods_CannotGetManifestLocation => {
                EosError::ModsCannotGetManifestLocation
            }
            EOS_EResult::EOS_Mods_UnsupportedOS => EosError::ModsUnsupportedOS,
            EOS_EResult::EOS_AntiCheat_ClientProtectionNotAvailable => {
                EosError::AntiCheatClientProtectionNotAvailable
            }
            EOS_EResult::EOS_AntiCheat_InvalidMode => EosError::AntiCheatInvalidMode,
            EOS_EResult::EOS_AntiCheat_ClientProductIdMismatch => {
                EosError::AntiCheatClientProductIdMismatch
            }
            EOS_EResult::EOS_AntiCheat_ClientSandboxIdMismatch => {
                EosError::AntiCheatClientSandboxIdMismatch
            }
            EOS_EResult::EOS_AntiCheat_ProtectMessageSessionKeyRequired => {
                EosError::AntiCheatProtectMessageSessionKeyRequired
            }
            EOS_EResult::EOS_AntiCheat_ProtectMessageValidationFailed => {
                EosError::AntiCheatProtectMessageValidationFailed
            }
            EOS_EResult::EOS_AntiCheat_ProtectMessageInitializationFailed => {
                EosError::AntiCheatProtectMessageInitializationFailed
            }
            EOS_EResult::EOS_AntiCheat_PeerAlreadyRegistered => {
                EosError::AntiCheatPeerAlreadyRegistered
            }
            EOS_EResult::EOS_AntiCheat_PeerNotFound => EosError::AntiCheatPeerNotFound,
            EOS_EResult::EOS_AntiCheat_PeerNotProtected => EosError::AntiCheatPeerNotProtected,
            EOS_EResult::EOS_AntiCheat_ClientDeploymentIdMismatch => {
                EosError::AntiCheatClientDeploymentIdMismatch
            }
            EOS_EResult::EOS_AntiCheat_DeviceIdAuthIsNotSupported => {
                EosError::AntiCheatDeviceIdAuthIsNotSupported
            }
            EOS_EResult::EOS_RTC_TooManyParticipants => EosError::RTCTooManyParticipants,
            EOS_EResult::EOS_RTC_RoomAlreadyExists => EosError::RTCRoomAlreadyExists,
            EOS_EResult::EOS_RTC_UserKicked => EosError::RTCUserKicked,
            EOS_EResult::EOS_RTC_UserBanned => EosError::RTCUserBanned,
            EOS_EResult::EOS_RTC_RoomWasLeft => EosError::RTCRoomWasLeft,
            EOS_EResult::EOS_RTC_ReconnectionTimegateExpired => {
                EosError::RTCReconnectionTimegateExpired
            }
            EOS_EResult::EOS_RTC_ShutdownInvoked => EosError::RTCShutdownInvoked,
            EOS_EResult::EOS_RTC_UserIsInBlocklist => EosError::RTCUserIsInBlocklist,
            EOS_EResult::EOS_ProgressionSnapshot_SnapshotIdUnavailable => {
                EosError::ProgressionSnapshotSnapshotIdUnavailable
            }
            EOS_EResult::EOS_KWS_ParentEmailMissing => EosError::KWSParentEmailMissing,
            EOS_EResult::EOS_KWS_UserGraduated => EosError::KWSUserGraduated,
            EOS_EResult::EOS_Android_JavaVMNotStored => EosError::AndroidJavaVMNotStored,
            EOS_EResult::EOS_Permission_RequiredPatchAvailable => {
                EosError::PermissionRequiredPatchAvailable
            }
            EOS_EResult::EOS_Permission_RequiredSystemUpdate => {
                EosError::PermissionRequiredSystemUpdate
            }
            EOS_EResult::EOS_Permission_AgeRestrictionFailure => {
                EosError::PermissionAgeRestrictionFailure
            }
            EOS_EResult::EOS_Permission_AccountTypeFailure => {
                EosError::PermissionAccountTypeFailure
            }
            EOS_EResult::EOS_Permission_ChatRestriction => EosError::PermissionChatRestriction,
            EOS_EResult::EOS_Permission_UGCRestriction => EosError::PermissionUGCRestriction,
            EOS_EResult::EOS_Permission_OnlinePlayRestricted => {
                EosError::PermissionOnlinePlayRestricted
            }
            EOS_EResult::EOS_DesktopCrossplay_ApplicationNotBootstrapped => {
                EosError::DesktopCrossplayApplicationNotBootstrapped
            }
            EOS_EResult::EOS_DesktopCrossplay_ServiceNotInstalled => {
                EosError::DesktopCrossplayServiceNotInstalled
            }
            EOS_EResult::EOS_DesktopCrossplay_ServiceStartFailed => {
                EosError::DesktopCrossplayServiceStartFailed
            }
            EOS_EResult::EOS_DesktopCrossplay_ServiceNotRunning => {
                EosError::DesktopCrossplayServiceNotRunning
            }
            EOS_EResult::EOS_CustomInvites_InviteFailed => EosError::CustomInvitesInviteFailed,
            EOS_EResult::EOS_UserInfo_BestDisplayNameIndeterminate => {
                EosError::UserInfoBestDisplayNameIndeterminate
            }
            EOS_EResult::EOS_UnexpectedError => EosError::UnexpectedError,
            _ => unreachable!(),
        }
    }
}
