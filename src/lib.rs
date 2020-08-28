mod utils {
    use serde::Deserialize;
    #[allow(dead_code)]
    pub fn from_str_to_t<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        T::from_str(&s).map_err(serde::de::Error::custom)
    }
    #[allow(dead_code)]
    pub fn from_str_to_opt_t<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
        D: serde::Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => Ok(Some(T::from_str(&s).map_err(serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
pub mod types {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An object of this type can be returned on every function call, in case of an error"]
    pub struct Error {
        #[doc = "Error code; subject to future changes. If the error code is 406, the error message must not be processed in any way and must not be displayed to the user"]
        pub code: i32,
        #[doc = "Error message; subject to future changes"]
        pub message: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An object of this type is returned on a successful function call for certain functions"]
    pub struct Ok {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains parameters for TDLib initialization"]
    pub struct TdlibParameters {
        #[doc = "If set to true, the Telegram test environment will be used instead of the production environment"]
        pub use_test_dc: bool,
        #[doc = "The path to the directory for the persistent database; if empty, the current working directory will be used"]
        pub database_directory: String,
        #[doc = "The path to the directory for storing files; if empty, database_directory will be used"]
        pub files_directory: String,
        #[doc = "If set to true, information about downloaded and uploaded files will be saved between application restarts"]
        pub use_file_database: bool,
        #[doc = "If set to true, the library will maintain a cache of users, basic groups, supergroups, channels and secret chats. Implies use_file_database"]
        pub use_chat_info_database: bool,
        #[doc = "If set to true, the library will maintain a cache of chats and messages. Implies use_chat_info_database"]
        pub use_message_database: bool,
        #[doc = "If set to true, support for secret chats will be enabled"]
        pub use_secret_chats: bool,
        #[doc = "Application identifier for Telegram API access, which can be obtained at https://my.telegram.org"]
        pub api_id: i32,
        #[doc = "Application identifier hash for Telegram API access, which can be obtained at https://my.telegram.org"]
        pub api_hash: String,
        #[doc = "IETF language tag of the user's operating system language; must be non-empty"]
        pub system_language_code: String,
        #[doc = "Model of the device the application is being run on; must be non-empty"]
        pub device_model: String,
        #[doc = "Version of the operating system the application is being run on; must be non-empty"]
        pub system_version: String,
        #[doc = "Application version; must be non-empty"]
        pub application_version: String,
        #[doc = "If set to true, old files will automatically be deleted"]
        pub enable_storage_optimizer: bool,
        #[doc = "If set to true, original file names will be ignored. Otherwise, downloaded files will be saved under names as close as possible to the original name"]
        pub ignore_file_names: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An authentication code is delivered via a private Telegram message, which can be viewed in another client"]
    pub struct AuthenticationCodeTypeTelegramMessage {
        #[doc = "Length of the code"]
        pub length: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An authentication code is delivered via an SMS message to the specified phone number"]
    pub struct AuthenticationCodeTypeSms {
        #[doc = "Length of the code"]
        pub length: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An authentication code is delivered via a phone call to the specified phone number"]
    pub struct AuthenticationCodeTypeCall {
        #[doc = "Length of the code"]
        pub length: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An authentication code is delivered by an immediately cancelled call to the specified phone number. The number from which the call was made is the code"]
    pub struct AuthenticationCodeTypeFlashCall {
        #[doc = "Pattern of the phone number from which the call will be made"]
        pub pattern: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Provides information about the method by which an authentication code is delivered to the user"]
    pub enum AuthenticationCodeType {
        AuthenticationCodeTypeTelegramMessage(AuthenticationCodeTypeTelegramMessage),
        AuthenticationCodeTypeSms(AuthenticationCodeTypeSms),
        AuthenticationCodeTypeCall(AuthenticationCodeTypeCall),
        AuthenticationCodeTypeFlashCall(AuthenticationCodeTypeFlashCall),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Information about the authentication code that was sent"]
    pub struct AuthenticationCodeInfo {
        #[doc = "A phone number that is being authenticated"]
        pub phone_number: String,
        #[serde(rename = "type")]
        #[doc = "Describes the way the code was sent to the user"]
        pub type_: AuthenticationCodeType,
        #[serde(default)]
        #[doc = "Describes the way the next code will be sent to the user; may be null"]
        pub next_type: Option<AuthenticationCodeType>,
        #[doc = "Timeout before the code should be re-sent, in seconds"]
        pub timeout: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Information about the email address authentication code that was sent"]
    pub struct EmailAddressAuthenticationCodeInfo {
        #[doc = "Pattern of the email address to which an authentication code was sent"]
        pub email_address_pattern: String,
        #[doc = "Length of the code; 0 if unknown"]
        pub length: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a part of the text that needs to be formatted in some unusual way"]
    pub struct TextEntity {
        #[doc = "Offset of the entity in UTF-16 code units"]
        pub offset: i32,
        #[doc = "Length of the entity, in UTF-16 code units"]
        pub length: i32,
        #[serde(rename = "type")]
        #[doc = "Type of the entity"]
        pub type_: TextEntityType,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of text entities"]
    pub struct TextEntities {
        #[doc = "List of text entities"]
        pub entities: Vec<TextEntity>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A text with some entities"]
    pub struct FormattedText {
        #[doc = "The text"]
        pub text: String,
        #[doc = "Entities contained in the text. Entities can be nested, but must not mutually intersect with each other. Pre, Code and PreCode entities can't contain other entities. Bold, Italic, Underline and Strikethrough entities can contain and to be contained in all other entities. All other entities can't contain each other"]
        pub entities: Vec<TextEntity>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains Telegram terms of service"]
    pub struct TermsOfService {
        #[doc = "Text of the terms of service"]
        pub text: FormattedText,
        #[doc = "The minimum age of a user to be able to accept the terms; 0 if any"]
        pub min_user_age: i32,
        #[doc = "True, if a blocking popup with terms of service must be shown to the user"]
        pub show_popup: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "TDLib needs TdlibParameters for initialization"]
    pub struct AuthorizationStateWaitTdlibParameters {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "TDLib needs an encryption key to decrypt the local database"]
    pub struct AuthorizationStateWaitEncryptionKey {
        #[doc = "True, if the database is currently encrypted"]
        pub is_encrypted: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "TDLib needs the user's phone number to authorize. Call `setAuthenticationPhoneNumber` to provide the phone number, or use `requestQrCodeAuthentication`, or `checkAuthenticationBotToken` for other authentication options"]
    pub struct AuthorizationStateWaitPhoneNumber {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "TDLib needs the user's authentication code to authorize"]
    pub struct AuthorizationStateWaitCode {
        #[doc = "Information about the authorization code that was sent"]
        pub code_info: AuthenticationCodeInfo,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link"]
    pub struct AuthorizationStateWaitOtherDeviceConfirmation {
        #[doc = "A tg:// URL for the QR code. The link will be updated frequently"]
        pub link: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is unregistered and need to accept terms of service and enter their first name and last name to finish registration"]
    pub struct AuthorizationStateWaitRegistration {
        #[doc = "Telegram terms of service"]
        pub terms_of_service: TermsOfService,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user has been authorized, but needs to enter a password to start using the application"]
    pub struct AuthorizationStateWaitPassword {
        #[doc = "Hint for the password; may be empty"]
        pub password_hint: String,
        #[doc = "True, if a recovery email address has been set up"]
        pub has_recovery_email_address: bool,
        #[doc = "Pattern of the email address to which the recovery email was sent; empty until a recovery email has been sent"]
        pub recovery_email_address_pattern: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user has been successfully authorized. TDLib is now ready to answer queries"]
    pub struct AuthorizationStateReady {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is currently logging out"]
    pub struct AuthorizationStateLoggingOut {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received"]
    pub struct AuthorizationStateClosing {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to with error code 500. To continue working, one should create a new instance of the TDLib client"]
    pub struct AuthorizationStateClosed {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents the current authorization state of the client"]
    pub enum AuthorizationState {
        AuthorizationStateWaitTdlibParameters(AuthorizationStateWaitTdlibParameters),
        AuthorizationStateWaitEncryptionKey(AuthorizationStateWaitEncryptionKey),
        AuthorizationStateWaitPhoneNumber(AuthorizationStateWaitPhoneNumber),
        AuthorizationStateWaitCode(AuthorizationStateWaitCode),
        AuthorizationStateWaitOtherDeviceConfirmation(
            AuthorizationStateWaitOtherDeviceConfirmation,
        ),
        AuthorizationStateWaitRegistration(AuthorizationStateWaitRegistration),
        AuthorizationStateWaitPassword(AuthorizationStateWaitPassword),
        AuthorizationStateReady(AuthorizationStateReady),
        AuthorizationStateLoggingOut(AuthorizationStateLoggingOut),
        AuthorizationStateClosing(AuthorizationStateClosing),
        AuthorizationStateClosed(AuthorizationStateClosed),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents the current state of 2-step verification"]
    pub struct PasswordState {
        #[doc = "True, if a 2-step verification password is set"]
        pub has_password: bool,
        #[doc = "Hint for the password; may be empty"]
        pub password_hint: String,
        #[doc = "True, if a recovery email is set"]
        pub has_recovery_email_address: bool,
        #[doc = "True, if some Telegram Passport elements were saved"]
        pub has_passport_data: bool,
        #[serde(default)]
        #[doc = "Information about the recovery email address to which the confirmation email was sent; may be null"]
        pub recovery_email_address_code_info: Option<EmailAddressAuthenticationCodeInfo>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about the current recovery email address"]
    pub struct RecoveryEmailAddress {
        #[doc = "Recovery email address"]
        pub recovery_email_address: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns information about the availability of a temporary password, which can be used for payments"]
    pub struct TemporaryPasswordState {
        #[doc = "True, if a temporary password is available"]
        pub has_password: bool,
        #[doc = "Time left before the temporary password expires, in seconds"]
        pub valid_for: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a local file"]
    pub struct LocalFile {
        #[doc = "Local path to the locally available file part; may be empty"]
        pub path: String,
        #[doc = "True, if it is possible to try to download or generate the file"]
        pub can_be_downloaded: bool,
        #[doc = "True, if the file can be deleted"]
        pub can_be_deleted: bool,
        #[doc = "True, if the file is currently being downloaded (or a local copy is being generated by some other means)"]
        pub is_downloading_active: bool,
        #[doc = "True, if the local copy is fully available"]
        pub is_downloading_completed: bool,
        #[doc = "Download will be started from this offset. downloaded_prefix_size is calculated from this offset"]
        pub download_offset: i32,
        #[doc = "If is_downloading_completed is false, then only some prefix of the file starting from download_offset is ready to be read. downloaded_prefix_size is the size of that prefix"]
        pub downloaded_prefix_size: i32,
        #[doc = "Total downloaded file bytes. Should be used only for calculating download progress. The actual file size may be bigger, and some parts of it may contain garbage"]
        pub downloaded_size: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a remote file"]
    pub struct RemoteFile {
        #[doc = "Remote file identifier; may be empty. Can be used across application restarts or even from other devices for the current user. Uniquely identifies a file, but a file can have a lot of different valid identifiers. If the ID starts with \"http://\" or \"https://\", it represents the HTTP URL of the file. TDLib is currently unable to download files if only their URL is known. If downloadFile is called on such a file or if it is sent to a secret chat, TDLib starts a file generation process by sending updateFileGenerationStart to the client with the HTTP URL in the original_path and \"#url#\" as the conversion string. Clients should generate the file by downloading it to the specified location"]
        pub id: String,
        #[doc = "Unique file identifier; may be empty if unknown. The unique file identifier which is the same for the same file even for different users and is persistent over time"]
        pub unique_id: String,
        #[doc = "True, if the file is currently being uploaded (or a remote copy is being generated by some other means)"]
        pub is_uploading_active: bool,
        #[doc = "True, if a remote copy is fully available"]
        pub is_uploading_completed: bool,
        #[doc = "Size of the remote available part of the file; 0 if unknown"]
        pub uploaded_size: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a file"]
    pub struct File {
        #[doc = "Unique file identifier"]
        pub id: i32,
        #[doc = "File size; 0 if unknown"]
        pub size: i32,
        #[doc = "Expected file size in case the exact file size is unknown, but an approximate size is known. Can be used to show download/upload progress"]
        pub expected_size: i32,
        #[doc = "Information about the local copy of the file"]
        pub local: LocalFile,
        #[doc = "Information about the remote copy of the file"]
        pub remote: RemoteFile,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A file defined by its unique ID"]
    pub struct InputFileId {
        #[doc = "Unique file identifier"]
        pub id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A file defined by its remote ID. The remote ID is guaranteed to be usable only if the corresponding file is still accessible to the user and known to TDLib. For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the client"]
    pub struct InputFileRemote {
        #[doc = "Remote file identifier"]
        pub id: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A file defined by a local path"]
    pub struct InputFileLocal {
        #[doc = "Local path to the file"]
        pub path: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A file generated by the client"]
    pub struct InputFileGenerated {
        #[doc = "Local path to a file from which the file is generated; may be empty if there is no such file"]
        pub original_path: String,
        #[doc = "String specifying the conversion applied to the original file; should be persistent across application restarts. Conversions beginning with '#' are reserved for internal TDLib usage"]
        pub conversion: String,
        #[doc = "Expected size of the generated file; 0 if unknown"]
        pub expected_size: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Points to a file"]
    pub enum InputFile {
        InputFileId(InputFileId),
        InputFileRemote(InputFileRemote),
        InputFileLocal(InputFileLocal),
        InputFileGenerated(InputFileGenerated),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Photo description"]
    pub struct PhotoSize {
        #[serde(rename = "type")]
        #[doc = "Thumbnail type (see https://core.telegram.org/constructor/photoSize)"]
        pub type_: String,
        #[doc = "Information about the photo file"]
        pub photo: File,
        #[doc = "Photo width"]
        pub width: i32,
        #[doc = "Photo height"]
        pub height: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Thumbnail image of a very poor quality and low resolution"]
    pub struct Minithumbnail {
        #[doc = "Thumbnail width, usually doesn't exceed 40"]
        pub width: i32,
        #[doc = "Thumbnail height, usually doesn't exceed 40"]
        pub height: i32,
        #[doc = "The thumbnail in JPEG format"]
        pub data: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A mask should be placed relatively to the forehead"]
    pub struct MaskPointForehead {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A mask should be placed relatively to the eyes"]
    pub struct MaskPointEyes {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A mask should be placed relatively to the mouth"]
    pub struct MaskPointMouth {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A mask should be placed relatively to the chin"]
    pub struct MaskPointChin {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Part of the face, relative to which a mask should be placed"]
    pub enum MaskPoint {
        MaskPointForehead(MaskPointForehead),
        MaskPointEyes(MaskPointEyes),
        MaskPointMouth(MaskPointMouth),
        MaskPointChin(MaskPointChin),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Position on a photo where a mask should be placed"]
    pub struct MaskPosition {
        #[doc = "Part of the face, relative to which the mask should be placed"]
        pub point: MaskPoint,
        #[doc = "Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. (For example, -1.0 will place the mask just to the left of the default mask position)"]
        pub x_shift: f64,
        #[doc = "Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. (For example, 1.0 will place the mask just below the default mask position)"]
        pub y_shift: f64,
        #[doc = "Mask scaling coefficient. (For example, 2.0 means a doubled size)"]
        pub scale: f64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes one answer option of a poll"]
    pub struct PollOption {
        #[doc = "Option text, 1-100 characters"]
        pub text: String,
        #[doc = "Number of voters for this option, available only for closed or voted polls"]
        pub voter_count: i32,
        #[doc = "The percentage of votes for this option, 0-100"]
        pub vote_percentage: i32,
        #[doc = "True, if the option was chosen by the user"]
        pub is_chosen: bool,
        #[doc = "True, if the option is being chosen by a pending setPollAnswer request"]
        pub is_being_chosen: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A regular poll"]
    pub struct PollTypeRegular {
        #[doc = "True, if multiple answer options can be chosen simultaneously"]
        pub allow_multiple_answers: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A poll in quiz mode, which has exactly one correct answer option and can be answered only once"]
    pub struct PollTypeQuiz {
        #[doc = "0-based identifier of the correct answer option; -1 for a yet unanswered poll"]
        pub correct_option_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the type of a poll"]
    pub enum PollType {
        PollTypeRegular(PollTypeRegular),
        PollTypeQuiz(PollTypeQuiz),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes an animation file. The animation must be encoded in GIF or MPEG4 format"]
    pub struct Animation {
        #[doc = "Duration of the animation, in seconds; as defined by the sender"]
        pub duration: i32,
        #[doc = "Width of the animation"]
        pub width: i32,
        #[doc = "Height of the animation"]
        pub height: i32,
        #[doc = "Original name of the file; as defined by the sender"]
        pub file_name: String,
        #[doc = "MIME type of the file, usually \"image/gif\" or \"video/mp4\""]
        pub mime_type: String,
        #[serde(default)]
        #[doc = "Animation minithumbnail; may be null"]
        pub minithumbnail: Option<Minithumbnail>,
        #[serde(default)]
        #[doc = "Animation thumbnail; may be null"]
        pub thumbnail: Option<PhotoSize>,
        #[doc = "File containing the animation"]
        pub animation: File,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes an audio file. Audio is usually in MP3 or M4A format"]
    pub struct Audio {
        #[doc = "Duration of the audio, in seconds; as defined by the sender"]
        pub duration: i32,
        #[doc = "Title of the audio; as defined by the sender"]
        pub title: String,
        #[doc = "Performer of the audio; as defined by the sender"]
        pub performer: String,
        #[doc = "Original name of the file; as defined by the sender"]
        pub file_name: String,
        #[doc = "The MIME type of the file; as defined by the sender"]
        pub mime_type: String,
        #[serde(default)]
        #[doc = "The minithumbnail of the album cover; may be null"]
        pub album_cover_minithumbnail: Option<Minithumbnail>,
        #[serde(default)]
        #[doc = "The thumbnail of the album cover; as defined by the sender. The full size thumbnail should be extracted from the downloaded file; may be null"]
        pub album_cover_thumbnail: Option<PhotoSize>,
        #[doc = "File containing the audio"]
        pub audio: File,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a document of any type"]
    pub struct Document {
        #[doc = "Original name of the file; as defined by the sender"]
        pub file_name: String,
        #[doc = "MIME type of the file; as defined by the sender"]
        pub mime_type: String,
        #[serde(default)]
        #[doc = "Document minithumbnail; may be null"]
        pub minithumbnail: Option<Minithumbnail>,
        #[serde(default)]
        #[doc = "Document thumbnail in JPEG or PNG format (PNG will be used only for background patterns); as defined by the sender; may be null"]
        pub thumbnail: Option<PhotoSize>,
        #[doc = "File containing the document"]
        pub document: File,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a photo"]
    pub struct Photo {
        #[doc = "True, if stickers were added to the photo"]
        pub has_stickers: bool,
        #[serde(default)]
        #[doc = "Photo minithumbnail; may be null"]
        pub minithumbnail: Option<Minithumbnail>,
        #[doc = "Available variants of the photo, in different sizes"]
        pub sizes: Vec<PhotoSize>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a sticker"]
    pub struct Sticker {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "The identifier of the sticker set to which the sticker belongs; 0 if none"]
        pub set_id: i64,
        #[doc = "Sticker width; as defined by the sender"]
        pub width: i32,
        #[doc = "Sticker height; as defined by the sender"]
        pub height: i32,
        #[doc = "Emoji corresponding to the sticker"]
        pub emoji: String,
        #[doc = "True, if the sticker is an animated sticker in TGS format"]
        pub is_animated: bool,
        #[doc = "True, if the sticker is a mask"]
        pub is_mask: bool,
        #[serde(default)]
        #[doc = "Position where the mask should be placed; may be null"]
        pub mask_position: Option<MaskPosition>,
        #[serde(default)]
        #[doc = "Sticker thumbnail in WEBP or JPEG format; may be null"]
        pub thumbnail: Option<PhotoSize>,
        #[doc = "File containing the sticker"]
        pub sticker: File,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a video file"]
    pub struct Video {
        #[doc = "Duration of the video, in seconds; as defined by the sender"]
        pub duration: i32,
        #[doc = "Video width; as defined by the sender"]
        pub width: i32,
        #[doc = "Video height; as defined by the sender"]
        pub height: i32,
        #[doc = "Original name of the file; as defined by the sender"]
        pub file_name: String,
        #[doc = "MIME type of the file; as defined by the sender"]
        pub mime_type: String,
        #[doc = "True, if stickers were added to the video"]
        pub has_stickers: bool,
        #[doc = "True, if the video should be tried to be streamed"]
        pub supports_streaming: bool,
        #[serde(default)]
        #[doc = "Video minithumbnail; may be null"]
        pub minithumbnail: Option<Minithumbnail>,
        #[serde(default)]
        #[doc = "Video thumbnail; as defined by the sender; may be null"]
        pub thumbnail: Option<PhotoSize>,
        #[doc = "File containing the video"]
        pub video: File,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a video note. The video must be equal in width and height, cropped to a circle, and stored in MPEG4 format"]
    pub struct VideoNote {
        #[doc = "Duration of the video, in seconds; as defined by the sender"]
        pub duration: i32,
        #[doc = "Video width and height; as defined by the sender"]
        pub length: i32,
        #[serde(default)]
        #[doc = "Video minithumbnail; may be null"]
        pub minithumbnail: Option<Minithumbnail>,
        #[serde(default)]
        #[doc = "Video thumbnail; as defined by the sender; may be null"]
        pub thumbnail: Option<PhotoSize>,
        #[doc = "File containing the video"]
        pub video: File,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a voice note. The voice note must be encoded with the Opus codec, and stored inside an OGG container. Voice notes can have only a single audio channel"]
    pub struct VoiceNote {
        #[doc = "Duration of the voice note, in seconds; as defined by the sender"]
        pub duration: i32,
        #[doc = "A waveform representation of the voice note in 5-bit format"]
        pub waveform: String,
        #[doc = "MIME type of the file; as defined by the sender"]
        pub mime_type: String,
        #[doc = "File containing the voice note"]
        pub voice: File,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a user contact"]
    pub struct Contact {
        #[doc = "Phone number of the user"]
        pub phone_number: String,
        #[doc = "First name of the user; 1-255 characters in length"]
        pub first_name: String,
        #[doc = "Last name of the user"]
        pub last_name: String,
        #[doc = "Additional data about the user in a form of vCard; 0-2048 bytes in length"]
        pub vcard: String,
        #[doc = "Identifier of the user, if known; otherwise 0"]
        pub user_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a location on planet Earth"]
    pub struct Location {
        #[doc = "Latitude of the location in degrees; as defined by the sender"]
        pub latitude: f64,
        #[doc = "Longitude of the location, in degrees; as defined by the sender"]
        pub longitude: f64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a venue"]
    pub struct Venue {
        #[doc = "Venue location; as defined by the sender"]
        pub location: Location,
        #[doc = "Venue name; as defined by the sender"]
        pub title: String,
        #[doc = "Venue address; as defined by the sender"]
        pub address: String,
        #[doc = "Provider of the venue database; as defined by the sender. Currently only \"foursquare\" needs to be supported"]
        pub provider: String,
        #[doc = "Identifier of the venue in the provider database; as defined by the sender"]
        pub id: String,
        #[serde(rename = "type")]
        #[doc = "Type of the venue in the provider database; as defined by the sender"]
        pub type_: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a game"]
    pub struct Game {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Game ID"]
        pub id: i64,
        #[doc = "Game short name. To share a game use the URL https://t.me/{bot_username}?game={game_short_name}"]
        pub short_name: String,
        #[doc = "Game title"]
        pub title: String,
        #[doc = "Game text, usually containing scoreboards for a game"]
        pub text: FormattedText,
        #[doc = "Describes a game"]
        pub description: String,
        #[doc = "Game photo"]
        pub photo: Photo,
        #[serde(default)]
        #[doc = "Game animation; may be null"]
        pub animation: Option<Animation>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a poll"]
    pub struct Poll {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique poll identifier"]
        pub id: i64,
        #[doc = "Poll question, 1-255 characters"]
        pub question: String,
        #[doc = "List of poll answer options"]
        pub options: Vec<PollOption>,
        #[doc = "Total number of voters, participating in the poll"]
        pub total_voter_count: i32,
        #[doc = "User identifiers of recent voters, if the poll is non-anonymous"]
        pub recent_voter_user_ids: Vec<i32>,
        #[doc = "True, if the poll is anonymous"]
        pub is_anonymous: bool,
        #[serde(rename = "type")]
        #[doc = "Type of the poll"]
        pub type_: PollType,
        #[doc = "True, if the poll is closed"]
        pub is_closed: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a user profile photo"]
    pub struct ProfilePhoto {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Photo identifier; 0 for an empty photo. Can be used to find a photo in a list of userProfilePhotos"]
        pub id: i64,
        #[doc = "A small (160x160) user profile photo. The file can be downloaded only before the photo is changed"]
        pub small: File,
        #[doc = "A big (640x640) user profile photo. The file can be downloaded only before the photo is changed"]
        pub big: File,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes the photo of a chat"]
    pub struct ChatPhoto {
        #[doc = "A small (160x160) chat photo. The file can be downloaded only before the photo is changed"]
        pub small: File,
        #[doc = "A big (640x640) chat photo. The file can be downloaded only before the photo is changed"]
        pub big: File,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A regular user"]
    pub struct UserTypeRegular {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A deleted user or deleted bot. No information on the user besides the user identifier is available. It is not possible to perform any active actions on this type of user"]
    pub struct UserTypeDeleted {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A bot (see https://core.telegram.org/bots)"]
    pub struct UserTypeBot {
        #[doc = "True, if the bot can be invited to basic group and supergroup chats"]
        pub can_join_groups: bool,
        #[doc = "True, if the bot can read all messages in basic group or supergroup chats and not just those addressed to the bot. In private and channel chats a bot can always read all messages"]
        pub can_read_all_group_messages: bool,
        #[doc = "True, if the bot supports inline queries"]
        pub is_inline: bool,
        #[doc = "Placeholder for inline queries (displayed on the client input field)"]
        pub inline_query_placeholder: String,
        #[doc = "True, if the location of the user should be sent with every inline query to this bot"]
        pub need_location: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "No information on the user besides the user identifier is available, yet this user has not been deleted. This object is extremely rare and must be handled like a deleted user. It is not possible to perform any actions on users of this type"]
    pub struct UserTypeUnknown {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents the type of a user. The following types are possible: regular users, deleted users and bots"]
    pub enum UserType {
        UserTypeRegular(UserTypeRegular),
        UserTypeDeleted(UserTypeDeleted),
        UserTypeBot(UserTypeBot),
        UserTypeUnknown(UserTypeUnknown),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents commands supported by a bot"]
    pub struct BotCommand {
        #[doc = "Text of the bot command"]
        pub command: String,
        #[doc = "Represents commands supported by a bot"]
        pub description: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Provides information about a bot and its supported commands"]
    pub struct BotInfo {
        #[doc = "Provides information about a bot and its supported commands"]
        pub description: String,
        #[doc = "A list of commands supported by the bot"]
        pub commands: Vec<BotCommand>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a location to which a chat is connected"]
    pub struct ChatLocation {
        #[doc = "The location"]
        pub location: Location,
        #[doc = "Location address; 1-64 characters, as defined by the chat owner"]
        pub address: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a user"]
    pub struct User {
        #[doc = "User identifier"]
        pub id: i32,
        #[doc = "First name of the user"]
        pub first_name: String,
        #[doc = "Last name of the user"]
        pub last_name: String,
        #[doc = "Username of the user"]
        pub username: String,
        #[doc = "Phone number of the user"]
        pub phone_number: String,
        #[doc = "Current online status of the user"]
        pub status: UserStatus,
        #[serde(default)]
        #[doc = "Profile photo of the user; may be null"]
        pub profile_photo: Option<ProfilePhoto>,
        #[doc = "The user is a contact of the current user"]
        pub is_contact: bool,
        #[doc = "The user is a contact of the current user and the current user is a contact of the user"]
        pub is_mutual_contact: bool,
        #[doc = "True, if the user is verified"]
        pub is_verified: bool,
        #[doc = "True, if the user is Telegram support account"]
        pub is_support: bool,
        #[doc = "If non-empty, it contains a human-readable description of the reason why access to this user must be restricted"]
        pub restriction_reason: String,
        #[doc = "True, if many users reported this user as a scam"]
        pub is_scam: bool,
        #[doc = "If false, the user is inaccessible, and the only information known about the user is inside this class. It can't be passed to any method except GetUser"]
        pub have_access: bool,
        #[serde(rename = "type")]
        #[doc = "Type of the user"]
        pub type_: UserType,
        #[serde(default)]
        #[doc = "IETF language tag of the user's language; only available to bots"]
        pub language_code: Option<String>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains full information about a user (except the full list of profile photos)"]
    pub struct UserFullInfo {
        #[doc = "True, if the user is blacklisted by the current user"]
        pub is_blocked: bool,
        #[doc = "True, if the user can be called"]
        pub can_be_called: bool,
        #[doc = "True, if the user can't be called due to their privacy settings"]
        pub has_private_calls: bool,
        #[doc = "True, if the current user needs to explicitly allow to share their phone number with the user when the method addContact is used"]
        pub need_phone_number_privacy_exception: bool,
        #[doc = "A short user bio"]
        pub bio: String,
        #[doc = "For bots, the text that is included with the link when users share the bot"]
        pub share_text: String,
        #[doc = "Number of group chats where both the other user and the current user are a member; 0 for the current user"]
        pub group_in_common_count: i32,
        #[serde(default)]
        #[doc = "If the user is a bot, information about the bot; may be null"]
        pub bot_info: Option<BotInfo>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains full information about a user profile photo"]
    pub struct UserProfilePhoto {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique user profile photo identifier"]
        pub id: i64,
        #[doc = "Point in time (Unix timestamp) when the photo has been added"]
        pub added_date: i32,
        #[doc = "Available variants of the user photo, in different sizes"]
        pub sizes: Vec<PhotoSize>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains part of the list of user photos"]
    pub struct UserProfilePhotos {
        #[doc = "Total number of user profile photos"]
        pub total_count: i32,
        #[doc = "A list of photos"]
        pub photos: Vec<UserProfilePhoto>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a list of users"]
    pub struct Users {
        #[doc = "Approximate total count of users found"]
        pub total_count: i32,
        #[doc = "A list of user identifiers"]
        pub user_ids: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a chat administrator"]
    pub struct ChatAdministrator {
        #[doc = "User identifier of the administrator"]
        pub user_id: i32,
        #[doc = "Custom title of the administrator"]
        pub custom_title: String,
        #[doc = "True, if the user is the owner of the chat"]
        pub is_owner: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a list of chat administrators"]
    pub struct ChatAdministrators {
        #[doc = "A list of chat administrators"]
        pub administrators: Vec<ChatAdministrator>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes actions that a user is allowed to take in a chat"]
    pub struct ChatPermissions {
        #[doc = "True, if the user can send text messages, contacts, locations, and venues"]
        pub can_send_messages: bool,
        #[doc = "True, if the user can send audio files, documents, photos, videos, video notes, and voice notes. Implies can_send_messages permissions"]
        pub can_send_media_messages: bool,
        #[doc = "True, if the user can send polls. Implies can_send_messages permissions"]
        pub can_send_polls: bool,
        #[doc = "True, if the user can send animations, games, and stickers and use inline bots. Implies can_send_messages permissions"]
        pub can_send_other_messages: bool,
        #[doc = "True, if the user may add a web page preview to their messages. Implies can_send_messages permissions"]
        pub can_add_web_page_previews: bool,
        #[doc = "True, if the user can change the chat title, photo, and other settings"]
        pub can_change_info: bool,
        #[doc = "True, if the user can invite new users to the chat"]
        pub can_invite_users: bool,
        #[doc = "True, if the user can pin messages"]
        pub can_pin_messages: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is the owner of a chat and has all the administrator privileges"]
    pub struct ChatMemberStatusCreator {
        #[doc = "A custom title of the owner; 0-16 characters without emojis; applicable to supergroups only"]
        pub custom_title: String,
        #[doc = "True, if the user is a member of the chat"]
        pub is_member: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is a member of a chat and has some additional privileges. In basic groups, administrators can edit and delete messages sent by others, add new members, and ban unprivileged members. In supergroups and channels, there are more detailed options for administrator privileges"]
    pub struct ChatMemberStatusAdministrator {
        #[doc = "A custom title of the administrator; 0-16 characters without emojis; applicable to supergroups only"]
        pub custom_title: String,
        #[doc = "True, if the current user can edit the administrator privileges for the called user"]
        pub can_be_edited: bool,
        #[doc = "True, if the administrator can change the chat title, photo, and other settings"]
        pub can_change_info: bool,
        #[doc = "True, if the administrator can create channel posts; applicable to channels only"]
        pub can_post_messages: bool,
        #[doc = "True, if the administrator can edit messages of other users and pin messages; applicable to channels only"]
        pub can_edit_messages: bool,
        #[doc = "True, if the administrator can delete messages of other users"]
        pub can_delete_messages: bool,
        #[doc = "True, if the administrator can invite new users to the chat"]
        pub can_invite_users: bool,
        #[doc = "True, if the administrator can restrict, ban, or unban chat members"]
        pub can_restrict_members: bool,
        #[doc = "True, if the administrator can pin messages; applicable to groups only"]
        pub can_pin_messages: bool,
        #[doc = "True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that were directly or indirectly promoted by them"]
        pub can_promote_members: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is a member of a chat, without any additional privileges or restrictions"]
    pub struct ChatMemberStatusMember {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is under certain restrictions in the chat. Not supported in basic groups and channels"]
    pub struct ChatMemberStatusRestricted {
        #[doc = "True, if the user is a member of the chat"]
        pub is_member: bool,
        #[doc = "Point in time (Unix timestamp) when restrictions will be lifted from the user; 0 if never. If the user is restricted for more than 366 days or for less than 30 seconds from the current time, the user is considered to be restricted forever"]
        pub restricted_until_date: i32,
        #[doc = "User permissions in the chat"]
        pub permissions: ChatPermissions,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is not a chat member"]
    pub struct ChatMemberStatusLeft {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user was banned (and hence is not a member of the chat). Implies the user can't return to the chat or view messages"]
    pub struct ChatMemberStatusBanned {
        #[doc = "Point in time (Unix timestamp) when the user will be unbanned; 0 if never. If the user is banned for more than 366 days or for less than 30 seconds from the current time, the user is considered to be banned forever"]
        pub banned_until_date: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Provides information about the status of a member in a chat"]
    pub enum ChatMemberStatus {
        ChatMemberStatusCreator(ChatMemberStatusCreator),
        ChatMemberStatusAdministrator(ChatMemberStatusAdministrator),
        ChatMemberStatusMember(ChatMemberStatusMember),
        ChatMemberStatusRestricted(ChatMemberStatusRestricted),
        ChatMemberStatusLeft(ChatMemberStatusLeft),
        ChatMemberStatusBanned(ChatMemberStatusBanned),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A user with information about joining/leaving a chat"]
    pub struct ChatMember {
        #[doc = "User identifier of the chat member"]
        pub user_id: i32,
        #[doc = "Identifier of a user that invited/promoted/banned this member in the chat; 0 if unknown"]
        pub inviter_user_id: i32,
        #[doc = "Point in time (Unix timestamp) when the user joined a chat"]
        pub joined_chat_date: i32,
        #[doc = "Status of the member in the chat"]
        pub status: ChatMemberStatus,
        #[serde(default)]
        #[doc = "If the user is a bot, information about the bot; may be null. Can be null even for a bot if the bot is not a chat member"]
        pub bot_info: Option<BotInfo>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of chat members"]
    pub struct ChatMembers {
        #[doc = "Approximate total count of chat members found"]
        pub total_count: i32,
        #[doc = "A list of chat members"]
        pub members: Vec<ChatMember>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns contacts of the user"]
    pub struct ChatMembersFilterContacts {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns the owner and administrators"]
    pub struct ChatMembersFilterAdministrators {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns all chat members, including restricted chat members"]
    pub struct ChatMembersFilterMembers {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns users under certain restrictions in the chat; can be used only by administrators in a supergroup"]
    pub struct ChatMembersFilterRestricted {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns users banned from the chat; can be used only by administrators in a supergroup or in a channel"]
    pub struct ChatMembersFilterBanned {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns bot members of the chat"]
    pub struct ChatMembersFilterBots {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Specifies the kind of chat members to return in searchChatMembers"]
    pub enum ChatMembersFilter {
        ChatMembersFilterContacts(ChatMembersFilterContacts),
        ChatMembersFilterAdministrators(ChatMembersFilterAdministrators),
        ChatMembersFilterMembers(ChatMembersFilterMembers),
        ChatMembersFilterRestricted(ChatMembersFilterRestricted),
        ChatMembersFilterBanned(ChatMembersFilterBanned),
        ChatMembersFilterBots(ChatMembersFilterBots),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns recently active users in reverse chronological order"]
    pub struct SupergroupMembersFilterRecent {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns contacts of the user, which are members of the supergroup or channel"]
    pub struct SupergroupMembersFilterContacts {
        #[doc = "Query to search for"]
        pub query: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns the owner and administrators"]
    pub struct SupergroupMembersFilterAdministrators {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Used to search for supergroup or channel members via a (string) query"]
    pub struct SupergroupMembersFilterSearch {
        #[doc = "Query to search for"]
        pub query: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns restricted supergroup members; can be used only by administrators"]
    pub struct SupergroupMembersFilterRestricted {
        #[doc = "Query to search for"]
        pub query: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns users banned from the supergroup or channel; can be used only by administrators"]
    pub struct SupergroupMembersFilterBanned {
        #[doc = "Query to search for"]
        pub query: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns bot members of the supergroup or channel"]
    pub struct SupergroupMembersFilterBots {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Specifies the kind of chat members to return in getSupergroupMembers"]
    pub enum SupergroupMembersFilter {
        SupergroupMembersFilterRecent(SupergroupMembersFilterRecent),
        SupergroupMembersFilterContacts(SupergroupMembersFilterContacts),
        SupergroupMembersFilterAdministrators(SupergroupMembersFilterAdministrators),
        SupergroupMembersFilterSearch(SupergroupMembersFilterSearch),
        SupergroupMembersFilterRestricted(SupergroupMembersFilterRestricted),
        SupergroupMembersFilterBanned(SupergroupMembersFilterBanned),
        SupergroupMembersFilterBots(SupergroupMembersFilterBots),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)"]
    pub struct BasicGroup {
        #[doc = "Group identifier"]
        pub id: i32,
        #[doc = "Number of members in the group"]
        pub member_count: i32,
        #[doc = "Status of the current user in the group"]
        pub status: ChatMemberStatus,
        #[doc = "True, if the group is active"]
        pub is_active: bool,
        #[doc = "Identifier of the supergroup to which this group was upgraded; 0 if none"]
        pub upgraded_to_supergroup_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains full information about a basic group"]
    pub struct BasicGroupFullInfo {
        #[doc = "Contains full information about a basic group"]
        pub description: String,
        #[doc = "User identifier of the creator of the group; 0 if unknown"]
        pub creator_user_id: i32,
        #[doc = "Group members"]
        pub members: Vec<ChatMember>,
        #[doc = "Invite link for this group; available only after it has been generated at least once and only for the group creator"]
        pub invite_link: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a supergroup or channel with zero or more members (subscribers in the case of channels). From the point of view of the system, a channel is a special kind of a supergroup: only administrators can post and see the list of members, and posts from all administrators use the name and photo of the channel instead of individual names and profile photos. Unlike supergroups, channels can have an unlimited number of subscribers"]
    pub struct Supergroup {
        #[doc = "Supergroup or channel identifier"]
        pub id: i32,
        #[doc = "Username of the supergroup or channel; empty for private supergroups or channels"]
        pub username: String,
        #[doc = "Point in time (Unix timestamp) when the current user joined, or the point in time when the supergroup or channel was created, in case the user is not a member"]
        pub date: i32,
        #[doc = "Status of the current user in the supergroup or channel; custom title will be always empty"]
        pub status: ChatMemberStatus,
        #[doc = "Member count; 0 if unknown. Currently it is guaranteed to be known only if the supergroup or channel was found through SearchPublicChats"]
        pub member_count: i32,
        #[doc = "True, if the channel has a discussion group, or the supergroup is the designated discussion group for a channel"]
        pub has_linked_chat: bool,
        #[doc = "True, if the supergroup is connected to a location, i.e. the supergroup is a location-based supergroup"]
        pub has_location: bool,
        #[doc = "True, if messages sent to the channel should contain information about the sender. This field is only applicable to channels"]
        pub sign_messages: bool,
        #[doc = "True, if the slow mode is enabled in the supergroup"]
        pub is_slow_mode_enabled: bool,
        #[doc = "True, if the supergroup is a channel"]
        pub is_channel: bool,
        #[doc = "True, if the supergroup or channel is verified"]
        pub is_verified: bool,
        #[doc = "If non-empty, contains a human-readable description of the reason why access to this supergroup or channel must be restricted"]
        pub restriction_reason: String,
        #[doc = "True, if many users reported this supergroup as a scam"]
        pub is_scam: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains full information about a supergroup or channel"]
    pub struct SupergroupFullInfo {
        #[doc = "Contains full information about a supergroup or channel"]
        pub description: String,
        #[doc = "Number of members in the supergroup or channel; 0 if unknown"]
        pub member_count: i32,
        #[doc = "Number of privileged users in the supergroup or channel; 0 if unknown"]
        pub administrator_count: i32,
        #[doc = "Number of restricted users in the supergroup; 0 if unknown"]
        pub restricted_count: i32,
        #[doc = "Number of users banned from chat; 0 if unknown"]
        pub banned_count: i32,
        #[doc = "Chat identifier of a discussion group for the channel, or a channel, for which the supergroup is the designated discussion group; 0 if none or unknown"]
        pub linked_chat_id: i64,
        #[doc = "Delay between consecutive sent messages for non-administrator supergroup members, in seconds"]
        pub slow_mode_delay: i32,
        #[doc = "Time left before next message can be sent in the supergroup, in seconds. An updateSupergroupFullInfo update is not triggered when value of this field changes, but both new and old values are non-zero"]
        pub slow_mode_delay_expires_in: f64,
        #[doc = "True, if members of the chat can be retrieved"]
        pub can_get_members: bool,
        #[doc = "True, if the chat username can be changed"]
        pub can_set_username: bool,
        #[doc = "True, if the supergroup sticker set can be changed"]
        pub can_set_sticker_set: bool,
        #[doc = "True, if the supergroup location can be changed"]
        pub can_set_location: bool,
        #[doc = "True, if the channel statistics is available through getChatStatisticsUrl"]
        pub can_view_statistics: bool,
        #[doc = "True, if new chat members will have access to old messages. In public or discussion groups and both public and private channels, old messages are always available, so this option affects only private supergroups without a linked chat. The value of this field is only available for chat administrators"]
        pub is_all_history_available: bool,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the supergroup sticker set; 0 if none"]
        pub sticker_set_id: i64,
        #[serde(default)]
        #[doc = "Location to which the supergroup is connected; may be null"]
        pub location: Option<ChatLocation>,
        #[doc = "Invite link for this chat"]
        pub invite_link: String,
        #[doc = "Identifier of the basic group from which supergroup was upgraded; 0 if none"]
        pub upgraded_from_basic_group_id: i32,
        #[doc = "Identifier of the last message in the basic group from which supergroup was upgraded; 0 if none"]
        pub upgraded_from_max_message_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The secret chat is not yet created; waiting for the other user to get online"]
    pub struct SecretChatStatePending {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The secret chat is ready to use"]
    pub struct SecretChatStateReady {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The secret chat is closed"]
    pub struct SecretChatStateClosed {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the current secret chat state"]
    pub enum SecretChatState {
        SecretChatStatePending(SecretChatStatePending),
        SecretChatStateReady(SecretChatStateReady),
        SecretChatStateClosed(SecretChatStateClosed),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a secret chat"]
    pub struct SecretChat {
        #[doc = "Secret chat identifier"]
        pub id: i32,
        #[doc = "Identifier of the chat partner"]
        pub user_id: i32,
        #[doc = "State of the secret chat"]
        pub state: SecretChatState,
        #[doc = "True, if the chat was created by the current user; otherwise false"]
        pub is_outbound: bool,
        #[doc = "Current message Time To Live setting (self-destruct timer) for the chat, in seconds"]
        pub ttl: i32,
        #[doc = "Hash of the currently used key for comparison with the hash of the chat partner's key. This is a string of 36 little-endian bytes, which must be split into groups of 2 bits, each denoting a pixel of one of 4 colors FFFFFF, D5E6F3, 2D5775, and 2F99C9. The pixels must be used to make a 12x12 square image filled from left to right, top to bottom. Alternatively, the first 32 bytes of the hash can be converted to the hexadecimal format and printed as 32 2-digit hex numbers"]
        pub key_hash: String,
        #[doc = "Secret chat layer; determines features supported by the other client. Video notes are supported if the layer >= 66; nested text entities and underline and strikethrough entities are supported if the layer >= 101"]
        pub layer: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The message was originally written by a known user"]
    pub struct MessageForwardOriginUser {
        #[doc = "Identifier of the user that originally sent the message"]
        pub sender_user_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The message was originally written by a user, which is hidden by their privacy settings"]
    pub struct MessageForwardOriginHiddenUser {
        #[doc = "Name of the sender"]
        pub sender_name: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The message was originally a post in a channel"]
    pub struct MessageForwardOriginChannel {
        #[doc = "Identifier of the chat from which the message was originally forwarded"]
        pub chat_id: i64,
        #[doc = "Message identifier of the original message; 0 if unknown"]
        pub message_id: i64,
        #[doc = "Original post author signature"]
        pub author_signature: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains information about the origin of a forwarded message"]
    pub enum MessageForwardOrigin {
        MessageForwardOriginUser(MessageForwardOriginUser),
        MessageForwardOriginHiddenUser(MessageForwardOriginHiddenUser),
        MessageForwardOriginChannel(MessageForwardOriginChannel),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a forwarded message"]
    pub struct MessageForwardInfo {
        #[doc = "Origin of a forwarded message"]
        pub origin: MessageForwardOrigin,
        #[doc = "Point in time (Unix timestamp) when the message was originally sent"]
        pub date: i32,
        #[doc = "For messages forwarded to the chat with the current user (Saved Messages) or to the channel's discussion group, the identifier of the chat from which the message was forwarded last time; 0 if unknown"]
        pub from_chat_id: i64,
        #[doc = "For messages forwarded to the chat with the current user (Saved Messages) or to the channel's discussion group, the identifier of the original message from which the new message was forwarded last time; 0 if unknown"]
        pub from_message_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The message is being sent now, but has not yet been delivered to the server"]
    pub struct MessageSendingStatePending {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The message failed to be sent"]
    pub struct MessageSendingStateFailed {
        #[doc = "An error code; 0 if unknown"]
        pub error_code: i32,
        #[doc = "Error message"]
        pub error_message: String,
        #[doc = "True, if the message can be re-sent"]
        pub can_retry: bool,
        #[doc = "Time left before the message can be re-sent, in seconds. No update is sent when this field changes"]
        pub retry_after: f64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains information about the sending state of the message"]
    pub enum MessageSendingState {
        MessageSendingStatePending(MessageSendingStatePending),
        MessageSendingStateFailed(MessageSendingStateFailed),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a message"]
    pub struct Message {
        #[doc = "Message identifier, unique for the chat to which the message belongs"]
        pub id: i64,
        #[doc = "Identifier of the user who sent the message; 0 if unknown. Currently, it is unknown for channel posts and for channel posts automatically forwarded to discussion group"]
        pub sender_user_id: i32,
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[serde(default)]
        #[doc = "Information about the sending state of the message; may be null"]
        pub sending_state: Option<MessageSendingState>,
        #[serde(default)]
        #[doc = "Information about the scheduling state of the message; may be null"]
        pub scheduling_state: Option<MessageSchedulingState>,
        #[doc = "True, if the message is outgoing"]
        pub is_outgoing: bool,
        #[doc = "True, if the message can be edited. For live location and poll messages this fields shows whether editMessageLiveLocation or stopPoll can be used with this message by the client"]
        pub can_be_edited: bool,
        #[doc = "True, if the message can be forwarded"]
        pub can_be_forwarded: bool,
        #[doc = "True, if the message can be deleted only for the current user while other users will continue to see it"]
        pub can_be_deleted_only_for_self: bool,
        #[doc = "True, if the message can be deleted for all users"]
        pub can_be_deleted_for_all_users: bool,
        #[doc = "True, if the message is a channel post. All messages to channels are channel posts, all other messages are not channel posts"]
        pub is_channel_post: bool,
        #[doc = "True, if the message contains an unread mention for the current user"]
        pub contains_unread_mention: bool,
        #[doc = "Point in time (Unix timestamp) when the message was sent"]
        pub date: i32,
        #[doc = "Point in time (Unix timestamp) when the message was last edited"]
        pub edit_date: i32,
        #[serde(default)]
        #[doc = "Information about the initial message sender; may be null"]
        pub forward_info: Option<MessageForwardInfo>,
        #[doc = "If non-zero, the identifier of the message this message is replying to; can be the identifier of a deleted message"]
        pub reply_to_message_id: i64,
        #[doc = "For self-destructing messages, the message's TTL (Time To Live), in seconds; 0 if none. TDLib will send updateDeleteMessages or updateMessageContent once the TTL expires"]
        pub ttl: i32,
        #[doc = "Time left before the message expires, in seconds"]
        pub ttl_expires_in: f64,
        #[doc = "If non-zero, the user identifier of the bot through which this message was sent"]
        pub via_bot_user_id: i32,
        #[doc = "For channel posts, optional author signature"]
        pub author_signature: String,
        #[doc = "Number of times this message was viewed"]
        pub views: i32,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique identifier of an album this message belongs to. Only photos and videos can be grouped together in albums"]
        pub media_album_id: i64,
        #[doc = "If non-empty, contains a human-readable description of the reason why access to this message must be restricted"]
        pub restriction_reason: String,
        #[doc = "Content of the message"]
        pub content: MessageContent,
        #[serde(default)]
        #[doc = "Reply markup for the message; may be null"]
        pub reply_markup: Option<ReplyMarkup>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of messages"]
    pub struct Messages {
        #[doc = "Approximate total count of messages found"]
        pub total_count: i32,
        #[serde(default)]
        #[doc = "List of messages; messages may be null"]
        pub messages: Option<Vec<Message>>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of messages found by a search"]
    pub struct FoundMessages {
        #[doc = "List of messages"]
        pub messages: Vec<Message>,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Value to pass as from_search_id to get more results"]
        pub next_from_search_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Notification settings applied to all private and secret chats when the corresponding chat setting has a default value"]
    pub struct NotificationSettingsScopePrivateChats {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Notification settings applied to all basic groups and supergroups when the corresponding chat setting has a default value"]
    pub struct NotificationSettingsScopeGroupChats {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Notification settings applied to all channels when the corresponding chat setting has a default value"]
    pub struct NotificationSettingsScopeChannelChats {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the types of chats to which notification settings are applied"]
    pub enum NotificationSettingsScope {
        NotificationSettingsScopePrivateChats(NotificationSettingsScopePrivateChats),
        NotificationSettingsScopeGroupChats(NotificationSettingsScopeGroupChats),
        NotificationSettingsScopeChannelChats(NotificationSettingsScopeChannelChats),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about notification settings for a chat"]
    pub struct ChatNotificationSettings {
        #[doc = "If true, mute_for is ignored and the value for the relevant type of chat is used instead"]
        pub use_default_mute_for: bool,
        #[doc = "Time left before notifications will be unmuted, in seconds"]
        pub mute_for: i32,
        #[doc = "If true, sound is ignored and the value for the relevant type of chat is used instead"]
        pub use_default_sound: bool,
        #[doc = "The name of an audio file to be used for notification sounds; only applies to iOS applications"]
        pub sound: String,
        #[doc = "If true, show_preview is ignored and the value for the relevant type of chat is used instead"]
        pub use_default_show_preview: bool,
        #[doc = "True, if message content should be displayed in notifications"]
        pub show_preview: bool,
        #[doc = "If true, disable_pinned_message_notifications is ignored and the value for the relevant type of chat is used instead"]
        pub use_default_disable_pinned_message_notifications: bool,
        #[doc = "If true, notifications for incoming pinned messages will be created as for an ordinary unread message"]
        pub disable_pinned_message_notifications: bool,
        #[doc = "If true, disable_mention_notifications is ignored and the value for the relevant type of chat is used instead"]
        pub use_default_disable_mention_notifications: bool,
        #[doc = "If true, notifications for messages with mentions will be created as for an ordinary unread message"]
        pub disable_mention_notifications: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about notification settings for several chats"]
    pub struct ScopeNotificationSettings {
        #[doc = "Time left before notifications will be unmuted, in seconds"]
        pub mute_for: i32,
        #[doc = "The name of an audio file to be used for notification sounds; only applies to iOS applications"]
        pub sound: String,
        #[doc = "True, if message content should be displayed in notifications"]
        pub show_preview: bool,
        #[doc = "True, if notifications for incoming pinned messages will be created as for an ordinary unread message"]
        pub disable_pinned_message_notifications: bool,
        #[doc = "True, if notifications for messages with mentions will be created as for an ordinary unread message"]
        pub disable_mention_notifications: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a message draft"]
    pub struct DraftMessage {
        #[doc = "Identifier of the message to reply to; 0 if none"]
        pub reply_to_message_id: i64,
        #[doc = "Content of the message draft; this should always be of type inputMessageText"]
        pub input_message_text: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An ordinary chat with a user"]
    pub struct ChatTypePrivate {
        #[doc = "User identifier"]
        pub user_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A basic group (i.e., a chat with 0-200 other users)"]
    pub struct ChatTypeBasicGroup {
        #[doc = "Basic group identifier"]
        pub basic_group_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A supergroup (i.e. a chat with up to GetOption(\"supergroup_max_size\") other users), or channel (with unlimited members)"]
    pub struct ChatTypeSupergroup {
        #[doc = "Supergroup or channel identifier"]
        pub supergroup_id: i32,
        #[doc = "True, if the supergroup is a channel"]
        pub is_channel: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A secret chat with a user"]
    pub struct ChatTypeSecret {
        #[doc = "Secret chat identifier"]
        pub secret_chat_id: i32,
        #[doc = "User identifier of the secret chat peer"]
        pub user_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the type of a chat"]
    pub enum ChatType {
        ChatTypePrivate(ChatTypePrivate),
        ChatTypeBasicGroup(ChatTypeBasicGroup),
        ChatTypeSupergroup(ChatTypeSupergroup),
        ChatTypeSecret(ChatTypeSecret),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A main list of chats"]
    pub struct ChatListMain {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A list of chats usually located at the top of the main chat list. Unmuted chats are automatically moved from the Archive to the Main chat list when a new message arrives"]
    pub struct ChatListArchive {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes a list of chats"]
    pub enum ChatList {
        ChatListMain(ChatListMain),
        ChatListArchive(ChatListArchive),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat. (Can be a private chat, basic group, supergroup, or secret chat)"]
    pub struct Chat {
        #[doc = "Chat unique identifier"]
        pub id: i64,
        #[serde(rename = "type")]
        #[doc = "Type of the chat"]
        pub type_: ChatType,
        #[serde(default)]
        #[doc = "A chat list to which the chat belongs; may be null"]
        pub chat_list: Option<ChatList>,
        #[doc = "Chat title"]
        pub title: String,
        #[serde(default)]
        #[doc = "Chat photo; may be null"]
        pub photo: Option<ChatPhoto>,
        #[doc = "Actions that non-administrator chat members are allowed to take in the chat"]
        pub permissions: ChatPermissions,
        #[serde(default)]
        #[doc = "Last message in the chat; may be null"]
        pub last_message: Option<Message>,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Descending parameter by which chats are sorted in the main chat list. If the order number of two chats is the same, they must be sorted in descending order by ID. If 0, the position of the chat in the list is undetermined"]
        pub order: i64,
        #[doc = "True, if the chat is pinned"]
        pub is_pinned: bool,
        #[doc = "True, if the chat is marked as unread"]
        pub is_marked_as_unread: bool,
        #[doc = "True, if the chat is sponsored by the user's MTProxy server"]
        pub is_sponsored: bool,
        #[doc = "True, if the chat has scheduled messages"]
        pub has_scheduled_messages: bool,
        #[doc = "True, if the chat messages can be deleted only for the current user while other users will continue to see the messages"]
        pub can_be_deleted_only_for_self: bool,
        #[doc = "True, if the chat messages can be deleted for all users"]
        pub can_be_deleted_for_all_users: bool,
        #[doc = "True, if the chat can be reported to Telegram moderators through reportChat"]
        pub can_be_reported: bool,
        #[doc = "Default value of the disable_notification parameter, used when a message is sent to the chat"]
        pub default_disable_notification: bool,
        #[doc = "Number of unread messages in the chat"]
        pub unread_count: i32,
        #[doc = "Identifier of the last read incoming message"]
        pub last_read_inbox_message_id: i64,
        #[doc = "Identifier of the last read outgoing message"]
        pub last_read_outbox_message_id: i64,
        #[doc = "Number of unread messages with a mention/reply in the chat"]
        pub unread_mention_count: i32,
        #[doc = "Notification settings for this chat"]
        pub notification_settings: ChatNotificationSettings,
        #[serde(default)]
        #[doc = "Describes actions which should be possible to do through a chat action bar; may be null"]
        pub action_bar: Option<ChatActionBar>,
        #[doc = "Identifier of the pinned message in the chat; 0 if none"]
        pub pinned_message_id: i64,
        #[doc = "Identifier of the message from which reply markup needs to be used; 0 if there is no default custom reply markup in the chat"]
        pub reply_markup_message_id: i64,
        #[serde(default)]
        #[doc = "A draft of a message in the chat; may be null"]
        pub draft_message: Option<DraftMessage>,
        #[doc = "Contains client-specific data associated with the chat. (For example, the chat position or local chat notification settings can be stored here.) Persistent if the message database is used"]
        pub client_data: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a list of chats"]
    pub struct Chats {
        #[doc = "List of chat identifiers"]
        pub chat_ids: Vec<i64>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a chat located nearby"]
    pub struct ChatNearby {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Distance to the chat location in meters"]
        pub distance: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a list of chats located nearby"]
    pub struct ChatsNearby {
        #[doc = "List of users nearby"]
        pub users_nearby: Vec<ChatNearby>,
        #[doc = "List of location-based supergroups nearby"]
        pub supergroups_nearby: Vec<ChatNearby>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a chat invite link"]
    pub struct ChatInviteLink {
        #[doc = "Chat invite link"]
        pub invite_link: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a chat invite link"]
    pub struct ChatInviteLinkInfo {
        #[doc = "Chat identifier of the invite link; 0 if the user is not a member of this chat"]
        pub chat_id: i64,
        #[serde(rename = "type")]
        #[doc = "Contains information about the type of the chat"]
        pub type_: ChatType,
        #[doc = "Title of the chat"]
        pub title: String,
        #[serde(default)]
        #[doc = "Chat photo; may be null"]
        pub photo: Option<ChatPhoto>,
        #[doc = "Number of members"]
        pub member_count: i32,
        #[doc = "User identifiers of some chat members that may be known to the current user"]
        pub member_user_ids: Vec<i32>,
        #[doc = "True, if the chat is a public supergroup or channel, i.e. it has a username or it is a location-based supergroup"]
        pub is_public: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat is public, because it has username"]
    pub struct PublicChatTypeHasUsername {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat is public, because it is a location-based supergroup"]
    pub struct PublicChatTypeIsLocationBased {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes a type of public chats"]
    pub enum PublicChatType {
        PublicChatTypeHasUsername(PublicChatTypeHasUsername),
        PublicChatTypeIsLocationBased(PublicChatTypeIsLocationBased),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat can be reported as spam using the method reportChat with the reason chatReportReasonSpam"]
    pub struct ChatActionBarReportSpam {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat is a location-based supergroup, which can be reported as having unrelated location using the method reportChat with the reason chatReportReasonUnrelatedLocation"]
    pub struct ChatActionBarReportUnrelatedLocation {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be added to the contact list using the method addContact, or the other user can be blocked using the method blockUser"]
    pub struct ChatActionBarReportAddBlock {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat is a private or secret chat and the other user can be added to the contact list using the method addContact"]
    pub struct ChatActionBarAddContact {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat is a private or secret chat with a mutual contact and the user's phone number can be shared with the other user using the method sharePhoneNumber"]
    pub struct ChatActionBarSharePhoneNumber {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes actions which should be possible to do through a chat action bar"]
    pub enum ChatActionBar {
        ChatActionBarReportSpam(ChatActionBarReportSpam),
        ChatActionBarReportUnrelatedLocation(ChatActionBarReportUnrelatedLocation),
        ChatActionBarReportAddBlock(ChatActionBarReportAddBlock),
        ChatActionBarAddContact(ChatActionBarAddContact),
        ChatActionBarSharePhoneNumber(ChatActionBarSharePhoneNumber),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A simple button, with text that should be sent when the button is pressed"]
    pub struct KeyboardButtonTypeText {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A button that sends the user's phone number when pressed; available only in private chats"]
    pub struct KeyboardButtonTypeRequestPhoneNumber {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A button that sends the user's location when pressed; available only in private chats"]
    pub struct KeyboardButtonTypeRequestLocation {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A button that allows the user to create and send a poll when pressed; available only in private chats"]
    pub struct KeyboardButtonTypeRequestPoll {
        #[doc = "If true, only regular polls must be allowed to create"]
        pub force_regular: bool,
        #[doc = "If true, only polls in quiz mode must be allowed to create"]
        pub force_quiz: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes a keyboard button type"]
    pub enum KeyboardButtonType {
        KeyboardButtonTypeText(KeyboardButtonTypeText),
        KeyboardButtonTypeRequestPhoneNumber(KeyboardButtonTypeRequestPhoneNumber),
        KeyboardButtonTypeRequestLocation(KeyboardButtonTypeRequestLocation),
        KeyboardButtonTypeRequestPoll(KeyboardButtonTypeRequestPoll),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a single button in a bot keyboard"]
    pub struct KeyboardButton {
        #[doc = "Text of the button"]
        pub text: String,
        #[serde(rename = "type")]
        #[doc = "Type of the button"]
        pub type_: KeyboardButtonType,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A button that opens a specified URL"]
    pub struct InlineKeyboardButtonTypeUrl {
        #[doc = "HTTP or tg:// URL to open"]
        pub url: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A button that opens a specified URL and automatically logs in in current user if they allowed to do that"]
    pub struct InlineKeyboardButtonTypeLoginUrl {
        #[doc = "An HTTP URL to open"]
        pub url: String,
        #[doc = "Unique button identifier"]
        pub id: i32,
        #[doc = "If non-empty, new text of the button in forwarded messages"]
        pub forward_text: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A button that sends a special callback query to a bot"]
    pub struct InlineKeyboardButtonTypeCallback {
        #[doc = "Data to be sent to the bot via a callback query"]
        pub data: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A button with a game that sends a special callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame"]
    pub struct InlineKeyboardButtonTypeCallbackGame {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A button that forces an inline query to the bot to be inserted in the input field"]
    pub struct InlineKeyboardButtonTypeSwitchInline {
        #[doc = "Inline query to be sent to the bot"]
        pub query: String,
        #[doc = "True, if the inline query should be sent from the current chat"]
        pub in_current_chat: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageInvoice"]
    pub struct InlineKeyboardButtonTypeBuy {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the type of an inline keyboard button"]
    pub enum InlineKeyboardButtonType {
        InlineKeyboardButtonTypeUrl(InlineKeyboardButtonTypeUrl),
        InlineKeyboardButtonTypeLoginUrl(InlineKeyboardButtonTypeLoginUrl),
        InlineKeyboardButtonTypeCallback(InlineKeyboardButtonTypeCallback),
        InlineKeyboardButtonTypeCallbackGame(InlineKeyboardButtonTypeCallbackGame),
        InlineKeyboardButtonTypeSwitchInline(InlineKeyboardButtonTypeSwitchInline),
        InlineKeyboardButtonTypeBuy(InlineKeyboardButtonTypeBuy),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a single button in an inline keyboard"]
    pub struct InlineKeyboardButton {
        #[doc = "Text of the button"]
        pub text: String,
        #[serde(rename = "type")]
        #[doc = "Type of the button"]
        pub type_: InlineKeyboardButtonType,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Instructs clients to remove the keyboard once this message has been received. This kind of keyboard can't be received in an incoming message; instead, UpdateChatReplyMarkup with message_id == 0 will be sent"]
    pub struct ReplyMarkupRemoveKeyboard {
        #[doc = "True, if the keyboard is removed only for the mentioned users or the target user of a reply"]
        pub is_personal: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Instructs clients to force a reply to this message"]
    pub struct ReplyMarkupForceReply {
        #[doc = "True, if a forced reply must automatically be shown to the current user. For outgoing messages, specify true to show the forced reply only for the mentioned users and for the target user of a reply"]
        pub is_personal: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a custom keyboard layout to quickly reply to bots"]
    pub struct ReplyMarkupShowKeyboard {
        #[doc = "A list of rows of bot keyboard buttons"]
        pub rows: Vec<Vec<KeyboardButton>>,
        #[doc = "True, if the client needs to resize the keyboard vertically"]
        pub resize_keyboard: bool,
        #[doc = "True, if the client needs to hide the keyboard after use"]
        pub one_time: bool,
        #[doc = "True, if the keyboard must automatically be shown to the current user. For outgoing messages, specify true to show the keyboard only for the mentioned users and for the target user of a reply"]
        pub is_personal: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains an inline keyboard layout"]
    pub struct ReplyMarkupInlineKeyboard {
        #[doc = "A list of rows of inline keyboard buttons"]
        pub rows: Vec<Vec<InlineKeyboardButton>>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains a description of a custom keyboard and actions that can be done with it to quickly reply to bots"]
    pub enum ReplyMarkup {
        ReplyMarkupRemoveKeyboard(ReplyMarkupRemoveKeyboard),
        ReplyMarkupForceReply(ReplyMarkupForceReply),
        ReplyMarkupShowKeyboard(ReplyMarkupShowKeyboard),
        ReplyMarkupInlineKeyboard(ReplyMarkupInlineKeyboard),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An HTTP url needs to be open"]
    pub struct LoginUrlInfoOpen {
        #[doc = "The URL to open"]
        pub url: String,
        #[doc = "True, if there is no need to show an ordinary open URL confirm"]
        pub skip_confirm: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An authorization confirmation dialog needs to be shown to the user"]
    pub struct LoginUrlInfoRequestConfirmation {
        #[doc = "An HTTP URL to be opened"]
        pub url: String,
        #[doc = "A domain of the URL"]
        pub domain: String,
        #[doc = "User identifier of a bot linked with the website"]
        pub bot_user_id: i32,
        #[doc = "True, if the user needs to be requested to give the permission to the bot to send them messages"]
        pub request_write_access: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains information about an inline button of type inlineKeyboardButtonTypeLoginUrl"]
    pub enum LoginUrlInfo {
        LoginUrlInfoOpen(LoginUrlInfoOpen),
        LoginUrlInfoRequestConfirmation(LoginUrlInfoRequestConfirmation),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A plain text"]
    pub struct RichTextPlain {
        #[doc = "Text"]
        pub text: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A bold rich text"]
    pub struct RichTextBold {
        #[doc = "Text"]
        pub text: Box<RichText>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An italicized rich text"]
    pub struct RichTextItalic {
        #[doc = "Text"]
        pub text: Box<RichText>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An underlined rich text"]
    pub struct RichTextUnderline {
        #[doc = "Text"]
        pub text: Box<RichText>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A strikethrough rich text"]
    pub struct RichTextStrikethrough {
        #[doc = "Text"]
        pub text: Box<RichText>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A fixed-width rich text"]
    pub struct RichTextFixed {
        #[doc = "Text"]
        pub text: Box<RichText>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A rich text URL link"]
    pub struct RichTextUrl {
        #[doc = "Text"]
        pub text: Box<RichText>,
        #[doc = "URL"]
        pub url: String,
        #[doc = "True, if the URL has cached instant view server-side"]
        pub is_cached: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A rich text email link"]
    pub struct RichTextEmailAddress {
        #[doc = "Text"]
        pub text: Box<RichText>,
        #[doc = "Email address"]
        pub email_address: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A subscript rich text"]
    pub struct RichTextSubscript {
        #[doc = "Text"]
        pub text: Box<RichText>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A superscript rich text"]
    pub struct RichTextSuperscript {
        #[doc = "Text"]
        pub text: Box<RichText>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A marked rich text"]
    pub struct RichTextMarked {
        #[doc = "Text"]
        pub text: Box<RichText>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A rich text phone number"]
    pub struct RichTextPhoneNumber {
        #[doc = "Text"]
        pub text: Box<RichText>,
        #[doc = "Phone number"]
        pub phone_number: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A small image inside the text"]
    pub struct RichTextIcon {
        #[doc = "The image represented as a document. The image can be in GIF, JPEG or PNG format"]
        pub document: Document,
        #[doc = "Width of a bounding box in which the image should be shown; 0 if unknown"]
        pub width: i32,
        #[doc = "Height of a bounding box in which the image should be shown; 0 if unknown"]
        pub height: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A rich text anchor"]
    pub struct RichTextAnchor {
        #[doc = "Text"]
        pub text: Box<RichText>,
        #[doc = "Anchor name"]
        pub name: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A concatenation of rich texts"]
    pub struct RichTexts {
        #[doc = "Texts"]
        pub texts: Vec<RichText>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes a text object inside an instant-view web page"]
    pub enum RichText {
        RichTextPlain(RichTextPlain),
        RichTextBold(RichTextBold),
        RichTextItalic(RichTextItalic),
        RichTextUnderline(RichTextUnderline),
        RichTextStrikethrough(RichTextStrikethrough),
        RichTextFixed(RichTextFixed),
        RichTextUrl(RichTextUrl),
        RichTextEmailAddress(RichTextEmailAddress),
        RichTextSubscript(RichTextSubscript),
        RichTextSuperscript(RichTextSuperscript),
        RichTextMarked(RichTextMarked),
        RichTextPhoneNumber(RichTextPhoneNumber),
        RichTextIcon(Box<RichTextIcon>),
        RichTextAnchor(RichTextAnchor),
        RichTexts(RichTexts),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a caption of an instant view web page block, consisting of a text and a trailing credit"]
    pub struct PageBlockCaption {
        #[doc = "Content of the caption"]
        pub text: RichText,
        #[doc = "Block credit (like HTML tag <cite>)"]
        pub credit: RichText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes an item of a list page block"]
    pub struct PageBlockListItem {
        #[doc = "Item label"]
        pub label: String,
        #[doc = "Item blocks"]
        pub page_blocks: Vec<PageBlock>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The content should be left-aligned"]
    pub struct PageBlockHorizontalAlignmentLeft {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The content should be center-aligned"]
    pub struct PageBlockHorizontalAlignmentCenter {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The content should be right-aligned"]
    pub struct PageBlockHorizontalAlignmentRight {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes a horizontal alignment of a table cell content"]
    pub enum PageBlockHorizontalAlignment {
        PageBlockHorizontalAlignmentLeft(PageBlockHorizontalAlignmentLeft),
        PageBlockHorizontalAlignmentCenter(PageBlockHorizontalAlignmentCenter),
        PageBlockHorizontalAlignmentRight(PageBlockHorizontalAlignmentRight),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The content should be top-aligned"]
    pub struct PageBlockVerticalAlignmentTop {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The content should be middle-aligned"]
    pub struct PageBlockVerticalAlignmentMiddle {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The content should be bottom-aligned"]
    pub struct PageBlockVerticalAlignmentBottom {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes a Vertical alignment of a table cell content"]
    pub enum PageBlockVerticalAlignment {
        PageBlockVerticalAlignmentTop(PageBlockVerticalAlignmentTop),
        PageBlockVerticalAlignmentMiddle(PageBlockVerticalAlignmentMiddle),
        PageBlockVerticalAlignmentBottom(PageBlockVerticalAlignmentBottom),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a cell of a table"]
    pub struct PageBlockTableCell {
        #[serde(default)]
        #[doc = "Cell text; may be null. If the text is null, then the cell should be invisible"]
        pub text: Option<RichText>,
        #[doc = "True, if it is a header cell"]
        pub is_header: bool,
        #[doc = "The number of columns the cell should span"]
        pub colspan: i32,
        #[doc = "The number of rows the cell should span"]
        pub rowspan: i32,
        #[doc = "Horizontal cell content alignment"]
        pub align: PageBlockHorizontalAlignment,
        #[doc = "Vertical cell content alignment"]
        pub valign: PageBlockVerticalAlignment,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a related article"]
    pub struct PageBlockRelatedArticle {
        #[doc = "Related article URL"]
        pub url: String,
        #[doc = "Article title; may be empty"]
        pub title: String,
        #[doc = "Contains information about a related article"]
        pub description: String,
        #[serde(default)]
        #[doc = "Article photo; may be null"]
        pub photo: Option<Photo>,
        #[doc = "Article author; may be empty"]
        pub author: String,
        #[doc = "Point in time (Unix timestamp) when the article was published; 0 if unknown"]
        pub publish_date: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The title of a page"]
    pub struct PageBlockTitle {
        #[doc = "Title"]
        pub title: RichText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The subtitle of a page"]
    pub struct PageBlockSubtitle {
        #[doc = "Subtitle"]
        pub subtitle: RichText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The author and publishing date of a page"]
    pub struct PageBlockAuthorDate {
        #[doc = "Author"]
        pub author: RichText,
        #[doc = "Point in time (Unix timestamp) when the article was published; 0 if unknown"]
        pub publish_date: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A header"]
    pub struct PageBlockHeader {
        #[doc = "Header"]
        pub header: RichText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A subheader"]
    pub struct PageBlockSubheader {
        #[doc = "Subheader"]
        pub subheader: RichText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A kicker"]
    pub struct PageBlockKicker {
        #[doc = "Kicker"]
        pub kicker: RichText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A text paragraph"]
    pub struct PageBlockParagraph {
        #[doc = "Paragraph text"]
        pub text: RichText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A preformatted text paragraph"]
    pub struct PageBlockPreformatted {
        #[doc = "Paragraph text"]
        pub text: RichText,
        #[doc = "Programming language for which the text should be formatted"]
        pub language: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The footer of a page"]
    pub struct PageBlockFooter {
        #[doc = "Footer"]
        pub footer: RichText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An empty block separating a page"]
    pub struct PageBlockDivider {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An invisible anchor on a page, which can be used in a URL to open the page from the specified anchor"]
    pub struct PageBlockAnchor {
        #[doc = "Name of the anchor"]
        pub name: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A list of data blocks"]
    pub struct PageBlockList {
        #[doc = "The items of the list"]
        pub items: Vec<PageBlockListItem>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A block quote"]
    pub struct PageBlockBlockQuote {
        #[doc = "Quote text"]
        pub text: RichText,
        #[doc = "Quote credit"]
        pub credit: RichText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A pull quote"]
    pub struct PageBlockPullQuote {
        #[doc = "Quote text"]
        pub text: RichText,
        #[doc = "Quote credit"]
        pub credit: RichText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An animation"]
    pub struct PageBlockAnimation {
        #[serde(default)]
        #[doc = "Animation file; may be null"]
        pub animation: Option<Animation>,
        #[doc = "Animation caption"]
        pub caption: PageBlockCaption,
        #[doc = "True, if the animation should be played automatically"]
        pub need_autoplay: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An audio file"]
    pub struct PageBlockAudio {
        #[serde(default)]
        #[doc = "Audio file; may be null"]
        pub audio: Option<Audio>,
        #[doc = "Audio file caption"]
        pub caption: PageBlockCaption,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A photo"]
    pub struct PageBlockPhoto {
        #[serde(default)]
        #[doc = "Photo file; may be null"]
        pub photo: Option<Photo>,
        #[doc = "Photo caption"]
        pub caption: PageBlockCaption,
        #[doc = "URL that needs to be opened when the photo is clicked"]
        pub url: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A video"]
    pub struct PageBlockVideo {
        #[serde(default)]
        #[doc = "Video file; may be null"]
        pub video: Option<Video>,
        #[doc = "Video caption"]
        pub caption: PageBlockCaption,
        #[doc = "True, if the video should be played automatically"]
        pub need_autoplay: bool,
        #[doc = "True, if the video should be looped"]
        pub is_looped: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A voice note"]
    pub struct PageBlockVoiceNote {
        #[serde(default)]
        #[doc = "Voice note; may be null"]
        pub voice_note: Option<VoiceNote>,
        #[doc = "Voice note caption"]
        pub caption: PageBlockCaption,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A page cover"]
    pub struct PageBlockCover {
        #[doc = "Cover"]
        pub cover: Box<PageBlock>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An embedded web page"]
    pub struct PageBlockEmbedded {
        #[doc = "Web page URL, if available"]
        pub url: String,
        #[doc = "HTML-markup of the embedded page"]
        pub html: String,
        #[serde(default)]
        #[doc = "Poster photo, if available; may be null"]
        pub poster_photo: Option<Photo>,
        #[doc = "Block width; 0 if unknown"]
        pub width: i32,
        #[doc = "Block height; 0 if unknown"]
        pub height: i32,
        #[doc = "Block caption"]
        pub caption: PageBlockCaption,
        #[doc = "True, if the block should be full width"]
        pub is_full_width: bool,
        #[doc = "True, if scrolling should be allowed"]
        pub allow_scrolling: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An embedded post"]
    pub struct PageBlockEmbeddedPost {
        #[doc = "Web page URL"]
        pub url: String,
        #[doc = "Post author"]
        pub author: String,
        #[serde(default)]
        #[doc = "Post author photo; may be null"]
        pub author_photo: Option<Photo>,
        #[doc = "Point in time (Unix timestamp) when the post was created; 0 if unknown"]
        pub date: i32,
        #[doc = "Post content"]
        pub page_blocks: Vec<PageBlock>,
        #[doc = "Post caption"]
        pub caption: PageBlockCaption,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A collage"]
    pub struct PageBlockCollage {
        #[doc = "Collage item contents"]
        pub page_blocks: Vec<PageBlock>,
        #[doc = "Block caption"]
        pub caption: PageBlockCaption,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A slideshow"]
    pub struct PageBlockSlideshow {
        #[doc = "Slideshow item contents"]
        pub page_blocks: Vec<PageBlock>,
        #[doc = "Block caption"]
        pub caption: PageBlockCaption,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A link to a chat"]
    pub struct PageBlockChatLink {
        #[doc = "Chat title"]
        pub title: String,
        #[serde(default)]
        #[doc = "Chat photo; may be null"]
        pub photo: Option<ChatPhoto>,
        #[doc = "Chat username, by which all other information about the chat should be resolved"]
        pub username: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A table"]
    pub struct PageBlockTable {
        #[doc = "Table caption"]
        pub caption: RichText,
        #[doc = "Table cells"]
        pub cells: Vec<Vec<PageBlockTableCell>>,
        #[doc = "True, if the table is bordered"]
        pub is_bordered: bool,
        #[doc = "True, if the table is striped"]
        pub is_striped: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A collapsible block"]
    pub struct PageBlockDetails {
        #[doc = "Always visible heading for the block"]
        pub header: RichText,
        #[doc = "Block contents"]
        pub page_blocks: Vec<PageBlock>,
        #[doc = "True, if the block is open by default"]
        pub is_open: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Related articles"]
    pub struct PageBlockRelatedArticles {
        #[doc = "Block header"]
        pub header: RichText,
        #[doc = "List of related articles"]
        pub articles: Vec<PageBlockRelatedArticle>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A map"]
    pub struct PageBlockMap {
        #[doc = "Location of the map center"]
        pub location: Location,
        #[doc = "Map zoom level"]
        pub zoom: i32,
        #[doc = "Map width"]
        pub width: i32,
        #[doc = "Map height"]
        pub height: i32,
        #[doc = "Block caption"]
        pub caption: PageBlockCaption,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes a block of an instant view web page"]
    pub enum PageBlock {
        PageBlockTitle(PageBlockTitle),
        PageBlockSubtitle(PageBlockSubtitle),
        PageBlockAuthorDate(PageBlockAuthorDate),
        PageBlockHeader(PageBlockHeader),
        PageBlockSubheader(PageBlockSubheader),
        PageBlockKicker(PageBlockKicker),
        PageBlockParagraph(PageBlockParagraph),
        PageBlockPreformatted(PageBlockPreformatted),
        PageBlockFooter(PageBlockFooter),
        PageBlockDivider(PageBlockDivider),
        PageBlockAnchor(PageBlockAnchor),
        PageBlockList(PageBlockList),
        PageBlockBlockQuote(PageBlockBlockQuote),
        PageBlockPullQuote(PageBlockPullQuote),
        PageBlockAnimation(PageBlockAnimation),
        PageBlockAudio(PageBlockAudio),
        PageBlockPhoto(PageBlockPhoto),
        PageBlockVideo(PageBlockVideo),
        PageBlockVoiceNote(PageBlockVoiceNote),
        PageBlockCover(PageBlockCover),
        PageBlockEmbedded(PageBlockEmbedded),
        PageBlockEmbeddedPost(PageBlockEmbeddedPost),
        PageBlockCollage(PageBlockCollage),
        PageBlockSlideshow(PageBlockSlideshow),
        PageBlockChatLink(PageBlockChatLink),
        PageBlockTable(PageBlockTable),
        PageBlockDetails(PageBlockDetails),
        PageBlockRelatedArticles(PageBlockRelatedArticles),
        PageBlockMap(PageBlockMap),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes an instant view page for a web page"]
    pub struct WebPageInstantView {
        #[doc = "Content of the web page"]
        pub page_blocks: Vec<PageBlock>,
        #[doc = "Version of the instant view, currently can be 1 or 2"]
        pub version: i32,
        #[doc = "Instant view URL; may be different from WebPage.url and must be used for the correct anchors handling"]
        pub url: String,
        #[doc = "True, if the instant view must be shown from right to left"]
        pub is_rtl: bool,
        #[doc = "True, if the instant view contains the full page. A network request might be needed to get the full web page instant view"]
        pub is_full: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a web page preview"]
    pub struct WebPage {
        #[doc = "Original URL of the link"]
        pub url: String,
        #[doc = "URL to display"]
        pub display_url: String,
        #[serde(rename = "type")]
        #[doc = "Type of the web page. Can be: article, photo, audio, video, document, profile, app, or something else"]
        pub type_: String,
        #[doc = "Short name of the site (e.g., Google Docs, App Store)"]
        pub site_name: String,
        #[doc = "Title of the content"]
        pub title: String,
        #[doc = "Describes a web page preview"]
        pub description: String,
        #[serde(default)]
        #[doc = "Image representing the content; may be null"]
        pub photo: Option<Photo>,
        #[doc = "URL to show in the embedded preview"]
        pub embed_url: String,
        #[doc = "MIME type of the embedded preview, (e.g., text/html or video/mp4)"]
        pub embed_type: String,
        #[doc = "Width of the embedded preview"]
        pub embed_width: i32,
        #[doc = "Height of the embedded preview"]
        pub embed_height: i32,
        #[doc = "Duration of the content, in seconds"]
        pub duration: i32,
        #[doc = "Author of the content"]
        pub author: String,
        #[serde(default)]
        #[doc = "Preview of the content as an animation, if available; may be null"]
        pub animation: Option<Animation>,
        #[serde(default)]
        #[doc = "Preview of the content as an audio file, if available; may be null"]
        pub audio: Option<Audio>,
        #[serde(default)]
        #[doc = "Preview of the content as a document, if available (currently only available for small PDF files and ZIP archives); may be null"]
        pub document: Option<Document>,
        #[serde(default)]
        #[doc = "Preview of the content as a sticker for small WEBP files, if available; may be null"]
        pub sticker: Option<Sticker>,
        #[serde(default)]
        #[doc = "Preview of the content as a video, if available; may be null"]
        pub video: Option<Video>,
        #[serde(default)]
        #[doc = "Preview of the content as a video note, if available; may be null"]
        pub video_note: Option<VideoNote>,
        #[serde(default)]
        #[doc = "Preview of the content as a voice note, if available; may be null"]
        pub voice_note: Option<VoiceNote>,
        #[doc = "Version of instant view, available for the web page (currently can be 1 or 2), 0 if none"]
        pub instant_view_version: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes an address"]
    pub struct Address {
        #[doc = "A two-letter ISO 3166-1 alpha-2 country code"]
        pub country_code: String,
        #[doc = "State, if applicable"]
        pub state: String,
        #[doc = "City"]
        pub city: String,
        #[doc = "First line of the address"]
        pub street_line1: String,
        #[doc = "Second line of the address"]
        pub street_line2: String,
        #[doc = "Address postal code"]
        pub postal_code: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Portion of the price of a product (e.g., \"delivery cost\", \"tax amount\")"]
    pub struct LabeledPricePart {
        #[doc = "Label for this portion of the product price"]
        pub label: String,
        #[doc = "Currency amount in minimal quantity of the currency"]
        pub amount: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Product invoice"]
    pub struct Invoice {
        #[doc = "ISO 4217 currency code"]
        pub currency: String,
        #[doc = "A list of objects used to calculate the total price of the product"]
        pub price_parts: Vec<LabeledPricePart>,
        #[doc = "True, if the payment is a test payment"]
        pub is_test: bool,
        #[doc = "True, if the user's name is needed for payment"]
        pub need_name: bool,
        #[doc = "True, if the user's phone number is needed for payment"]
        pub need_phone_number: bool,
        #[doc = "True, if the user's email address is needed for payment"]
        pub need_email_address: bool,
        #[doc = "True, if the user's shipping address is needed for payment"]
        pub need_shipping_address: bool,
        #[doc = "True, if the user's phone number will be sent to the provider"]
        pub send_phone_number_to_provider: bool,
        #[doc = "True, if the user's email address will be sent to the provider"]
        pub send_email_address_to_provider: bool,
        #[doc = "True, if the total price depends on the shipping method"]
        pub is_flexible: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Order information"]
    pub struct OrderInfo {
        #[doc = "Name of the user"]
        pub name: String,
        #[doc = "Phone number of the user"]
        pub phone_number: String,
        #[doc = "Email address of the user"]
        pub email_address: String,
        #[serde(default)]
        #[doc = "Shipping address for this order; may be null"]
        pub shipping_address: Option<Address>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "One shipping option"]
    pub struct ShippingOption {
        #[doc = "Shipping option identifier"]
        pub id: String,
        #[doc = "Option title"]
        pub title: String,
        #[doc = "A list of objects used to calculate the total shipping costs"]
        pub price_parts: Vec<LabeledPricePart>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about saved card credentials"]
    pub struct SavedCredentials {
        #[doc = "Unique identifier of the saved credentials"]
        pub id: String,
        #[doc = "Title of the saved credentials"]
        pub title: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password"]
    pub struct InputCredentialsSaved {
        #[doc = "Identifier of the saved credentials"]
        pub saved_credentials_id: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Applies if a user enters new credentials on a payment provider website"]
    pub struct InputCredentialsNew {
        #[doc = "Contains JSON-encoded data with a credential identifier from the payment provider"]
        pub data: String,
        #[doc = "True, if the credential identifier can be saved on the server side"]
        pub allow_save: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Applies if a user enters new credentials using Android Pay"]
    pub struct InputCredentialsAndroidPay {
        #[doc = "JSON-encoded data with the credential identifier"]
        pub data: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Applies if a user enters new credentials using Apple Pay"]
    pub struct InputCredentialsApplePay {
        #[doc = "JSON-encoded data with the credential identifier"]
        pub data: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains information about the payment method chosen by the user"]
    pub enum InputCredentials {
        InputCredentialsSaved(InputCredentialsSaved),
        InputCredentialsNew(InputCredentialsNew),
        InputCredentialsAndroidPay(InputCredentialsAndroidPay),
        InputCredentialsApplePay(InputCredentialsApplePay),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Stripe payment provider"]
    pub struct PaymentsProviderStripe {
        #[doc = "Stripe API publishable key"]
        pub publishable_key: String,
        #[doc = "True, if the user country must be provided"]
        pub need_country: bool,
        #[doc = "True, if the user ZIP/postal code must be provided"]
        pub need_postal_code: bool,
        #[doc = "True, if the cardholder name must be provided"]
        pub need_cardholder_name: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about an invoice payment form"]
    pub struct PaymentForm {
        #[doc = "Full information of the invoice"]
        pub invoice: Invoice,
        #[doc = "Payment form URL"]
        pub url: String,
        #[serde(default)]
        #[doc = "Contains information about the payment provider, if available, to support it natively without the need for opening the URL; may be null"]
        pub payments_provider: Option<PaymentsProviderStripe>,
        #[serde(default)]
        #[doc = "Saved server-side order information; may be null"]
        pub saved_order_info: Option<OrderInfo>,
        #[serde(default)]
        #[doc = "Contains information about saved card credentials; may be null"]
        pub saved_credentials: Option<SavedCredentials>,
        #[doc = "True, if the user can choose to save credentials"]
        pub can_save_credentials: bool,
        #[doc = "True, if the user will be able to save credentials protected by a password they set up"]
        pub need_password: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a temporary identifier of validated order information, which is stored for one hour. Also contains the available shipping options"]
    pub struct ValidatedOrderInfo {
        #[doc = "Temporary identifier of the order information"]
        pub order_info_id: String,
        #[doc = "Available shipping options"]
        pub shipping_options: Vec<ShippingOption>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains the result of a payment request"]
    pub struct PaymentResult {
        #[doc = "True, if the payment request was successful; otherwise the verification_url will be not empty"]
        pub success: bool,
        #[doc = "URL for additional payment credentials verification"]
        pub verification_url: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a successful payment"]
    pub struct PaymentReceipt {
        #[doc = "Point in time (Unix timestamp) when the payment was made"]
        pub date: i32,
        #[doc = "User identifier of the payment provider bot"]
        pub payments_provider_user_id: i32,
        #[doc = "Contains information about the invoice"]
        pub invoice: Invoice,
        #[serde(default)]
        #[doc = "Contains order information; may be null"]
        pub order_info: Option<OrderInfo>,
        #[serde(default)]
        #[doc = "Chosen shipping option; may be null"]
        pub shipping_option: Option<ShippingOption>,
        #[doc = "Title of the saved credentials"]
        pub credentials_title: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "File with the date it was uploaded"]
    pub struct DatedFile {
        #[doc = "The file"]
        pub file: File,
        #[doc = "Point in time (Unix timestamp) when the file was uploaded"]
        pub date: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's personal details"]
    pub struct PassportElementTypePersonalDetails {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's passport"]
    pub struct PassportElementTypePassport {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's driver license"]
    pub struct PassportElementTypeDriverLicense {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's identity card"]
    pub struct PassportElementTypeIdentityCard {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's internal passport"]
    pub struct PassportElementTypeInternalPassport {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's address"]
    pub struct PassportElementTypeAddress {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's utility bill"]
    pub struct PassportElementTypeUtilityBill {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's bank statement"]
    pub struct PassportElementTypeBankStatement {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's rental agreement"]
    pub struct PassportElementTypeRentalAgreement {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the registration page of the user's passport"]
    pub struct PassportElementTypePassportRegistration {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's temporary registration"]
    pub struct PassportElementTypeTemporaryRegistration {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's phone number"]
    pub struct PassportElementTypePhoneNumber {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's email address"]
    pub struct PassportElementTypeEmailAddress {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains the type of a Telegram Passport element"]
    pub enum PassportElementType {
        PassportElementTypePersonalDetails(PassportElementTypePersonalDetails),
        PassportElementTypePassport(PassportElementTypePassport),
        PassportElementTypeDriverLicense(PassportElementTypeDriverLicense),
        PassportElementTypeIdentityCard(PassportElementTypeIdentityCard),
        PassportElementTypeInternalPassport(PassportElementTypeInternalPassport),
        PassportElementTypeAddress(PassportElementTypeAddress),
        PassportElementTypeUtilityBill(PassportElementTypeUtilityBill),
        PassportElementTypeBankStatement(PassportElementTypeBankStatement),
        PassportElementTypeRentalAgreement(PassportElementTypeRentalAgreement),
        PassportElementTypePassportRegistration(PassportElementTypePassportRegistration),
        PassportElementTypeTemporaryRegistration(PassportElementTypeTemporaryRegistration),
        PassportElementTypePhoneNumber(PassportElementTypePhoneNumber),
        PassportElementTypeEmailAddress(PassportElementTypeEmailAddress),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a date according to the Gregorian calendar"]
    pub struct Date {
        #[doc = "Day of the month, 1-31"]
        pub day: i32,
        #[doc = "Month, 1-12"]
        pub month: i32,
        #[doc = "Year, 1-9999"]
        pub year: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains the user's personal details"]
    pub struct PersonalDetails {
        #[doc = "First name of the user written in English; 1-255 characters"]
        pub first_name: String,
        #[doc = "Middle name of the user written in English; 0-255 characters"]
        pub middle_name: String,
        #[doc = "Last name of the user written in English; 1-255 characters"]
        pub last_name: String,
        #[doc = "Native first name of the user; 1-255 characters"]
        pub native_first_name: String,
        #[doc = "Native middle name of the user; 0-255 characters"]
        pub native_middle_name: String,
        #[doc = "Native last name of the user; 1-255 characters"]
        pub native_last_name: String,
        #[doc = "Birthdate of the user"]
        pub birthdate: Date,
        #[doc = "Gender of the user, \"male\" or \"female\""]
        pub gender: String,
        #[doc = "A two-letter ISO 3166-1 alpha-2 country code of the user's country"]
        pub country_code: String,
        #[doc = "A two-letter ISO 3166-1 alpha-2 country code of the user's residence country"]
        pub residence_country_code: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An identity document"]
    pub struct IdentityDocument {
        #[doc = "Document number; 1-24 characters"]
        pub number: String,
        #[serde(default)]
        #[doc = "Document expiry date; may be null"]
        pub expiry_date: Option<Date>,
        #[doc = "Front side of the document"]
        pub front_side: DatedFile,
        #[doc = "Reverse side of the document; only for driver license and identity card"]
        pub reverse_side: DatedFile,
        #[serde(default)]
        #[doc = "Selfie with the document; may be null"]
        pub selfie: Option<DatedFile>,
        #[doc = "List of files containing a certified English translation of the document"]
        pub translation: Vec<DatedFile>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An identity document to be saved to Telegram Passport"]
    pub struct InputIdentityDocument {
        #[doc = "Document number; 1-24 characters"]
        pub number: String,
        #[doc = "Document expiry date, if available"]
        pub expiry_date: Date,
        #[doc = "Front side of the document"]
        pub front_side: InputFile,
        #[doc = "Reverse side of the document; only for driver license and identity card"]
        pub reverse_side: InputFile,
        #[doc = "Selfie with the document, if available"]
        pub selfie: InputFile,
        #[doc = "List of files containing a certified English translation of the document"]
        pub translation: Vec<InputFile>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A personal document, containing some information about a user"]
    pub struct PersonalDocument {
        #[doc = "List of files containing the pages of the document"]
        pub files: Vec<DatedFile>,
        #[doc = "List of files containing a certified English translation of the document"]
        pub translation: Vec<DatedFile>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A personal document to be saved to Telegram Passport"]
    pub struct InputPersonalDocument {
        #[doc = "List of files containing the pages of the document"]
        pub files: Vec<InputFile>,
        #[doc = "List of files containing a certified English translation of the document"]
        pub translation: Vec<InputFile>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's personal details"]
    pub struct PassportElementPersonalDetails {
        #[doc = "Personal details of the user"]
        pub personal_details: PersonalDetails,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's passport"]
    pub struct PassportElementPassport {
        #[doc = "Passport"]
        pub passport: IdentityDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's driver license"]
    pub struct PassportElementDriverLicense {
        #[doc = "Driver license"]
        pub driver_license: IdentityDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's identity card"]
    pub struct PassportElementIdentityCard {
        #[doc = "Identity card"]
        pub identity_card: IdentityDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's internal passport"]
    pub struct PassportElementInternalPassport {
        #[doc = "Internal passport"]
        pub internal_passport: IdentityDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's address"]
    pub struct PassportElementAddress {
        #[doc = "Address"]
        pub address: Address,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's utility bill"]
    pub struct PassportElementUtilityBill {
        #[doc = "Utility bill"]
        pub utility_bill: PersonalDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's bank statement"]
    pub struct PassportElementBankStatement {
        #[doc = "Bank statement"]
        pub bank_statement: PersonalDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's rental agreement"]
    pub struct PassportElementRentalAgreement {
        #[doc = "Rental agreement"]
        pub rental_agreement: PersonalDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's passport registration pages"]
    pub struct PassportElementPassportRegistration {
        #[doc = "Passport registration pages"]
        pub passport_registration: PersonalDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's temporary registration"]
    pub struct PassportElementTemporaryRegistration {
        #[doc = "Temporary registration"]
        pub temporary_registration: PersonalDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's phone number"]
    pub struct PassportElementPhoneNumber {
        #[doc = "Phone number"]
        pub phone_number: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element containing the user's email address"]
    pub struct PassportElementEmailAddress {
        #[doc = "Email address"]
        pub email_address: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains information about a Telegram Passport element"]
    pub enum PassportElement {
        PassportElementPersonalDetails(PassportElementPersonalDetails),
        PassportElementPassport(PassportElementPassport),
        PassportElementDriverLicense(PassportElementDriverLicense),
        PassportElementIdentityCard(PassportElementIdentityCard),
        PassportElementInternalPassport(PassportElementInternalPassport),
        PassportElementAddress(PassportElementAddress),
        PassportElementUtilityBill(PassportElementUtilityBill),
        PassportElementBankStatement(PassportElementBankStatement),
        PassportElementRentalAgreement(PassportElementRentalAgreement),
        PassportElementPassportRegistration(PassportElementPassportRegistration),
        PassportElementTemporaryRegistration(PassportElementTemporaryRegistration),
        PassportElementPhoneNumber(PassportElementPhoneNumber),
        PassportElementEmailAddress(PassportElementEmailAddress),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's personal details"]
    pub struct InputPassportElementPersonalDetails {
        #[doc = "Personal details of the user"]
        pub personal_details: PersonalDetails,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's passport"]
    pub struct InputPassportElementPassport {
        #[doc = "The passport to be saved"]
        pub passport: InputIdentityDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's driver license"]
    pub struct InputPassportElementDriverLicense {
        #[doc = "The driver license to be saved"]
        pub driver_license: InputIdentityDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's identity card"]
    pub struct InputPassportElementIdentityCard {
        #[doc = "The identity card to be saved"]
        pub identity_card: InputIdentityDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's internal passport"]
    pub struct InputPassportElementInternalPassport {
        #[doc = "The internal passport to be saved"]
        pub internal_passport: InputIdentityDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's address"]
    pub struct InputPassportElementAddress {
        #[doc = "The address to be saved"]
        pub address: Address,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's utility bill"]
    pub struct InputPassportElementUtilityBill {
        #[doc = "The utility bill to be saved"]
        pub utility_bill: InputPersonalDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's bank statement"]
    pub struct InputPassportElementBankStatement {
        #[doc = "The bank statement to be saved"]
        pub bank_statement: InputPersonalDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's rental agreement"]
    pub struct InputPassportElementRentalAgreement {
        #[doc = "The rental agreement to be saved"]
        pub rental_agreement: InputPersonalDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's passport registration"]
    pub struct InputPassportElementPassportRegistration {
        #[doc = "The passport registration page to be saved"]
        pub passport_registration: InputPersonalDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's temporary registration"]
    pub struct InputPassportElementTemporaryRegistration {
        #[doc = "The temporary registration document to be saved"]
        pub temporary_registration: InputPersonalDocument,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's phone number"]
    pub struct InputPassportElementPhoneNumber {
        #[doc = "The phone number to be saved"]
        pub phone_number: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Telegram Passport element to be saved containing the user's email address"]
    pub struct InputPassportElementEmailAddress {
        #[doc = "The email address to be saved"]
        pub email_address: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains information about a Telegram Passport element to be saved"]
    pub enum InputPassportElement {
        InputPassportElementPersonalDetails(InputPassportElementPersonalDetails),
        InputPassportElementPassport(InputPassportElementPassport),
        InputPassportElementDriverLicense(InputPassportElementDriverLicense),
        InputPassportElementIdentityCard(InputPassportElementIdentityCard),
        InputPassportElementInternalPassport(InputPassportElementInternalPassport),
        InputPassportElementAddress(InputPassportElementAddress),
        InputPassportElementUtilityBill(InputPassportElementUtilityBill),
        InputPassportElementBankStatement(InputPassportElementBankStatement),
        InputPassportElementRentalAgreement(InputPassportElementRentalAgreement),
        InputPassportElementPassportRegistration(InputPassportElementPassportRegistration),
        InputPassportElementTemporaryRegistration(InputPassportElementTemporaryRegistration),
        InputPassportElementPhoneNumber(InputPassportElementPhoneNumber),
        InputPassportElementEmailAddress(InputPassportElementEmailAddress),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about saved Telegram Passport elements"]
    pub struct PassportElements {
        #[doc = "Telegram Passport elements"]
        pub elements: Vec<PassportElement>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The element contains an error in an unspecified place. The error will be considered resolved when new data is added"]
    pub struct PassportElementErrorSourceUnspecified {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "One of the data fields contains an error. The error will be considered resolved when the value of the field changes"]
    pub struct PassportElementErrorSourceDataField {
        #[doc = "Field name"]
        pub field_name: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The front side of the document contains an error. The error will be considered resolved when the file with the front side changes"]
    pub struct PassportElementErrorSourceFrontSide {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The reverse side of the document contains an error. The error will be considered resolved when the file with the reverse side changes"]
    pub struct PassportElementErrorSourceReverseSide {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The selfie with the document contains an error. The error will be considered resolved when the file with the selfie changes"]
    pub struct PassportElementErrorSourceSelfie {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "One of files with the translation of the document contains an error. The error will be considered resolved when the file changes"]
    pub struct PassportElementErrorSourceTranslationFile {
        #[doc = "Index of a file with the error"]
        pub file_index: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The translation of the document contains an error. The error will be considered resolved when the list of translation files changes"]
    pub struct PassportElementErrorSourceTranslationFiles {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file contains an error. The error will be considered resolved when the file changes"]
    pub struct PassportElementErrorSourceFile {
        #[doc = "Index of a file with the error"]
        pub file_index: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The list of attached files contains an error. The error will be considered resolved when the list of files changes"]
    pub struct PassportElementErrorSourceFiles {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains the description of an error in a Telegram Passport element"]
    pub enum PassportElementErrorSource {
        PassportElementErrorSourceUnspecified(PassportElementErrorSourceUnspecified),
        PassportElementErrorSourceDataField(PassportElementErrorSourceDataField),
        PassportElementErrorSourceFrontSide(PassportElementErrorSourceFrontSide),
        PassportElementErrorSourceReverseSide(PassportElementErrorSourceReverseSide),
        PassportElementErrorSourceSelfie(PassportElementErrorSourceSelfie),
        PassportElementErrorSourceTranslationFile(PassportElementErrorSourceTranslationFile),
        PassportElementErrorSourceTranslationFiles(PassportElementErrorSourceTranslationFiles),
        PassportElementErrorSourceFile(PassportElementErrorSourceFile),
        PassportElementErrorSourceFiles(PassportElementErrorSourceFiles),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains the description of an error in a Telegram Passport element"]
    pub struct PassportElementError {
        #[serde(rename = "type")]
        #[doc = "Type of the Telegram Passport element which has the error"]
        pub type_: PassportElementType,
        #[doc = "Error message"]
        pub message: String,
        #[doc = "Error source"]
        pub source: PassportElementErrorSource,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a Telegram Passport element that was requested by a service"]
    pub struct PassportSuitableElement {
        #[serde(rename = "type")]
        #[doc = "Type of the element"]
        pub type_: PassportElementType,
        #[doc = "True, if a selfie is required with the identity document"]
        pub is_selfie_required: bool,
        #[doc = "True, if a certified English translation is required with the document"]
        pub is_translation_required: bool,
        #[doc = "True, if personal details must include the user's name in the language of their country of residence"]
        pub is_native_name_required: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a description of the required Telegram Passport element that was requested by a service"]
    pub struct PassportRequiredElement {
        #[doc = "List of Telegram Passport elements any of which is enough to provide"]
        pub suitable_elements: Vec<PassportSuitableElement>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a Telegram Passport authorization form that was requested"]
    pub struct PassportAuthorizationForm {
        #[doc = "Unique identifier of the authorization form"]
        pub id: i32,
        #[doc = "Information about the Telegram Passport elements that need to be provided to complete the form"]
        pub required_elements: Vec<PassportRequiredElement>,
        #[doc = "URL for the privacy policy of the service; may be empty"]
        pub privacy_policy_url: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a Telegram Passport elements and corresponding errors"]
    pub struct PassportElementsWithErrors {
        #[doc = "Telegram Passport elements"]
        pub elements: Vec<PassportElement>,
        #[doc = "Errors in the elements that are already available"]
        pub errors: Vec<PassportElementError>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains encrypted Telegram Passport data credentials"]
    pub struct EncryptedCredentials {
        #[doc = "The encrypted credentials"]
        pub data: String,
        #[doc = "The decrypted data hash"]
        pub hash: String,
        #[doc = "Secret for data decryption, encrypted with the service's public key"]
        pub secret: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about an encrypted Telegram Passport element; for bots only"]
    pub struct EncryptedPassportElement {
        #[serde(rename = "type")]
        #[doc = "Type of Telegram Passport element"]
        pub type_: PassportElementType,
        #[doc = "Encrypted JSON-encoded data about the user"]
        pub data: String,
        #[doc = "The front side of an identity document"]
        pub front_side: DatedFile,
        #[serde(default)]
        #[doc = "The reverse side of an identity document; may be null"]
        pub reverse_side: Option<DatedFile>,
        #[serde(default)]
        #[doc = "Selfie with the document; may be null"]
        pub selfie: Option<DatedFile>,
        #[doc = "List of files containing a certified English translation of the document"]
        pub translation: Vec<DatedFile>,
        #[doc = "List of attached files"]
        pub files: Vec<DatedFile>,
        #[doc = "Unencrypted data, phone number or email address"]
        pub value: String,
        #[doc = "Hash of the entire element"]
        pub hash: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The element contains an error in an unspecified place. The error will be considered resolved when new data is added"]
    pub struct InputPassportElementErrorSourceUnspecified {
        #[doc = "Current hash of the entire element"]
        pub element_hash: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A data field contains an error. The error is considered resolved when the field's value changes"]
    pub struct InputPassportElementErrorSourceDataField {
        #[doc = "Field name"]
        pub field_name: String,
        #[doc = "Current data hash"]
        pub data_hash: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The front side of the document contains an error. The error is considered resolved when the file with the front side of the document changes"]
    pub struct InputPassportElementErrorSourceFrontSide {
        #[doc = "Current hash of the file containing the front side"]
        pub file_hash: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The reverse side of the document contains an error. The error is considered resolved when the file with the reverse side of the document changes"]
    pub struct InputPassportElementErrorSourceReverseSide {
        #[doc = "Current hash of the file containing the reverse side"]
        pub file_hash: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The selfie contains an error. The error is considered resolved when the file with the selfie changes"]
    pub struct InputPassportElementErrorSourceSelfie {
        #[doc = "Current hash of the file containing the selfie"]
        pub file_hash: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "One of the files containing the translation of the document contains an error. The error is considered resolved when the file with the translation changes"]
    pub struct InputPassportElementErrorSourceTranslationFile {
        #[doc = "Current hash of the file containing the translation"]
        pub file_hash: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The translation of the document contains an error. The error is considered resolved when the list of files changes"]
    pub struct InputPassportElementErrorSourceTranslationFiles {
        #[doc = "Current hashes of all files with the translation"]
        pub file_hashes: Vec<String>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file contains an error. The error is considered resolved when the file changes"]
    pub struct InputPassportElementErrorSourceFile {
        #[doc = "Current hash of the file which has the error"]
        pub file_hash: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The list of attached files contains an error. The error is considered resolved when the file list changes"]
    pub struct InputPassportElementErrorSourceFiles {
        #[doc = "Current hashes of all attached files"]
        pub file_hashes: Vec<String>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains the description of an error in a Telegram Passport element; for bots only"]
    pub enum InputPassportElementErrorSource {
        InputPassportElementErrorSourceUnspecified(InputPassportElementErrorSourceUnspecified),
        InputPassportElementErrorSourceDataField(InputPassportElementErrorSourceDataField),
        InputPassportElementErrorSourceFrontSide(InputPassportElementErrorSourceFrontSide),
        InputPassportElementErrorSourceReverseSide(InputPassportElementErrorSourceReverseSide),
        InputPassportElementErrorSourceSelfie(InputPassportElementErrorSourceSelfie),
        InputPassportElementErrorSourceTranslationFile(
            InputPassportElementErrorSourceTranslationFile,
        ),
        InputPassportElementErrorSourceTranslationFiles(
            InputPassportElementErrorSourceTranslationFiles,
        ),
        InputPassportElementErrorSourceFile(InputPassportElementErrorSourceFile),
        InputPassportElementErrorSourceFiles(InputPassportElementErrorSourceFiles),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains the description of an error in a Telegram Passport element; for bots only"]
    pub struct InputPassportElementError {
        #[serde(rename = "type")]
        #[doc = "Type of Telegram Passport element that has the error"]
        pub type_: PassportElementType,
        #[doc = "Error message"]
        pub message: String,
        #[doc = "Error source"]
        pub source: InputPassportElementErrorSource,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A text message"]
    pub struct MessageText {
        #[doc = "Text of the message"]
        pub text: FormattedText,
        #[serde(default)]
        #[doc = "A preview of the web page that's mentioned in the text; may be null"]
        pub web_page: Option<WebPage>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An animation message (GIF-style)."]
    pub struct MessageAnimation {
        #[doc = "The animation description"]
        pub animation: Animation,
        #[doc = "Animation caption"]
        pub caption: FormattedText,
        #[doc = "True, if the animation thumbnail must be blurred and the animation must be shown only while tapped"]
        pub is_secret: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An audio message"]
    pub struct MessageAudio {
        #[doc = "The audio description"]
        pub audio: Audio,
        #[doc = "Audio caption"]
        pub caption: FormattedText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A document message (general file)"]
    pub struct MessageDocument {
        #[doc = "The document description"]
        pub document: Document,
        #[doc = "Document caption"]
        pub caption: FormattedText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A photo message"]
    pub struct MessagePhoto {
        #[doc = "The photo description"]
        pub photo: Photo,
        #[doc = "Photo caption"]
        pub caption: FormattedText,
        #[doc = "True, if the photo must be blurred and must be shown only while tapped"]
        pub is_secret: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An expired photo message (self-destructed after TTL has elapsed)"]
    pub struct MessageExpiredPhoto {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A sticker message"]
    pub struct MessageSticker {
        #[doc = "The sticker description"]
        pub sticker: Sticker,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A video message"]
    pub struct MessageVideo {
        #[doc = "The video description"]
        pub video: Video,
        #[doc = "Video caption"]
        pub caption: FormattedText,
        #[doc = "True, if the video thumbnail must be blurred and the video must be shown only while tapped"]
        pub is_secret: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An expired video message (self-destructed after TTL has elapsed)"]
    pub struct MessageExpiredVideo {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A video note message"]
    pub struct MessageVideoNote {
        #[doc = "The video note description"]
        pub video_note: VideoNote,
        #[doc = "True, if at least one of the recipients has viewed the video note"]
        pub is_viewed: bool,
        #[doc = "True, if the video note thumbnail must be blurred and the video note must be shown only while tapped"]
        pub is_secret: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A voice note message"]
    pub struct MessageVoiceNote {
        #[doc = "The voice note description"]
        pub voice_note: VoiceNote,
        #[doc = "Voice note caption"]
        pub caption: FormattedText,
        #[doc = "True, if at least one of the recipients has listened to the voice note"]
        pub is_listened: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a location"]
    pub struct MessageLocation {
        #[doc = "The location description"]
        pub location: Location,
        #[doc = "Time relative to the message sent date until which the location can be updated, in seconds"]
        pub live_period: i32,
        #[doc = "Left time for which the location can be updated, in seconds. updateMessageContent is not sent when this field changes"]
        pub expires_in: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with information about a venue"]
    pub struct MessageVenue {
        #[doc = "The venue description"]
        pub venue: Venue,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a user contact"]
    pub struct MessageContact {
        #[doc = "The contact description"]
        pub contact: Contact,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a game"]
    pub struct MessageGame {
        #[doc = "The game description"]
        pub game: Game,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a poll"]
    pub struct MessagePoll {
        #[doc = "The poll description"]
        pub poll: Poll,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with an invoice from a bot"]
    pub struct MessageInvoice {
        #[doc = "Product title"]
        pub title: String,
        #[doc = "A message with an invoice from a bot"]
        pub description: String,
        #[serde(default)]
        #[doc = "Product photo; may be null"]
        pub photo: Option<Photo>,
        #[doc = "Currency for the product price"]
        pub currency: String,
        #[doc = "Product total price in the minimal quantity of the currency"]
        pub total_amount: i64,
        #[doc = "Unique invoice bot start_parameter. To share an invoice use the URL https://t.me/{bot_username}?start={start_parameter}"]
        pub start_parameter: String,
        #[doc = "True, if the invoice is a test invoice"]
        pub is_test: bool,
        #[doc = "True, if the shipping address should be specified"]
        pub need_shipping_address: bool,
        #[doc = "The identifier of the message with the receipt, after the product has been purchased"]
        pub receipt_message_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with information about an ended call"]
    pub struct MessageCall {
        #[doc = "Reason why the call was discarded"]
        pub discard_reason: CallDiscardReason,
        #[doc = "Call duration, in seconds"]
        pub duration: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A newly created basic group"]
    pub struct MessageBasicGroupChatCreate {
        #[doc = "Title of the basic group"]
        pub title: String,
        #[doc = "User identifiers of members in the basic group"]
        pub member_user_ids: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A newly created supergroup or channel"]
    pub struct MessageSupergroupChatCreate {
        #[doc = "Title of the supergroup or channel"]
        pub title: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An updated chat title"]
    pub struct MessageChatChangeTitle {
        #[doc = "New chat title"]
        pub title: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An updated chat photo"]
    pub struct MessageChatChangePhoto {
        #[doc = "New chat photo"]
        pub photo: Photo,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A deleted chat photo"]
    pub struct MessageChatDeletePhoto {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "New chat members were added"]
    pub struct MessageChatAddMembers {
        #[doc = "User identifiers of the new members"]
        pub member_user_ids: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new member joined the chat by invite link"]
    pub struct MessageChatJoinByLink {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat member was deleted"]
    pub struct MessageChatDeleteMember {
        #[doc = "User identifier of the deleted chat member"]
        pub user_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A basic group was upgraded to a supergroup and was deactivated as the result"]
    pub struct MessageChatUpgradeTo {
        #[doc = "Identifier of the supergroup to which the basic group was upgraded"]
        pub supergroup_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A supergroup has been created from a basic group"]
    pub struct MessageChatUpgradeFrom {
        #[doc = "Title of the newly created supergroup"]
        pub title: String,
        #[doc = "The identifier of the original basic group"]
        pub basic_group_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message has been pinned"]
    pub struct MessagePinMessage {
        #[doc = "Identifier of the pinned message, can be an identifier of a deleted message or 0"]
        pub message_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A screenshot of a message in the chat has been taken"]
    pub struct MessageScreenshotTaken {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The TTL (Time To Live) setting messages in a secret chat has been changed"]
    pub struct MessageChatSetTtl {
        #[doc = "New TTL"]
        pub ttl: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A non-standard action has happened in the chat"]
    pub struct MessageCustomServiceAction {
        #[doc = "Message text to be shown in the chat"]
        pub text: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new high score was achieved in a game"]
    pub struct MessageGameScore {
        #[doc = "Identifier of the message with the game, can be an identifier of a deleted message"]
        pub game_message_id: i64,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the game; may be different from the games presented in the message with the game"]
        pub game_id: i64,
        #[doc = "New score"]
        pub score: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A payment has been completed"]
    pub struct MessagePaymentSuccessful {
        #[doc = "Identifier of the message with the corresponding invoice; can be an identifier of a deleted message"]
        pub invoice_message_id: i64,
        #[doc = "Currency for the price of the product"]
        pub currency: String,
        #[doc = "Total price for the product, in the minimal quantity of the currency"]
        pub total_amount: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A payment has been completed; for bots only"]
    pub struct MessagePaymentSuccessfulBot {
        #[doc = "Identifier of the message with the corresponding invoice; can be an identifier of a deleted message"]
        pub invoice_message_id: i64,
        #[doc = "Currency for price of the product"]
        pub currency: String,
        #[doc = "Total price for the product, in the minimal quantity of the currency"]
        pub total_amount: i64,
        #[doc = "Invoice payload"]
        pub invoice_payload: String,
        #[doc = "Identifier of the shipping option chosen by the user; may be empty if not applicable"]
        pub shipping_option_id: String,
        #[serde(default)]
        #[doc = "Information about the order; may be null"]
        pub order_info: Option<OrderInfo>,
        #[doc = "Telegram payment identifier"]
        pub telegram_payment_charge_id: String,
        #[doc = "Provider payment identifier"]
        pub provider_payment_charge_id: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A contact has registered with Telegram"]
    pub struct MessageContactRegistered {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The current user has connected a website by logging in using Telegram Login Widget on it"]
    pub struct MessageWebsiteConnected {
        #[doc = "Domain name of the connected website"]
        pub domain_name: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Telegram Passport data has been sent"]
    pub struct MessagePassportDataSent {
        #[doc = "List of Telegram Passport element types sent"]
        pub types: Vec<PassportElementType>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Telegram Passport data has been received; for bots only"]
    pub struct MessagePassportDataReceived {
        #[doc = "List of received Telegram Passport elements"]
        pub elements: Vec<EncryptedPassportElement>,
        #[doc = "Encrypted data credentials"]
        pub credentials: EncryptedCredentials,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Message content that is not supported by the client"]
    pub struct MessageUnsupported {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains the content of a message"]
    pub enum MessageContent {
        MessageText(MessageText),
        MessageAnimation(MessageAnimation),
        MessageAudio(MessageAudio),
        MessageDocument(MessageDocument),
        MessagePhoto(MessagePhoto),
        MessageExpiredPhoto(MessageExpiredPhoto),
        MessageSticker(MessageSticker),
        MessageVideo(MessageVideo),
        MessageExpiredVideo(MessageExpiredVideo),
        MessageVideoNote(MessageVideoNote),
        MessageVoiceNote(MessageVoiceNote),
        MessageLocation(MessageLocation),
        MessageVenue(MessageVenue),
        MessageContact(MessageContact),
        MessageGame(MessageGame),
        MessagePoll(MessagePoll),
        MessageInvoice(MessageInvoice),
        MessageCall(MessageCall),
        MessageBasicGroupChatCreate(MessageBasicGroupChatCreate),
        MessageSupergroupChatCreate(MessageSupergroupChatCreate),
        MessageChatChangeTitle(MessageChatChangeTitle),
        MessageChatChangePhoto(MessageChatChangePhoto),
        MessageChatDeletePhoto(MessageChatDeletePhoto),
        MessageChatAddMembers(MessageChatAddMembers),
        MessageChatJoinByLink(MessageChatJoinByLink),
        MessageChatDeleteMember(MessageChatDeleteMember),
        MessageChatUpgradeTo(MessageChatUpgradeTo),
        MessageChatUpgradeFrom(MessageChatUpgradeFrom),
        MessagePinMessage(MessagePinMessage),
        MessageScreenshotTaken(MessageScreenshotTaken),
        MessageChatSetTtl(MessageChatSetTtl),
        MessageCustomServiceAction(MessageCustomServiceAction),
        MessageGameScore(MessageGameScore),
        MessagePaymentSuccessful(MessagePaymentSuccessful),
        MessagePaymentSuccessfulBot(MessagePaymentSuccessfulBot),
        MessageContactRegistered(MessageContactRegistered),
        MessageWebsiteConnected(MessageWebsiteConnected),
        MessagePassportDataSent(MessagePassportDataSent),
        MessagePassportDataReceived(MessagePassportDataReceived),
        MessageUnsupported(MessageUnsupported),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A mention of a user by their username"]
    pub struct TextEntityTypeMention {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A hashtag text, beginning with \"#\""]
    pub struct TextEntityTypeHashtag {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A cashtag text, beginning with \"$\" and consisting of capital english letters (i.e. \"$USD\")"]
    pub struct TextEntityTypeCashtag {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A bot command, beginning with \"/\". This shouldn't be highlighted if there are no bots in the chat"]
    pub struct TextEntityTypeBotCommand {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An HTTP URL"]
    pub struct TextEntityTypeUrl {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An email address"]
    pub struct TextEntityTypeEmailAddress {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A phone number"]
    pub struct TextEntityTypePhoneNumber {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A bold text"]
    pub struct TextEntityTypeBold {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An italic text"]
    pub struct TextEntityTypeItalic {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An underlined text"]
    pub struct TextEntityTypeUnderline {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A strikethrough text"]
    pub struct TextEntityTypeStrikethrough {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Text that must be formatted as if inside a code HTML tag"]
    pub struct TextEntityTypeCode {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Text that must be formatted as if inside a pre HTML tag"]
    pub struct TextEntityTypePre {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Text that must be formatted as if inside pre, and code HTML tags"]
    pub struct TextEntityTypePreCode {
        #[doc = "Programming language of the code; as defined by the sender"]
        pub language: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A text description shown instead of a raw URL"]
    pub struct TextEntityTypeTextUrl {
        #[doc = "HTTP or tg:// URL to be opened when the link is clicked"]
        pub url: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A text shows instead of a raw mention of the user (e.g., when the user has no username)"]
    pub struct TextEntityTypeMentionName {
        #[doc = "Identifier of the mentioned user"]
        pub user_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents a part of the text which must be formatted differently"]
    pub enum TextEntityType {
        TextEntityTypeMention(TextEntityTypeMention),
        TextEntityTypeHashtag(TextEntityTypeHashtag),
        TextEntityTypeCashtag(TextEntityTypeCashtag),
        TextEntityTypeBotCommand(TextEntityTypeBotCommand),
        TextEntityTypeUrl(TextEntityTypeUrl),
        TextEntityTypeEmailAddress(TextEntityTypeEmailAddress),
        TextEntityTypePhoneNumber(TextEntityTypePhoneNumber),
        TextEntityTypeBold(TextEntityTypeBold),
        TextEntityTypeItalic(TextEntityTypeItalic),
        TextEntityTypeUnderline(TextEntityTypeUnderline),
        TextEntityTypeStrikethrough(TextEntityTypeStrikethrough),
        TextEntityTypeCode(TextEntityTypeCode),
        TextEntityTypePre(TextEntityTypePre),
        TextEntityTypePreCode(TextEntityTypePreCode),
        TextEntityTypeTextUrl(TextEntityTypeTextUrl),
        TextEntityTypeMentionName(TextEntityTypeMentionName),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A thumbnail to be sent along with a file; should be in JPEG or WEBP format for stickers, and less than 200 kB in size"]
    pub struct InputThumbnail {
        #[doc = "Thumbnail file to send. Sending thumbnails by file_id is currently not supported"]
        pub thumbnail: InputFile,
        #[doc = "Thumbnail width, usually shouldn't exceed 320. Use 0 if unknown"]
        pub width: i32,
        #[doc = "Thumbnail height, usually shouldn't exceed 320. Use 0 if unknown"]
        pub height: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The message will be sent at the specified date"]
    pub struct MessageSchedulingStateSendAtDate {
        #[doc = "Date the message will be sent. The date must be within 367 days in the future"]
        pub send_date: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The message will be sent when the peer will be online. Applicable to private chats only and when the exact online status of the peer is known"]
    pub struct MessageSchedulingStateSendWhenOnline {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains information about the time when a scheduled message will be sent"]
    pub enum MessageSchedulingState {
        MessageSchedulingStateSendAtDate(MessageSchedulingStateSendAtDate),
        MessageSchedulingStateSendWhenOnline(MessageSchedulingStateSendWhenOnline),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Options to be used when a message is send"]
    pub struct SendMessageOptions {
        #[doc = "Pass true to disable notification for the message. Must be false if the message is sent to a secret chat"]
        pub disable_notification: bool,
        #[doc = "Pass true if the message is sent from the background"]
        pub from_background: bool,
        #[doc = "Message scheduling state. Messages sent to a secret chat, live location messages and self-destructing messages can't be scheduled"]
        pub scheduling_state: MessageSchedulingState,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A text message"]
    pub struct InputMessageText {
        #[doc = "Formatted text to be sent; 1-GetOption(\"message_text_length_max\") characters. Only Bold, Italic, Underline, Strikethrough, Code, Pre, PreCode, TextUrl and MentionName entities are allowed to be specified manually"]
        pub text: FormattedText,
        #[doc = "True, if rich web page previews for URLs in the message text should be disabled"]
        pub disable_web_page_preview: bool,
        #[doc = "True, if a chat message draft should be deleted"]
        pub clear_draft: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An animation message (GIF-style)."]
    pub struct InputMessageAnimation {
        #[doc = "Animation file to be sent"]
        pub animation: InputFile,
        #[doc = "Animation thumbnail, if available"]
        pub thumbnail: InputThumbnail,
        #[doc = "Duration of the animation, in seconds"]
        pub duration: i32,
        #[doc = "Width of the animation; may be replaced by the server"]
        pub width: i32,
        #[doc = "Height of the animation; may be replaced by the server"]
        pub height: i32,
        #[doc = "Animation caption; 0-GetOption(\"message_caption_length_max\") characters"]
        pub caption: FormattedText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An audio message"]
    pub struct InputMessageAudio {
        #[doc = "Audio file to be sent"]
        pub audio: InputFile,
        #[doc = "Thumbnail of the cover for the album, if available"]
        pub album_cover_thumbnail: InputThumbnail,
        #[doc = "Duration of the audio, in seconds; may be replaced by the server"]
        pub duration: i32,
        #[doc = "Title of the audio; 0-64 characters; may be replaced by the server"]
        pub title: String,
        #[doc = "Performer of the audio; 0-64 characters, may be replaced by the server"]
        pub performer: String,
        #[doc = "Audio caption; 0-GetOption(\"message_caption_length_max\") characters"]
        pub caption: FormattedText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A document message (general file)"]
    pub struct InputMessageDocument {
        #[doc = "Document to be sent"]
        pub document: InputFile,
        #[doc = "Document thumbnail, if available"]
        pub thumbnail: InputThumbnail,
        #[doc = "Document caption; 0-GetOption(\"message_caption_length_max\") characters"]
        pub caption: FormattedText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A photo message"]
    pub struct InputMessagePhoto {
        #[doc = "Photo to send"]
        pub photo: InputFile,
        #[doc = "Photo thumbnail to be sent, this is sent to the other party in secret chats only"]
        pub thumbnail: InputThumbnail,
        #[doc = "File identifiers of the stickers added to the photo, if applicable"]
        pub added_sticker_file_ids: Vec<i32>,
        #[doc = "Photo width"]
        pub width: i32,
        #[doc = "Photo height"]
        pub height: i32,
        #[doc = "Photo caption; 0-GetOption(\"message_caption_length_max\") characters"]
        pub caption: FormattedText,
        #[doc = "Photo TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats"]
        pub ttl: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A sticker message"]
    pub struct InputMessageSticker {
        #[doc = "Sticker to be sent"]
        pub sticker: InputFile,
        #[doc = "Sticker thumbnail, if available"]
        pub thumbnail: InputThumbnail,
        #[doc = "Sticker width"]
        pub width: i32,
        #[doc = "Sticker height"]
        pub height: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A video message"]
    pub struct InputMessageVideo {
        #[doc = "Video to be sent"]
        pub video: InputFile,
        #[doc = "Video thumbnail, if available"]
        pub thumbnail: InputThumbnail,
        #[doc = "File identifiers of the stickers added to the video, if applicable"]
        pub added_sticker_file_ids: Vec<i32>,
        #[doc = "Duration of the video, in seconds"]
        pub duration: i32,
        #[doc = "Video width"]
        pub width: i32,
        #[doc = "Video height"]
        pub height: i32,
        #[doc = "True, if the video should be tried to be streamed"]
        pub supports_streaming: bool,
        #[doc = "Video caption; 0-GetOption(\"message_caption_length_max\") characters"]
        pub caption: FormattedText,
        #[doc = "Video TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats"]
        pub ttl: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A video note message"]
    pub struct InputMessageVideoNote {
        #[doc = "Video note to be sent"]
        pub video_note: InputFile,
        #[doc = "Video thumbnail, if available"]
        pub thumbnail: InputThumbnail,
        #[doc = "Duration of the video, in seconds"]
        pub duration: i32,
        #[doc = "Video width and height; must be positive and not greater than 640"]
        pub length: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A voice note message"]
    pub struct InputMessageVoiceNote {
        #[doc = "Voice note to be sent"]
        pub voice_note: InputFile,
        #[doc = "Duration of the voice note, in seconds"]
        pub duration: i32,
        #[doc = "Waveform representation of the voice note, in 5-bit format"]
        pub waveform: String,
        #[doc = "Voice note caption; 0-GetOption(\"message_caption_length_max\") characters"]
        pub caption: FormattedText,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a location"]
    pub struct InputMessageLocation {
        #[doc = "Location to be sent"]
        pub location: Location,
        #[doc = "Period for which the location can be updated, in seconds; should be between 60 and 86400 for a live location and 0 otherwise"]
        pub live_period: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with information about a venue"]
    pub struct InputMessageVenue {
        #[doc = "Venue to send"]
        pub venue: Venue,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message containing a user contact"]
    pub struct InputMessageContact {
        #[doc = "Contact to send"]
        pub contact: Contact,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a game; not supported for channels or secret chats"]
    pub struct InputMessageGame {
        #[doc = "User identifier of the bot that owns the game"]
        pub bot_user_id: i32,
        #[doc = "Short name of the game"]
        pub game_short_name: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with an invoice; can be used only by bots and only in private chats"]
    pub struct InputMessageInvoice {
        #[doc = "Invoice"]
        pub invoice: Invoice,
        #[doc = "Product title; 1-32 characters"]
        pub title: String,
        #[doc = "A message with an invoice; can be used only by bots and only in private chats"]
        pub description: String,
        #[doc = "Product photo URL; optional"]
        pub photo_url: String,
        #[doc = "Product photo size"]
        pub photo_size: i32,
        #[doc = "Product photo width"]
        pub photo_width: i32,
        #[doc = "Product photo height"]
        pub photo_height: i32,
        #[doc = "The invoice payload"]
        pub payload: String,
        #[doc = "Payment provider token"]
        pub provider_token: String,
        #[doc = "JSON-encoded data about the invoice, which will be shared with the payment provider"]
        pub provider_data: String,
        #[doc = "Unique invoice bot start_parameter for the generation of this invoice"]
        pub start_parameter: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a poll. Polls can't be sent to secret chats. Polls can be sent only to a private chat with a bot"]
    pub struct InputMessagePoll {
        #[doc = "Poll question, 1-255 characters"]
        pub question: String,
        #[doc = "List of poll answer options, 2-10 strings 1-100 characters each"]
        pub options: Vec<String>,
        #[doc = "True, if the poll voters are anonymous. Non-anonymous polls can't be sent or forwarded to channels"]
        pub is_anonymous: bool,
        #[serde(rename = "type")]
        #[doc = "Type of the poll"]
        pub type_: PollType,
        #[serde(default)]
        #[doc = "True, if the poll needs to be sent already closed; for bots only"]
        pub is_closed: Option<bool>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A forwarded message"]
    pub struct InputMessageForwarded {
        #[doc = "Identifier for the chat this forwarded message came from"]
        pub from_chat_id: i64,
        #[doc = "Identifier of the message to forward"]
        pub message_id: i64,
        #[doc = "True, if a game message should be shared within a launched game; applies only to game messages"]
        pub in_game_share: bool,
        #[doc = "True, if content of the message needs to be copied without a link to the original message. Always true if the message is forwarded to a secret chat"]
        pub send_copy: bool,
        #[doc = "True, if media caption of the message copy needs to be removed. Ignored if send_copy is false"]
        pub remove_caption: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "The content of a message to send"]
    pub enum InputMessageContent {
        InputMessageText(InputMessageText),
        InputMessageAnimation(InputMessageAnimation),
        InputMessageAudio(InputMessageAudio),
        InputMessageDocument(InputMessageDocument),
        InputMessagePhoto(InputMessagePhoto),
        InputMessageSticker(InputMessageSticker),
        InputMessageVideo(InputMessageVideo),
        InputMessageVideoNote(InputMessageVideoNote),
        InputMessageVoiceNote(InputMessageVoiceNote),
        InputMessageLocation(InputMessageLocation),
        InputMessageVenue(InputMessageVenue),
        InputMessageContact(InputMessageContact),
        InputMessageGame(InputMessageGame),
        InputMessageInvoice(InputMessageInvoice),
        InputMessagePoll(InputMessagePoll),
        InputMessageForwarded(InputMessageForwarded),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns all found messages, no filter is applied"]
    pub struct SearchMessagesFilterEmpty {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only animation messages"]
    pub struct SearchMessagesFilterAnimation {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only audio messages"]
    pub struct SearchMessagesFilterAudio {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only document messages"]
    pub struct SearchMessagesFilterDocument {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only photo messages"]
    pub struct SearchMessagesFilterPhoto {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only video messages"]
    pub struct SearchMessagesFilterVideo {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only voice note messages"]
    pub struct SearchMessagesFilterVoiceNote {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only photo and video messages"]
    pub struct SearchMessagesFilterPhotoAndVideo {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only messages containing URLs"]
    pub struct SearchMessagesFilterUrl {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only messages containing chat photos"]
    pub struct SearchMessagesFilterChatPhoto {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only call messages"]
    pub struct SearchMessagesFilterCall {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only incoming call messages with missed/declined discard reasons"]
    pub struct SearchMessagesFilterMissedCall {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only video note messages"]
    pub struct SearchMessagesFilterVideoNote {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only voice and video note messages"]
    pub struct SearchMessagesFilterVoiceAndVideoNote {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only messages with mentions of the current user, or messages that are replies to their messages"]
    pub struct SearchMessagesFilterMention {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Returns only messages with unread mentions of the current user, or messages that are replies to their messages. When using this filter the results can't be additionally filtered by a query or by the sending user"]
    pub struct SearchMessagesFilterUnreadMention {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents a filter for message search results"]
    pub enum SearchMessagesFilter {
        SearchMessagesFilterEmpty(SearchMessagesFilterEmpty),
        SearchMessagesFilterAnimation(SearchMessagesFilterAnimation),
        SearchMessagesFilterAudio(SearchMessagesFilterAudio),
        SearchMessagesFilterDocument(SearchMessagesFilterDocument),
        SearchMessagesFilterPhoto(SearchMessagesFilterPhoto),
        SearchMessagesFilterVideo(SearchMessagesFilterVideo),
        SearchMessagesFilterVoiceNote(SearchMessagesFilterVoiceNote),
        SearchMessagesFilterPhotoAndVideo(SearchMessagesFilterPhotoAndVideo),
        SearchMessagesFilterUrl(SearchMessagesFilterUrl),
        SearchMessagesFilterChatPhoto(SearchMessagesFilterChatPhoto),
        SearchMessagesFilterCall(SearchMessagesFilterCall),
        SearchMessagesFilterMissedCall(SearchMessagesFilterMissedCall),
        SearchMessagesFilterVideoNote(SearchMessagesFilterVideoNote),
        SearchMessagesFilterVoiceAndVideoNote(SearchMessagesFilterVoiceAndVideoNote),
        SearchMessagesFilterMention(SearchMessagesFilterMention),
        SearchMessagesFilterUnreadMention(SearchMessagesFilterUnreadMention),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is typing a message"]
    pub struct ChatActionTyping {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is recording a video"]
    pub struct ChatActionRecordingVideo {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is uploading a video"]
    pub struct ChatActionUploadingVideo {
        #[doc = "Upload progress, as a percentage"]
        pub progress: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is recording a voice note"]
    pub struct ChatActionRecordingVoiceNote {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is uploading a voice note"]
    pub struct ChatActionUploadingVoiceNote {
        #[doc = "Upload progress, as a percentage"]
        pub progress: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is uploading a photo"]
    pub struct ChatActionUploadingPhoto {
        #[doc = "Upload progress, as a percentage"]
        pub progress: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is uploading a document"]
    pub struct ChatActionUploadingDocument {
        #[doc = "Upload progress, as a percentage"]
        pub progress: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is picking a location or venue to send"]
    pub struct ChatActionChoosingLocation {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is picking a contact to send"]
    pub struct ChatActionChoosingContact {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user has started to play a game"]
    pub struct ChatActionStartPlayingGame {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is recording a video note"]
    pub struct ChatActionRecordingVideoNote {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is uploading a video note"]
    pub struct ChatActionUploadingVideoNote {
        #[doc = "Upload progress, as a percentage"]
        pub progress: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user has cancelled the previous action"]
    pub struct ChatActionCancel {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the different types of activity in a chat"]
    pub enum ChatAction {
        ChatActionTyping(ChatActionTyping),
        ChatActionRecordingVideo(ChatActionRecordingVideo),
        ChatActionUploadingVideo(ChatActionUploadingVideo),
        ChatActionRecordingVoiceNote(ChatActionRecordingVoiceNote),
        ChatActionUploadingVoiceNote(ChatActionUploadingVoiceNote),
        ChatActionUploadingPhoto(ChatActionUploadingPhoto),
        ChatActionUploadingDocument(ChatActionUploadingDocument),
        ChatActionChoosingLocation(ChatActionChoosingLocation),
        ChatActionChoosingContact(ChatActionChoosingContact),
        ChatActionStartPlayingGame(ChatActionStartPlayingGame),
        ChatActionRecordingVideoNote(ChatActionRecordingVideoNote),
        ChatActionUploadingVideoNote(ChatActionUploadingVideoNote),
        ChatActionCancel(ChatActionCancel),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user status was never changed"]
    pub struct UserStatusEmpty {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is online"]
    pub struct UserStatusOnline {
        #[doc = "Point in time (Unix timestamp) when the user's online status will expire"]
        pub expires: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is offline"]
    pub struct UserStatusOffline {
        #[doc = "Point in time (Unix timestamp) when the user was last online"]
        pub was_online: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user was online recently"]
    pub struct UserStatusRecently {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is offline, but was online last week"]
    pub struct UserStatusLastWeek {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user is offline, but was online last month"]
    pub struct UserStatusLastMonth {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the last time the user was online"]
    pub enum UserStatus {
        UserStatusEmpty(UserStatusEmpty),
        UserStatusOnline(UserStatusOnline),
        UserStatusOffline(UserStatusOffline),
        UserStatusRecently(UserStatusRecently),
        UserStatusLastWeek(UserStatusLastWeek),
        UserStatusLastMonth(UserStatusLastMonth),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a list of stickers"]
    pub struct Stickers {
        #[doc = "List of stickers"]
        pub stickers: Vec<Sticker>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a list of emoji"]
    pub struct Emojis {
        #[doc = "List of emojis"]
        pub emojis: Vec<String>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a sticker set"]
    pub struct StickerSet {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the sticker set"]
        pub id: i64,
        #[doc = "Title of the sticker set"]
        pub title: String,
        #[doc = "Name of the sticker set"]
        pub name: String,
        #[serde(default)]
        #[doc = "Sticker set thumbnail in WEBP format with width and height 100; may be null. The file can be downloaded only before the thumbnail is changed"]
        pub thumbnail: Option<PhotoSize>,
        #[doc = "True, if the sticker set has been installed by the current user"]
        pub is_installed: bool,
        #[doc = "True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously"]
        pub is_archived: bool,
        #[doc = "True, if the sticker set is official"]
        pub is_official: bool,
        #[doc = "True, is the stickers in the set are animated"]
        pub is_animated: bool,
        #[doc = "True, if the stickers in the set are masks"]
        pub is_masks: bool,
        #[doc = "True for already viewed trending sticker sets"]
        pub is_viewed: bool,
        #[doc = "List of stickers in this set"]
        pub stickers: Vec<Sticker>,
        #[doc = "A list of emoji corresponding to the stickers in the same order. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object"]
        pub emojis: Vec<Emojis>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents short information about a sticker set"]
    pub struct StickerSetInfo {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the sticker set"]
        pub id: i64,
        #[doc = "Title of the sticker set"]
        pub title: String,
        #[doc = "Name of the sticker set"]
        pub name: String,
        #[serde(default)]
        #[doc = "Sticker set thumbnail in WEBP format with width and height 100; may be null"]
        pub thumbnail: Option<PhotoSize>,
        #[doc = "True, if the sticker set has been installed by current user"]
        pub is_installed: bool,
        #[doc = "True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously"]
        pub is_archived: bool,
        #[doc = "True, if the sticker set is official"]
        pub is_official: bool,
        #[doc = "True, is the stickers in the set are animated"]
        pub is_animated: bool,
        #[doc = "True, if the stickers in the set are masks"]
        pub is_masks: bool,
        #[doc = "True for already viewed trending sticker sets"]
        pub is_viewed: bool,
        #[doc = "Total number of stickers in the set"]
        pub size: i32,
        #[doc = "Contains up to the first 5 stickers from the set, depending on the context. If the client needs more stickers the full set should be requested"]
        pub covers: Vec<Sticker>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a list of sticker sets"]
    pub struct StickerSets {
        #[doc = "Approximate total number of sticker sets found"]
        pub total_count: i32,
        #[doc = "List of sticker sets"]
        pub sets: Vec<StickerSetInfo>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The call wasn't discarded, or the reason is unknown"]
    pub struct CallDiscardReasonEmpty {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The call was ended before the conversation started. It was cancelled by the caller or missed by the other party"]
    pub struct CallDiscardReasonMissed {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The call was ended before the conversation started. It was declined by the other party"]
    pub struct CallDiscardReasonDeclined {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The call was ended during the conversation because the users were disconnected"]
    pub struct CallDiscardReasonDisconnected {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The call was ended because one of the parties hung up"]
    pub struct CallDiscardReasonHungUp {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the reason why a call was discarded"]
    pub enum CallDiscardReason {
        CallDiscardReasonEmpty(CallDiscardReasonEmpty),
        CallDiscardReasonMissed(CallDiscardReasonMissed),
        CallDiscardReasonDeclined(CallDiscardReasonDeclined),
        CallDiscardReasonDisconnected(CallDiscardReasonDisconnected),
        CallDiscardReasonHungUp(CallDiscardReasonHungUp),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Specifies the supported call protocols"]
    pub struct CallProtocol {
        #[doc = "True, if UDP peer-to-peer connections are supported"]
        pub udp_p2p: bool,
        #[doc = "True, if connection through UDP reflectors is supported"]
        pub udp_reflector: bool,
        #[doc = "The minimum supported API layer; use 65"]
        pub min_layer: i32,
        #[doc = "The maximum supported API layer; use 65"]
        pub max_layer: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes the address of UDP reflectors"]
    pub struct CallConnection {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Reflector identifier"]
        pub id: i64,
        #[doc = "IPv4 reflector address"]
        pub ip: String,
        #[doc = "IPv6 reflector address"]
        pub ipv6: String,
        #[doc = "Reflector port number"]
        pub port: i32,
        #[doc = "Connection peer tag"]
        pub peer_tag: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains the call identifier"]
    pub struct CallId {
        #[doc = "Call identifier"]
        pub id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The call is pending, waiting to be accepted by a user"]
    pub struct CallStatePending {
        #[doc = "True, if the call has already been created by the server"]
        pub is_created: bool,
        #[doc = "True, if the call has already been received by the other party"]
        pub is_received: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The call has been answered and encryption keys are being exchanged"]
    pub struct CallStateExchangingKeys {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The call is ready to use"]
    pub struct CallStateReady {
        #[doc = "Call protocols supported by the peer"]
        pub protocol: CallProtocol,
        #[doc = "Available UDP reflectors"]
        pub connections: Vec<CallConnection>,
        #[doc = "A JSON-encoded call config"]
        pub config: String,
        #[doc = "Call encryption key"]
        pub encryption_key: String,
        #[doc = "Encryption key emojis fingerprint"]
        pub emojis: Vec<String>,
        #[doc = "True, if peer-to-peer connection is allowed by users privacy settings"]
        pub allow_p2p: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The call is hanging up after discardCall has been called"]
    pub struct CallStateHangingUp {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The call has ended successfully"]
    pub struct CallStateDiscarded {
        #[doc = "The reason, why the call has ended"]
        pub reason: CallDiscardReason,
        #[doc = "True, if the call rating should be sent to the server"]
        pub need_rating: bool,
        #[doc = "True, if the call debug information should be sent to the server"]
        pub need_debug_information: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The call has ended with an error"]
    pub struct CallStateError {
        #[doc = "Error. An error with the code 4005000 will be returned if an outgoing call is missed because of an expired timeout"]
        pub error: Error,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the current call state"]
    pub enum CallState {
        CallStatePending(CallStatePending),
        CallStateExchangingKeys(CallStateExchangingKeys),
        CallStateReady(CallStateReady),
        CallStateHangingUp(CallStateHangingUp),
        CallStateDiscarded(CallStateDiscarded),
        CallStateError(CallStateError),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user heard their own voice"]
    pub struct CallProblemEcho {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user heard background noise"]
    pub struct CallProblemNoise {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The other side kept disappearing"]
    pub struct CallProblemInterruptions {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The speech was distorted"]
    pub struct CallProblemDistortedSpeech {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user couldn't hear the other side"]
    pub struct CallProblemSilentLocal {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The other side couldn't hear the user"]
    pub struct CallProblemSilentRemote {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The call ended unexpectedly"]
    pub struct CallProblemDropped {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the exact type of a problem with a call"]
    pub enum CallProblem {
        CallProblemEcho(CallProblemEcho),
        CallProblemNoise(CallProblemNoise),
        CallProblemInterruptions(CallProblemInterruptions),
        CallProblemDistortedSpeech(CallProblemDistortedSpeech),
        CallProblemSilentLocal(CallProblemSilentLocal),
        CallProblemSilentRemote(CallProblemSilentRemote),
        CallProblemDropped(CallProblemDropped),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a call"]
    pub struct Call {
        #[doc = "Call identifier, not persistent"]
        pub id: i32,
        #[doc = "Peer user identifier"]
        pub user_id: i32,
        #[doc = "True, if the call is outgoing"]
        pub is_outgoing: bool,
        #[doc = "Call state"]
        pub state: CallState,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains settings for the authentication of the user's phone number"]
    pub struct PhoneNumberAuthenticationSettings {
        #[doc = "Pass true if the authentication code may be sent via flash call to the specified phone number"]
        pub allow_flash_call: bool,
        #[doc = "Pass true if the authenticated phone number is used on the current device"]
        pub is_current_phone_number: bool,
        #[doc = "For official applications only. True, if the app can use Android SMS Retriever API (requires Google Play Services >= 10.2) to automatically receive the authentication code from the SMS. See https://developers.google.com/identity/sms-retriever/ for more details"]
        pub allow_sms_retriever_api: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a list of animations"]
    pub struct Animations {
        #[doc = "List of animations"]
        pub animations: Vec<Animation>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents the result of an ImportContacts request"]
    pub struct ImportedContacts {
        #[doc = "User identifiers of the imported contacts in the same order as they were specified in the request; 0 if the contact is not yet a registered user"]
        pub user_ids: Vec<i32>,
        #[doc = "The number of users that imported the corresponding contact; 0 for already registered users or if unavailable"]
        pub importer_count: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains an HTTP URL"]
    pub struct HttpUrl {
        #[doc = "The URL"]
        pub url: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a link to an animated GIF"]
    pub struct InputInlineQueryResultAnimatedGif {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Title of the query result"]
        pub title: String,
        #[doc = "URL of the static result thumbnail (JPEG or GIF), if it exists"]
        pub thumbnail_url: String,
        #[doc = "The URL of the GIF-file (file size must not exceed 1MB)"]
        pub gif_url: String,
        #[doc = "Duration of the GIF, in seconds"]
        pub gif_duration: i32,
        #[doc = "Width of the GIF"]
        pub gif_width: i32,
        #[doc = "Height of the GIF"]
        pub gif_height: i32,
        #[serde(default)]
        #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageAnimation, InputMessageLocation, InputMessageVenue or InputMessageContact"]
        pub input_message_content: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a link to an animated (i.e. without sound) H.264/MPEG-4 AVC video"]
    pub struct InputInlineQueryResultAnimatedMpeg4 {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Title of the result"]
        pub title: String,
        #[doc = "URL of the static result thumbnail (JPEG or GIF), if it exists"]
        pub thumbnail_url: String,
        #[doc = "The URL of the MPEG4-file (file size must not exceed 1MB)"]
        pub mpeg4_url: String,
        #[doc = "Duration of the video, in seconds"]
        pub mpeg4_duration: i32,
        #[doc = "Width of the video"]
        pub mpeg4_width: i32,
        #[doc = "Height of the video"]
        pub mpeg4_height: i32,
        #[serde(default)]
        #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageAnimation, InputMessageLocation, InputMessageVenue or InputMessageContact"]
        pub input_message_content: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a link to an article or web page"]
    pub struct InputInlineQueryResultArticle {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "URL of the result, if it exists"]
        pub url: String,
        #[doc = "True, if the URL must be not shown"]
        pub hide_url: bool,
        #[doc = "Title of the result"]
        pub title: String,
        #[doc = "Represents a link to an article or web page"]
        pub description: String,
        #[doc = "URL of the result thumbnail, if it exists"]
        pub thumbnail_url: String,
        #[doc = "Thumbnail width, if known"]
        pub thumbnail_width: i32,
        #[doc = "Thumbnail height, if known"]
        pub thumbnail_height: i32,
        #[serde(default)]
        #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact"]
        pub input_message_content: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a link to an MP3 audio file"]
    pub struct InputInlineQueryResultAudio {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Title of the audio file"]
        pub title: String,
        #[doc = "Performer of the audio file"]
        pub performer: String,
        #[doc = "The URL of the audio file"]
        pub audio_url: String,
        #[doc = "Audio file duration, in seconds"]
        pub audio_duration: i32,
        #[serde(default)]
        #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageAudio, InputMessageLocation, InputMessageVenue or InputMessageContact"]
        pub input_message_content: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a user contact"]
    pub struct InputInlineQueryResultContact {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "User contact"]
        pub contact: Contact,
        #[doc = "URL of the result thumbnail, if it exists"]
        pub thumbnail_url: String,
        #[doc = "Thumbnail width, if known"]
        pub thumbnail_width: i32,
        #[doc = "Thumbnail height, if known"]
        pub thumbnail_height: i32,
        #[serde(default)]
        #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact"]
        pub input_message_content: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a link to a file"]
    pub struct InputInlineQueryResultDocument {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Title of the resulting file"]
        pub title: String,
        #[doc = "Represents a link to a file"]
        pub description: String,
        #[doc = "URL of the file"]
        pub document_url: String,
        #[doc = "MIME type of the file content; only \"application/pdf\" and \"application/zip\" are currently allowed"]
        pub mime_type: String,
        #[doc = "The URL of the file thumbnail, if it exists"]
        pub thumbnail_url: String,
        #[doc = "Width of the thumbnail"]
        pub thumbnail_width: i32,
        #[doc = "Height of the thumbnail"]
        pub thumbnail_height: i32,
        #[serde(default)]
        #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageDocument, InputMessageLocation, InputMessageVenue or InputMessageContact"]
        pub input_message_content: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a game"]
    pub struct InputInlineQueryResultGame {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Short name of the game"]
        pub game_short_name: String,
        #[serde(default)]
        #[doc = "Message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a point on the map"]
    pub struct InputInlineQueryResultLocation {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Location result"]
        pub location: Location,
        #[doc = "Amount of time relative to the message sent time until the location can be updated, in seconds"]
        pub live_period: i32,
        #[doc = "Title of the result"]
        pub title: String,
        #[doc = "URL of the result thumbnail, if it exists"]
        pub thumbnail_url: String,
        #[doc = "Thumbnail width, if known"]
        pub thumbnail_width: i32,
        #[doc = "Thumbnail height, if known"]
        pub thumbnail_height: i32,
        #[serde(default)]
        #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact"]
        pub input_message_content: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents link to a JPEG image"]
    pub struct InputInlineQueryResultPhoto {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Title of the result, if known"]
        pub title: String,
        #[doc = "Represents link to a JPEG image"]
        pub description: String,
        #[doc = "URL of the photo thumbnail, if it exists"]
        pub thumbnail_url: String,
        #[doc = "The URL of the JPEG photo (photo size must not exceed 5MB)"]
        pub photo_url: String,
        #[doc = "Width of the photo"]
        pub photo_width: i32,
        #[doc = "Height of the photo"]
        pub photo_height: i32,
        #[serde(default)]
        #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessagePhoto, InputMessageLocation, InputMessageVenue or InputMessageContact"]
        pub input_message_content: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a link to a WEBP or TGS sticker"]
    pub struct InputInlineQueryResultSticker {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "URL of the sticker thumbnail, if it exists"]
        pub thumbnail_url: String,
        #[doc = "The URL of the WEBP or TGS sticker (sticker file size must not exceed 5MB)"]
        pub sticker_url: String,
        #[doc = "Width of the sticker"]
        pub sticker_width: i32,
        #[doc = "Height of the sticker"]
        pub sticker_height: i32,
        #[serde(default)]
        #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, inputMessageSticker, InputMessageLocation, InputMessageVenue or InputMessageContact"]
        pub input_message_content: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents information about a venue"]
    pub struct InputInlineQueryResultVenue {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Venue result"]
        pub venue: Venue,
        #[doc = "URL of the result thumbnail, if it exists"]
        pub thumbnail_url: String,
        #[doc = "Thumbnail width, if known"]
        pub thumbnail_width: i32,
        #[doc = "Thumbnail height, if known"]
        pub thumbnail_height: i32,
        #[serde(default)]
        #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact"]
        pub input_message_content: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a link to a page containing an embedded video player or a video file"]
    pub struct InputInlineQueryResultVideo {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Title of the result"]
        pub title: String,
        #[doc = "Represents a link to a page containing an embedded video player or a video file"]
        pub description: String,
        #[doc = "The URL of the video thumbnail (JPEG), if it exists"]
        pub thumbnail_url: String,
        #[doc = "URL of the embedded video player or video file"]
        pub video_url: String,
        #[doc = "MIME type of the content of the video URL, only \"text/html\" or \"video/mp4\" are currently supported"]
        pub mime_type: String,
        #[doc = "Width of the video"]
        pub video_width: i32,
        #[doc = "Height of the video"]
        pub video_height: i32,
        #[doc = "Video duration, in seconds"]
        pub video_duration: i32,
        #[serde(default)]
        #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageVideo, InputMessageLocation, InputMessageVenue or InputMessageContact"]
        pub input_message_content: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a link to an opus-encoded audio file within an OGG container, single channel audio"]
    pub struct InputInlineQueryResultVoiceNote {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Title of the voice note"]
        pub title: String,
        #[doc = "The URL of the voice note file"]
        pub voice_note_url: String,
        #[doc = "Duration of the voice note, in seconds"]
        pub voice_note_duration: i32,
        #[serde(default)]
        #[doc = "The message reply markup. Must be of type replyMarkupInlineKeyboard or null"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageVoiceNote, InputMessageLocation, InputMessageVenue or InputMessageContact"]
        pub input_message_content: InputMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents a single result of an inline query; for bots only"]
    pub enum InputInlineQueryResult {
        InputInlineQueryResultAnimatedGif(InputInlineQueryResultAnimatedGif),
        InputInlineQueryResultAnimatedMpeg4(InputInlineQueryResultAnimatedMpeg4),
        InputInlineQueryResultArticle(InputInlineQueryResultArticle),
        InputInlineQueryResultAudio(InputInlineQueryResultAudio),
        InputInlineQueryResultContact(InputInlineQueryResultContact),
        InputInlineQueryResultDocument(InputInlineQueryResultDocument),
        InputInlineQueryResultGame(InputInlineQueryResultGame),
        InputInlineQueryResultLocation(InputInlineQueryResultLocation),
        InputInlineQueryResultPhoto(InputInlineQueryResultPhoto),
        InputInlineQueryResultSticker(InputInlineQueryResultSticker),
        InputInlineQueryResultVenue(InputInlineQueryResultVenue),
        InputInlineQueryResultVideo(InputInlineQueryResultVideo),
        InputInlineQueryResultVoiceNote(InputInlineQueryResultVoiceNote),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a link to an article or web page"]
    pub struct InlineQueryResultArticle {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "URL of the result, if it exists"]
        pub url: String,
        #[doc = "True, if the URL must be not shown"]
        pub hide_url: bool,
        #[doc = "Title of the result"]
        pub title: String,
        #[doc = "Represents a link to an article or web page"]
        pub description: String,
        #[serde(default)]
        #[doc = "Result thumbnail; may be null"]
        pub thumbnail: Option<PhotoSize>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a user contact"]
    pub struct InlineQueryResultContact {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "A user contact"]
        pub contact: Contact,
        #[serde(default)]
        #[doc = "Result thumbnail; may be null"]
        pub thumbnail: Option<PhotoSize>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a point on the map"]
    pub struct InlineQueryResultLocation {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Location result"]
        pub location: Location,
        #[doc = "Title of the result"]
        pub title: String,
        #[serde(default)]
        #[doc = "Result thumbnail; may be null"]
        pub thumbnail: Option<PhotoSize>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents information about a venue"]
    pub struct InlineQueryResultVenue {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Venue result"]
        pub venue: Venue,
        #[serde(default)]
        #[doc = "Result thumbnail; may be null"]
        pub thumbnail: Option<PhotoSize>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents information about a game"]
    pub struct InlineQueryResultGame {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Game result"]
        pub game: Game,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents an animation file"]
    pub struct InlineQueryResultAnimation {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Animation file"]
        pub animation: Animation,
        #[doc = "Animation title"]
        pub title: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents an audio file"]
    pub struct InlineQueryResultAudio {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Audio file"]
        pub audio: Audio,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a document"]
    pub struct InlineQueryResultDocument {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Document"]
        pub document: Document,
        #[doc = "Document title"]
        pub title: String,
        #[doc = "Represents a document"]
        pub description: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a photo"]
    pub struct InlineQueryResultPhoto {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Photo"]
        pub photo: Photo,
        #[doc = "Title of the result, if known"]
        pub title: String,
        #[doc = "Represents a photo"]
        pub description: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a sticker"]
    pub struct InlineQueryResultSticker {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Sticker"]
        pub sticker: Sticker,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a video"]
    pub struct InlineQueryResultVideo {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Video"]
        pub video: Video,
        #[doc = "Title of the video"]
        pub title: String,
        #[doc = "Represents a video"]
        pub description: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a voice note"]
    pub struct InlineQueryResultVoiceNote {
        #[doc = "Unique identifier of the query result"]
        pub id: String,
        #[doc = "Voice note"]
        pub voice_note: VoiceNote,
        #[doc = "Title of the voice note"]
        pub title: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents a single result of an inline query"]
    pub enum InlineQueryResult {
        InlineQueryResultArticle(InlineQueryResultArticle),
        InlineQueryResultContact(InlineQueryResultContact),
        InlineQueryResultLocation(InlineQueryResultLocation),
        InlineQueryResultVenue(InlineQueryResultVenue),
        InlineQueryResultGame(Box<InlineQueryResultGame>),
        InlineQueryResultAnimation(InlineQueryResultAnimation),
        InlineQueryResultAudio(InlineQueryResultAudio),
        InlineQueryResultDocument(InlineQueryResultDocument),
        InlineQueryResultPhoto(InlineQueryResultPhoto),
        InlineQueryResultSticker(InlineQueryResultSticker),
        InlineQueryResultVideo(InlineQueryResultVideo),
        InlineQueryResultVoiceNote(InlineQueryResultVoiceNote),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query"]
    pub struct InlineQueryResults {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique identifier of the inline query"]
        pub inline_query_id: i64,
        #[doc = "The offset for the next request. If empty, there are no more results"]
        pub next_offset: String,
        #[doc = "Results of the query"]
        pub results: Vec<InlineQueryResult>,
        #[doc = "If non-empty, this text should be shown on the button, which opens a private chat with the bot and sends the bot a start message with the switch_pm_parameter"]
        pub switch_pm_text: String,
        #[doc = "Parameter for the bot start message"]
        pub switch_pm_parameter: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The payload from a general callback button"]
    pub struct CallbackQueryPayloadData {
        #[doc = "Data that was attached to the callback button"]
        pub data: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The payload from a game callback button"]
    pub struct CallbackQueryPayloadGame {
        #[doc = "A short name of the game that was attached to the callback button"]
        pub game_short_name: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents a payload of a callback query"]
    pub enum CallbackQueryPayload {
        CallbackQueryPayloadData(CallbackQueryPayloadData),
        CallbackQueryPayloadGame(CallbackQueryPayloadGame),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a bot's answer to a callback query"]
    pub struct CallbackQueryAnswer {
        #[doc = "Text of the answer"]
        pub text: String,
        #[doc = "True, if an alert should be shown to the user instead of a toast notification"]
        pub show_alert: bool,
        #[doc = "URL to be opened"]
        pub url: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains the result of a custom request"]
    pub struct CustomRequestResult {
        #[doc = "A JSON-serialized result"]
        pub result: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains one row of the game high score table"]
    pub struct GameHighScore {
        #[doc = "Position in the high score table"]
        pub position: i32,
        #[doc = "User identifier"]
        pub user_id: i32,
        #[doc = "User score"]
        pub score: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of game high scores"]
    pub struct GameHighScores {
        #[doc = "A list of game high scores"]
        pub scores: Vec<GameHighScore>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message was edited"]
    pub struct ChatEventMessageEdited {
        #[doc = "The original message before the edit"]
        pub old_message: Message,
        #[doc = "The message after it was edited"]
        pub new_message: Message,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message was deleted"]
    pub struct ChatEventMessageDeleted {
        #[doc = "Deleted message"]
        pub message: Message,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A poll in a message was stopped"]
    pub struct ChatEventPollStopped {
        #[doc = "The message with the poll"]
        pub message: Message,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message was pinned"]
    pub struct ChatEventMessagePinned {
        #[doc = "Pinned message"]
        pub message: Message,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message was unpinned"]
    pub struct ChatEventMessageUnpinned {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new member joined the chat"]
    pub struct ChatEventMemberJoined {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A member left the chat"]
    pub struct ChatEventMemberLeft {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new chat member was invited"]
    pub struct ChatEventMemberInvited {
        #[doc = "New member user identifier"]
        pub user_id: i32,
        #[doc = "New member status"]
        pub status: ChatMemberStatus,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat member has gained/lost administrator status, or the list of their administrator privileges has changed"]
    pub struct ChatEventMemberPromoted {
        #[doc = "Chat member user identifier"]
        pub user_id: i32,
        #[doc = "Previous status of the chat member"]
        pub old_status: ChatMemberStatus,
        #[doc = "New status of the chat member"]
        pub new_status: ChatMemberStatus,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat member was restricted/unrestricted or banned/unbanned, or the list of their restrictions has changed"]
    pub struct ChatEventMemberRestricted {
        #[doc = "Chat member user identifier"]
        pub user_id: i32,
        #[doc = "Previous status of the chat member"]
        pub old_status: ChatMemberStatus,
        #[doc = "New status of the chat member"]
        pub new_status: ChatMemberStatus,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat title was changed"]
    pub struct ChatEventTitleChanged {
        #[doc = "Previous chat title"]
        pub old_title: String,
        #[doc = "New chat title"]
        pub new_title: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat permissions was changed"]
    pub struct ChatEventPermissionsChanged {
        #[doc = "Previous chat permissions"]
        pub old_permissions: ChatPermissions,
        #[doc = "New chat permissions"]
        pub new_permissions: ChatPermissions,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat description was changed"]
    pub struct ChatEventDescriptionChanged {
        #[doc = "Previous chat description"]
        pub old_description: String,
        #[doc = "New chat description"]
        pub new_description: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat username was changed"]
    pub struct ChatEventUsernameChanged {
        #[doc = "Previous chat username"]
        pub old_username: String,
        #[doc = "New chat username"]
        pub new_username: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat photo was changed"]
    pub struct ChatEventPhotoChanged {
        #[serde(default)]
        #[doc = "Previous chat photo value; may be null"]
        pub old_photo: Option<Photo>,
        #[serde(default)]
        #[doc = "New chat photo value; may be null"]
        pub new_photo: Option<Photo>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The can_invite_users permission of a supergroup chat was toggled"]
    pub struct ChatEventInvitesToggled {
        #[doc = "New value of can_invite_users permission"]
        pub can_invite_users: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The linked chat of a supergroup was changed"]
    pub struct ChatEventLinkedChatChanged {
        #[doc = "Previous supergroup linked chat identifier"]
        pub old_linked_chat_id: i64,
        #[doc = "New supergroup linked chat identifier"]
        pub new_linked_chat_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The slow_mode_delay setting of a supergroup was changed"]
    pub struct ChatEventSlowModeDelayChanged {
        #[doc = "Previous value of slow_mode_delay"]
        pub old_slow_mode_delay: i32,
        #[doc = "New value of slow_mode_delay"]
        pub new_slow_mode_delay: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The sign_messages setting of a channel was toggled"]
    pub struct ChatEventSignMessagesToggled {
        #[doc = "New value of sign_messages"]
        pub sign_messages: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The supergroup sticker set was changed"]
    pub struct ChatEventStickerSetChanged {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Previous identifier of the chat sticker set; 0 if none"]
        pub old_sticker_set_id: i64,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "New identifier of the chat sticker set; 0 if none"]
        pub new_sticker_set_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The supergroup location was changed"]
    pub struct ChatEventLocationChanged {
        #[serde(default)]
        #[doc = "Previous location; may be null"]
        pub old_location: Option<ChatLocation>,
        #[serde(default)]
        #[doc = "New location; may be null"]
        pub new_location: Option<ChatLocation>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The is_all_history_available setting of a supergroup was toggled"]
    pub struct ChatEventIsAllHistoryAvailableToggled {
        #[doc = "New value of is_all_history_available"]
        pub is_all_history_available: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents a chat event"]
    pub enum ChatEventAction {
        ChatEventMessageEdited(ChatEventMessageEdited),
        ChatEventMessageDeleted(ChatEventMessageDeleted),
        ChatEventPollStopped(ChatEventPollStopped),
        ChatEventMessagePinned(ChatEventMessagePinned),
        ChatEventMessageUnpinned(ChatEventMessageUnpinned),
        ChatEventMemberJoined(ChatEventMemberJoined),
        ChatEventMemberLeft(ChatEventMemberLeft),
        ChatEventMemberInvited(ChatEventMemberInvited),
        ChatEventMemberPromoted(ChatEventMemberPromoted),
        ChatEventMemberRestricted(ChatEventMemberRestricted),
        ChatEventTitleChanged(ChatEventTitleChanged),
        ChatEventPermissionsChanged(ChatEventPermissionsChanged),
        ChatEventDescriptionChanged(ChatEventDescriptionChanged),
        ChatEventUsernameChanged(ChatEventUsernameChanged),
        ChatEventPhotoChanged(ChatEventPhotoChanged),
        ChatEventInvitesToggled(ChatEventInvitesToggled),
        ChatEventLinkedChatChanged(ChatEventLinkedChatChanged),
        ChatEventSlowModeDelayChanged(ChatEventSlowModeDelayChanged),
        ChatEventSignMessagesToggled(ChatEventSignMessagesToggled),
        ChatEventStickerSetChanged(ChatEventStickerSetChanged),
        ChatEventLocationChanged(ChatEventLocationChanged),
        ChatEventIsAllHistoryAvailableToggled(ChatEventIsAllHistoryAvailableToggled),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a chat event"]
    pub struct ChatEvent {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Chat event identifier"]
        pub id: i64,
        #[doc = "Point in time (Unix timestamp) when the event happened"]
        pub date: i32,
        #[doc = "Identifier of the user who performed the action that triggered the event"]
        pub user_id: i32,
        #[doc = "Action performed by the user"]
        pub action: ChatEventAction,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of chat events"]
    pub struct ChatEvents {
        #[doc = "List of events"]
        pub events: Vec<ChatEvent>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a set of filters used to obtain a chat event log"]
    pub struct ChatEventLogFilters {
        #[doc = "True, if message edits should be returned"]
        pub message_edits: bool,
        #[doc = "True, if message deletions should be returned"]
        pub message_deletions: bool,
        #[doc = "True, if pin/unpin events should be returned"]
        pub message_pins: bool,
        #[doc = "True, if members joining events should be returned"]
        pub member_joins: bool,
        #[doc = "True, if members leaving events should be returned"]
        pub member_leaves: bool,
        #[doc = "True, if invited member events should be returned"]
        pub member_invites: bool,
        #[doc = "True, if member promotion/demotion events should be returned"]
        pub member_promotions: bool,
        #[doc = "True, if member restricted/unrestricted/banned/unbanned events should be returned"]
        pub member_restrictions: bool,
        #[doc = "True, if changes in chat information should be returned"]
        pub info_changes: bool,
        #[doc = "True, if changes in chat settings should be returned"]
        pub setting_changes: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An ordinary language pack string"]
    pub struct LanguagePackStringValueOrdinary {
        #[doc = "String value"]
        pub value: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A language pack string which has different forms based on the number of some object it mentions. See https://www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more info"]
    pub struct LanguagePackStringValuePluralized {
        #[doc = "Value for zero objects"]
        pub zero_value: String,
        #[doc = "Value for one object"]
        pub one_value: String,
        #[doc = "Value for two objects"]
        pub two_value: String,
        #[doc = "Value for few objects"]
        pub few_value: String,
        #[doc = "Value for many objects"]
        pub many_value: String,
        #[doc = "Default value"]
        pub other_value: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A deleted language pack string, the value should be taken from the built-in english language pack"]
    pub struct LanguagePackStringValueDeleted {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents the value of a string in a language pack"]
    pub enum LanguagePackStringValue {
        LanguagePackStringValueOrdinary(LanguagePackStringValueOrdinary),
        LanguagePackStringValuePluralized(LanguagePackStringValuePluralized),
        LanguagePackStringValueDeleted(LanguagePackStringValueDeleted),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents one language pack string"]
    pub struct LanguagePackString {
        #[doc = "String key"]
        pub key: String,
        #[doc = "String value"]
        pub value: LanguagePackStringValue,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of language pack strings"]
    pub struct LanguagePackStrings {
        #[doc = "A list of language pack strings"]
        pub strings: Vec<LanguagePackString>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a language pack"]
    pub struct LanguagePackInfo {
        #[doc = "Unique language pack identifier"]
        pub id: String,
        #[doc = "Identifier of a base language pack; may be empty. If a string is missed in the language pack, then it should be fetched from base language pack. Unsupported in custom language packs"]
        pub base_language_pack_id: String,
        #[doc = "Language name"]
        pub name: String,
        #[doc = "Name of the language in that language"]
        pub native_name: String,
        #[doc = "A language code to be used to apply plural forms. See https://www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more info"]
        pub plural_code: String,
        #[doc = "True, if the language pack is official"]
        pub is_official: bool,
        #[doc = "True, if the language pack strings are RTL"]
        pub is_rtl: bool,
        #[doc = "True, if the language pack is a beta language pack"]
        pub is_beta: bool,
        #[doc = "True, if the language pack is installed by the current user"]
        pub is_installed: bool,
        #[doc = "Total number of non-deleted strings from the language pack"]
        pub total_string_count: i32,
        #[doc = "Total number of translated strings from the language pack"]
        pub translated_string_count: i32,
        #[doc = "Total number of non-deleted strings from the language pack available locally"]
        pub local_string_count: i32,
        #[doc = "Link to language translation interface; empty for custom local language packs"]
        pub translation_url: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about the current localization target"]
    pub struct LocalizationTargetInfo {
        #[doc = "List of available language packs for this application"]
        pub language_packs: Vec<LanguagePackInfo>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A token for Firebase Cloud Messaging"]
    pub struct DeviceTokenFirebaseCloudMessaging {
        #[doc = "Device registration token; may be empty to de-register a device"]
        pub token: String,
        #[doc = "True, if push notifications should be additionally encrypted"]
        pub encrypt: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A token for Apple Push Notification service"]
    pub struct DeviceTokenApplePush {
        #[doc = "Device token; may be empty to de-register a device"]
        pub device_token: String,
        #[doc = "True, if App Sandbox is enabled"]
        pub is_app_sandbox: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A token for Apple Push Notification service VoIP notifications"]
    pub struct DeviceTokenApplePushVoIP {
        #[doc = "Device token; may be empty to de-register a device"]
        pub device_token: String,
        #[doc = "True, if App Sandbox is enabled"]
        pub is_app_sandbox: bool,
        #[doc = "True, if push notifications should be additionally encrypted"]
        pub encrypt: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A token for Windows Push Notification Services"]
    pub struct DeviceTokenWindowsPush {
        #[doc = "The access token that will be used to send notifications; may be empty to de-register a device"]
        pub access_token: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A token for Microsoft Push Notification Service"]
    pub struct DeviceTokenMicrosoftPush {
        #[doc = "Push notification channel URI; may be empty to de-register a device"]
        pub channel_uri: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A token for Microsoft Push Notification Service VoIP channel"]
    pub struct DeviceTokenMicrosoftPushVoIP {
        #[doc = "Push notification channel URI; may be empty to de-register a device"]
        pub channel_uri: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A token for web Push API"]
    pub struct DeviceTokenWebPush {
        #[doc = "Absolute URL exposed by the push service where the application server can send push messages; may be empty to de-register a device"]
        pub endpoint: String,
        #[doc = "Base64url-encoded P-256 elliptic curve Diffie-Hellman public key"]
        pub p256dh_base64url: String,
        #[doc = "Base64url-encoded authentication secret"]
        pub auth_base64url: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A token for Simple Push API for Firefox OS"]
    pub struct DeviceTokenSimplePush {
        #[doc = "Absolute URL exposed by the push service where the application server can send push messages; may be empty to de-register a device"]
        pub endpoint: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A token for Ubuntu Push Client service"]
    pub struct DeviceTokenUbuntuPush {
        #[doc = "Token; may be empty to de-register a device"]
        pub token: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A token for BlackBerry Push Service"]
    pub struct DeviceTokenBlackBerryPush {
        #[doc = "Token; may be empty to de-register a device"]
        pub token: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A token for Tizen Push Service"]
    pub struct DeviceTokenTizenPush {
        #[doc = "Push service registration identifier; may be empty to de-register a device"]
        pub reg_id: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents a data needed to subscribe for push notifications through registerDevice method. To use specific push notification service, you must specify the correct application platform and upload valid server authentication data at https://my.telegram.org"]
    pub enum DeviceToken {
        DeviceTokenFirebaseCloudMessaging(DeviceTokenFirebaseCloudMessaging),
        DeviceTokenApplePush(DeviceTokenApplePush),
        DeviceTokenApplePushVoIP(DeviceTokenApplePushVoIP),
        DeviceTokenWindowsPush(DeviceTokenWindowsPush),
        DeviceTokenMicrosoftPush(DeviceTokenMicrosoftPush),
        DeviceTokenMicrosoftPushVoIP(DeviceTokenMicrosoftPushVoIP),
        DeviceTokenWebPush(DeviceTokenWebPush),
        DeviceTokenSimplePush(DeviceTokenSimplePush),
        DeviceTokenUbuntuPush(DeviceTokenUbuntuPush),
        DeviceTokenBlackBerryPush(DeviceTokenBlackBerryPush),
        DeviceTokenTizenPush(DeviceTokenTizenPush),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification"]
    pub struct PushReceiverId {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "The globally unique identifier of push notification subscription"]
        pub id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a solid fill of a background"]
    pub struct BackgroundFillSolid {
        #[doc = "A color of the background in the RGB24 format"]
        pub color: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a gradient fill of a background"]
    pub struct BackgroundFillGradient {
        #[doc = "A top color of the background in the RGB24 format"]
        pub top_color: i32,
        #[doc = "A bottom color of the background in the RGB24 format"]
        pub bottom_color: i32,
        #[doc = "Clockwise rotation angle of the gradient, in degrees; 0-359. Should be always divisible by 45"]
        pub rotation_angle: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes a fill of a background"]
    pub enum BackgroundFill {
        BackgroundFillSolid(BackgroundFillSolid),
        BackgroundFillGradient(BackgroundFillGradient),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A wallpaper in JPEG format"]
    pub struct BackgroundTypeWallpaper {
        #[doc = "True, if the wallpaper must be downscaled to fit in 450x450 square and then box-blurred with radius 12"]
        pub is_blurred: bool,
        #[doc = "True, if the background needs to be slightly moved when device is tilted"]
        pub is_moving: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A PNG or TGV (gzipped subset of SVG with MIME type \"application/x-tgwallpattern\") pattern to be combined with the background fill chosen by the user"]
    pub struct BackgroundTypePattern {
        #[doc = "Description of the background fill"]
        pub fill: BackgroundFill,
        #[doc = "Intensity of the pattern when it is shown above the filled background, 0-100"]
        pub intensity: i32,
        #[doc = "True, if the background needs to be slightly moved when device is tilted"]
        pub is_moving: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A filled background"]
    pub struct BackgroundTypeFill {
        #[doc = "Description of the background fill"]
        pub fill: BackgroundFill,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the type of a background"]
    pub enum BackgroundType {
        BackgroundTypeWallpaper(BackgroundTypeWallpaper),
        BackgroundTypePattern(BackgroundTypePattern),
        BackgroundTypeFill(BackgroundTypeFill),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a chat background"]
    pub struct Background {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique background identifier"]
        pub id: i64,
        #[doc = "True, if this is one of default backgrounds"]
        pub is_default: bool,
        #[doc = "True, if the background is dark and is recommended to be used with dark theme"]
        pub is_dark: bool,
        #[doc = "Unique background name"]
        pub name: String,
        #[serde(default)]
        #[doc = "Document with the background; may be null. Null only for filled backgrounds"]
        pub document: Option<Document>,
        #[serde(rename = "type")]
        #[doc = "Type of the background"]
        pub type_: BackgroundType,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of backgrounds"]
    pub struct Backgrounds {
        #[doc = "A list of backgrounds"]
        pub backgrounds: Vec<Background>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A background from a local file"]
    pub struct InputBackgroundLocal {
        #[doc = "Background file to use. Only inputFileLocal and inputFileGenerated are supported. The file must be in JPEG format for wallpapers and in PNG format for patterns"]
        pub background: InputFile,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A background from the server"]
    pub struct InputBackgroundRemote {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "The background identifier"]
        pub background_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains information about background to set"]
    pub enum InputBackground {
        InputBackgroundLocal(InputBackgroundLocal),
        InputBackgroundRemote(InputBackgroundRemote),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of hashtags"]
    pub struct Hashtags {
        #[doc = "A list of hashtags"]
        pub hashtags: Vec<String>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The session can be used"]
    pub struct CanTransferOwnershipResultOk {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The 2-step verification needs to be enabled first"]
    pub struct CanTransferOwnershipResultPasswordNeeded {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The 2-step verification was enabled recently, user needs to wait"]
    pub struct CanTransferOwnershipResultPasswordTooFresh {
        #[doc = "Time left before the session can be used to transfer ownership of a chat, in seconds"]
        pub retry_after: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The session was created recently, user needs to wait"]
    pub struct CanTransferOwnershipResultSessionTooFresh {
        #[doc = "Time left before the session can be used to transfer ownership of a chat, in seconds"]
        pub retry_after: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents result of checking whether the current session can be used to transfer a chat ownership to another user"]
    pub enum CanTransferOwnershipResult {
        CanTransferOwnershipResultOk(CanTransferOwnershipResultOk),
        CanTransferOwnershipResultPasswordNeeded(CanTransferOwnershipResultPasswordNeeded),
        CanTransferOwnershipResultPasswordTooFresh(CanTransferOwnershipResultPasswordTooFresh),
        CanTransferOwnershipResultSessionTooFresh(CanTransferOwnershipResultSessionTooFresh),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The username can be set"]
    pub struct CheckChatUsernameResultOk {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The username is invalid"]
    pub struct CheckChatUsernameResultUsernameInvalid {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The username is occupied"]
    pub struct CheckChatUsernameResultUsernameOccupied {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user has too much chats with username, one of them should be made private first"]
    pub struct CheckChatUsernameResultPublicChatsTooMuch {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user can't be a member of a public supergroup"]
    pub struct CheckChatUsernameResultPublicGroupsUnavailable {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents result of checking whether a username can be set for a chat"]
    pub enum CheckChatUsernameResult {
        CheckChatUsernameResultOk(CheckChatUsernameResultOk),
        CheckChatUsernameResultUsernameInvalid(CheckChatUsernameResultUsernameInvalid),
        CheckChatUsernameResultUsernameOccupied(CheckChatUsernameResultUsernameOccupied),
        CheckChatUsernameResultPublicChatsTooMuch(CheckChatUsernameResultPublicChatsTooMuch),
        CheckChatUsernameResultPublicGroupsUnavailable(
            CheckChatUsernameResultPublicGroupsUnavailable,
        ),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A general message with hidden content"]
    pub struct PushMessageContentHidden {
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An animation message (GIF-style)."]
    pub struct PushMessageContentAnimation {
        #[serde(default)]
        #[doc = "Message content; may be null"]
        pub animation: Option<Animation>,
        #[doc = "Animation caption"]
        pub caption: String,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An audio message"]
    pub struct PushMessageContentAudio {
        #[serde(default)]
        #[doc = "Message content; may be null"]
        pub audio: Option<Audio>,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a user contact"]
    pub struct PushMessageContentContact {
        #[doc = "Contact's name"]
        pub name: String,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A contact has registered with Telegram"]
    pub struct PushMessageContentContactRegistered {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A document message (a general file)"]
    pub struct PushMessageContentDocument {
        #[serde(default)]
        #[doc = "Message content; may be null"]
        pub document: Option<Document>,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a game"]
    pub struct PushMessageContentGame {
        #[doc = "Game title, empty for pinned game message"]
        pub title: String,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new high score was achieved in a game"]
    pub struct PushMessageContentGameScore {
        #[doc = "Game title, empty for pinned message"]
        pub title: String,
        #[doc = "New score, 0 for pinned message"]
        pub score: i32,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with an invoice from a bot"]
    pub struct PushMessageContentInvoice {
        #[doc = "Product price"]
        pub price: String,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a location"]
    pub struct PushMessageContentLocation {
        #[doc = "True, if the location is live"]
        pub is_live: bool,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A photo message"]
    pub struct PushMessageContentPhoto {
        #[serde(default)]
        #[doc = "Message content; may be null"]
        pub photo: Option<Photo>,
        #[doc = "Photo caption"]
        pub caption: String,
        #[doc = "True, if the photo is secret"]
        pub is_secret: bool,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a poll"]
    pub struct PushMessageContentPoll {
        #[doc = "Poll question"]
        pub question: String,
        #[doc = "True, if the poll is regular and not in quiz mode"]
        pub is_regular: bool,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A screenshot of a message in the chat has been taken"]
    pub struct PushMessageContentScreenshotTaken {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a sticker"]
    pub struct PushMessageContentSticker {
        #[serde(default)]
        #[doc = "Message content; may be null"]
        pub sticker: Option<Sticker>,
        #[doc = "Emoji corresponding to the sticker; may be empty"]
        pub emoji: String,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A text message"]
    pub struct PushMessageContentText {
        #[doc = "Message text"]
        pub text: String,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A video message"]
    pub struct PushMessageContentVideo {
        #[serde(default)]
        #[doc = "Message content; may be null"]
        pub video: Option<Video>,
        #[doc = "Video caption"]
        pub caption: String,
        #[doc = "True, if the video is secret"]
        pub is_secret: bool,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A video note message"]
    pub struct PushMessageContentVideoNote {
        #[serde(default)]
        #[doc = "Message content; may be null"]
        pub video_note: Option<VideoNote>,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A voice note message"]
    pub struct PushMessageContentVoiceNote {
        #[serde(default)]
        #[doc = "Message content; may be null"]
        pub voice_note: Option<VoiceNote>,
        #[doc = "True, if the message is a pinned message with the specified content"]
        pub is_pinned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A newly created basic group"]
    pub struct PushMessageContentBasicGroupChatCreate {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "New chat members were invited to a group"]
    pub struct PushMessageContentChatAddMembers {
        #[doc = "Name of the added member"]
        pub member_name: String,
        #[doc = "True, if the current user was added to the group"]
        pub is_current_user: bool,
        #[doc = "True, if the user has returned to the group themself"]
        pub is_returned: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat photo was edited"]
    pub struct PushMessageContentChatChangePhoto {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat title was edited"]
    pub struct PushMessageContentChatChangeTitle {
        #[doc = "New chat title"]
        pub title: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat member was deleted"]
    pub struct PushMessageContentChatDeleteMember {
        #[doc = "Name of the deleted member"]
        pub member_name: String,
        #[doc = "True, if the current user was deleted from the group"]
        pub is_current_user: bool,
        #[doc = "True, if the user has left the group themself"]
        pub is_left: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new member joined the chat by invite link"]
    pub struct PushMessageContentChatJoinByLink {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A forwarded messages"]
    pub struct PushMessageContentMessageForwards {
        #[doc = "Number of forwarded messages"]
        pub total_count: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A media album"]
    pub struct PushMessageContentMediaAlbum {
        #[doc = "Number of messages in the album"]
        pub total_count: i32,
        #[doc = "True, if the album has at least one photo"]
        pub has_photos: bool,
        #[doc = "True, if the album has at least one video"]
        pub has_videos: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains content of a push message notification"]
    pub enum PushMessageContent {
        PushMessageContentHidden(PushMessageContentHidden),
        PushMessageContentAnimation(PushMessageContentAnimation),
        PushMessageContentAudio(PushMessageContentAudio),
        PushMessageContentContact(PushMessageContentContact),
        PushMessageContentContactRegistered(PushMessageContentContactRegistered),
        PushMessageContentDocument(PushMessageContentDocument),
        PushMessageContentGame(PushMessageContentGame),
        PushMessageContentGameScore(PushMessageContentGameScore),
        PushMessageContentInvoice(PushMessageContentInvoice),
        PushMessageContentLocation(PushMessageContentLocation),
        PushMessageContentPhoto(PushMessageContentPhoto),
        PushMessageContentPoll(PushMessageContentPoll),
        PushMessageContentScreenshotTaken(PushMessageContentScreenshotTaken),
        PushMessageContentSticker(PushMessageContentSticker),
        PushMessageContentText(PushMessageContentText),
        PushMessageContentVideo(PushMessageContentVideo),
        PushMessageContentVideoNote(PushMessageContentVideoNote),
        PushMessageContentVoiceNote(PushMessageContentVoiceNote),
        PushMessageContentBasicGroupChatCreate(PushMessageContentBasicGroupChatCreate),
        PushMessageContentChatAddMembers(PushMessageContentChatAddMembers),
        PushMessageContentChatChangePhoto(PushMessageContentChatChangePhoto),
        PushMessageContentChatChangeTitle(PushMessageContentChatChangeTitle),
        PushMessageContentChatDeleteMember(PushMessageContentChatDeleteMember),
        PushMessageContentChatJoinByLink(PushMessageContentChatJoinByLink),
        PushMessageContentMessageForwards(PushMessageContentMessageForwards),
        PushMessageContentMediaAlbum(PushMessageContentMediaAlbum),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "New message was received"]
    pub struct NotificationTypeNewMessage {
        #[doc = "The message"]
        pub message: Message,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "New secret chat was created"]
    pub struct NotificationTypeNewSecretChat {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "New call was received"]
    pub struct NotificationTypeNewCall {
        #[doc = "Call identifier"]
        pub call_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "New message was received through a push notification"]
    pub struct NotificationTypeNewPushMessage {
        #[doc = "The message identifier. The message will not be available in the chat history, but the ID can be used in viewMessages and as reply_to_message_id"]
        pub message_id: i64,
        #[doc = "Sender of the message. Corresponding user may be inaccessible"]
        pub sender_user_id: i32,
        #[doc = "Push message content"]
        pub content: PushMessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains detailed information about a notification"]
    pub enum NotificationType {
        NotificationTypeNewMessage(NotificationTypeNewMessage),
        NotificationTypeNewSecretChat(NotificationTypeNewSecretChat),
        NotificationTypeNewCall(NotificationTypeNewCall),
        NotificationTypeNewPushMessage(NotificationTypeNewPushMessage),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with ordinary unread messages"]
    pub struct NotificationGroupTypeMessages {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with unread mentions of the current user, replies to their messages, or a pinned message"]
    pub struct NotificationGroupTypeMentions {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A group containing a notification of type notificationTypeNewSecretChat"]
    pub struct NotificationGroupTypeSecretChat {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A group containing notifications of type notificationTypeNewCall"]
    pub struct NotificationGroupTypeCalls {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the type of notifications in a notification group"]
    pub enum NotificationGroupType {
        NotificationGroupTypeMessages(NotificationGroupTypeMessages),
        NotificationGroupTypeMentions(NotificationGroupTypeMentions),
        NotificationGroupTypeSecretChat(NotificationGroupTypeSecretChat),
        NotificationGroupTypeCalls(NotificationGroupTypeCalls),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a notification"]
    pub struct Notification {
        #[doc = "Unique persistent identifier of this notification"]
        pub id: i32,
        #[doc = "Notification date"]
        pub date: i32,
        #[doc = "True, if the notification was initially silent"]
        pub is_silent: bool,
        #[serde(rename = "type")]
        #[doc = "Notification type"]
        pub type_: NotificationType,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a group of notifications"]
    pub struct NotificationGroup {
        #[doc = "Unique persistent auto-incremented from 1 identifier of the notification group"]
        pub id: i32,
        #[serde(rename = "type")]
        #[doc = "Type of the group"]
        pub type_: NotificationGroupType,
        #[doc = "Identifier of a chat to which all notifications in the group belong"]
        pub chat_id: i64,
        #[doc = "Total number of active notifications in the group"]
        pub total_count: i32,
        #[doc = "The list of active notifications"]
        pub notifications: Vec<Notification>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a boolean option"]
    pub struct OptionValueBoolean {
        #[doc = "The value of the option"]
        pub value: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents an unknown option or an option which has a default value"]
    pub struct OptionValueEmpty {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents an integer option"]
    pub struct OptionValueInteger {
        #[doc = "The value of the option"]
        pub value: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a string option"]
    pub struct OptionValueString {
        #[doc = "The value of the option"]
        pub value: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents the value of an option"]
    pub enum OptionValue {
        OptionValueBoolean(OptionValueBoolean),
        OptionValueEmpty(OptionValueEmpty),
        OptionValueInteger(OptionValueInteger),
        OptionValueString(OptionValueString),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents one member of a JSON object"]
    pub struct JsonObjectMember {
        #[doc = "Member's key"]
        pub key: String,
        #[doc = "Member's value"]
        pub value: JsonValue,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a null JSON value"]
    pub struct JsonValueNull {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a boolean JSON value"]
    pub struct JsonValueBoolean {
        #[doc = "The value"]
        pub value: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a numeric JSON value"]
    pub struct JsonValueNumber {
        #[doc = "The value"]
        pub value: f64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a string JSON value"]
    pub struct JsonValueString {
        #[doc = "The value"]
        pub value: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a JSON array"]
    pub struct JsonValueArray {
        #[doc = "The list of array elements"]
        pub values: Vec<JsonValue>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a JSON object"]
    pub struct JsonValueObject {
        #[doc = "The list of object members"]
        pub members: Vec<JsonObjectMember>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents a JSON value"]
    pub enum JsonValue {
        JsonValueNull(JsonValueNull),
        JsonValueBoolean(JsonValueBoolean),
        JsonValueNumber(JsonValueNumber),
        JsonValueString(JsonValueString),
        JsonValueArray(JsonValueArray),
        JsonValueObject(JsonValueObject),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A rule to allow all users to do something"]
    pub struct UserPrivacySettingRuleAllowAll {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A rule to allow all of a user's contacts to do something"]
    pub struct UserPrivacySettingRuleAllowContacts {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A rule to allow certain specified users to do something"]
    pub struct UserPrivacySettingRuleAllowUsers {
        #[doc = "The user identifiers, total number of users in all rules must not exceed 1000"]
        pub user_ids: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A rule to allow all members of certain specified basic groups and supergroups to doing something"]
    pub struct UserPrivacySettingRuleAllowChatMembers {
        #[doc = "The chat identifiers, total number of chats in all rules must not exceed 20"]
        pub chat_ids: Vec<i64>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A rule to restrict all users from doing something"]
    pub struct UserPrivacySettingRuleRestrictAll {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A rule to restrict all contacts of a user from doing something"]
    pub struct UserPrivacySettingRuleRestrictContacts {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A rule to restrict all specified users from doing something"]
    pub struct UserPrivacySettingRuleRestrictUsers {
        #[doc = "The user identifiers, total number of users in all rules must not exceed 1000"]
        pub user_ids: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A rule to restrict all members of specified basic groups and supergroups from doing something"]
    pub struct UserPrivacySettingRuleRestrictChatMembers {
        #[doc = "The chat identifiers, total number of chats in all rules must not exceed 20"]
        pub chat_ids: Vec<i64>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents a single rule for managing privacy settings"]
    pub enum UserPrivacySettingRule {
        UserPrivacySettingRuleAllowAll(UserPrivacySettingRuleAllowAll),
        UserPrivacySettingRuleAllowContacts(UserPrivacySettingRuleAllowContacts),
        UserPrivacySettingRuleAllowUsers(UserPrivacySettingRuleAllowUsers),
        UserPrivacySettingRuleAllowChatMembers(UserPrivacySettingRuleAllowChatMembers),
        UserPrivacySettingRuleRestrictAll(UserPrivacySettingRuleRestrictAll),
        UserPrivacySettingRuleRestrictContacts(UserPrivacySettingRuleRestrictContacts),
        UserPrivacySettingRuleRestrictUsers(UserPrivacySettingRuleRestrictUsers),
        UserPrivacySettingRuleRestrictChatMembers(UserPrivacySettingRuleRestrictChatMembers),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed"]
    pub struct UserPrivacySettingRules {
        #[doc = "A list of rules"]
        pub rules: Vec<UserPrivacySettingRule>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A privacy setting for managing whether the user's online status is visible"]
    pub struct UserPrivacySettingShowStatus {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A privacy setting for managing whether the user's profile photo is visible"]
    pub struct UserPrivacySettingShowProfilePhoto {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A privacy setting for managing whether a link to the user's account is included in forwarded messages"]
    pub struct UserPrivacySettingShowLinkInForwardedMessages {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A privacy setting for managing whether the user's phone number is visible"]
    pub struct UserPrivacySettingShowPhoneNumber {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A privacy setting for managing whether the user can be invited to chats"]
    pub struct UserPrivacySettingAllowChatInvites {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A privacy setting for managing whether the user can be called"]
    pub struct UserPrivacySettingAllowCalls {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A privacy setting for managing whether peer-to-peer connections can be used for calls"]
    pub struct UserPrivacySettingAllowPeerToPeerCalls {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A privacy setting for managing whether the user can be found by their phone number. Checked only if the phone number is not known to the other user. Can be set only to \"Allow contacts\" or \"Allow all\""]
    pub struct UserPrivacySettingAllowFindingByPhoneNumber {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes available user privacy settings"]
    pub enum UserPrivacySetting {
        UserPrivacySettingShowStatus(UserPrivacySettingShowStatus),
        UserPrivacySettingShowProfilePhoto(UserPrivacySettingShowProfilePhoto),
        UserPrivacySettingShowLinkInForwardedMessages(
            UserPrivacySettingShowLinkInForwardedMessages,
        ),
        UserPrivacySettingShowPhoneNumber(UserPrivacySettingShowPhoneNumber),
        UserPrivacySettingAllowChatInvites(UserPrivacySettingAllowChatInvites),
        UserPrivacySettingAllowCalls(UserPrivacySettingAllowCalls),
        UserPrivacySettingAllowPeerToPeerCalls(UserPrivacySettingAllowPeerToPeerCalls),
        UserPrivacySettingAllowFindingByPhoneNumber(UserPrivacySettingAllowFindingByPhoneNumber),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about the period of inactivity after which the current user's account will automatically be deleted"]
    pub struct AccountTtl {
        #[doc = "Number of days of inactivity before the account will be flagged for deletion; should range from 30-366 days"]
        pub days: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about one session in a Telegram application used by the current user. Sessions should be shown to the user in the returned order"]
    pub struct Session {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Session identifier"]
        pub id: i64,
        #[doc = "True, if this session is the current session"]
        pub is_current: bool,
        #[doc = "True, if a password is needed to complete authorization of the session"]
        pub is_password_pending: bool,
        #[doc = "Telegram API identifier, as provided by the application"]
        pub api_id: i32,
        #[doc = "Name of the application, as provided by the application"]
        pub application_name: String,
        #[doc = "The version of the application, as provided by the application"]
        pub application_version: String,
        #[doc = "True, if the application is an official application or uses the api_id of an official application"]
        pub is_official_application: bool,
        #[doc = "Model of the device the application has been run or is running on, as provided by the application"]
        pub device_model: String,
        #[doc = "Operating system the application has been run or is running on, as provided by the application"]
        pub platform: String,
        #[doc = "Version of the operating system the application has been run or is running on, as provided by the application"]
        pub system_version: String,
        #[doc = "Point in time (Unix timestamp) when the user has logged in"]
        pub log_in_date: i32,
        #[doc = "Point in time (Unix timestamp) when the session was last used"]
        pub last_active_date: i32,
        #[doc = "IP address from which the session was created, in human-readable format"]
        pub ip: String,
        #[doc = "A two-letter country code for the country from which the session was created, based on the IP address"]
        pub country: String,
        #[doc = "Region code from which the session was created, based on the IP address"]
        pub region: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of sessions"]
    pub struct Sessions {
        #[doc = "List of sessions"]
        pub sessions: Vec<Session>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about one website the current user is logged in with Telegram"]
    pub struct ConnectedWebsite {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Website identifier"]
        pub id: i64,
        #[doc = "The domain name of the website"]
        pub domain_name: String,
        #[doc = "User identifier of a bot linked with the website"]
        pub bot_user_id: i32,
        #[doc = "The version of a browser used to log in"]
        pub browser: String,
        #[doc = "Operating system the browser is running on"]
        pub platform: String,
        #[doc = "Point in time (Unix timestamp) when the user was logged in"]
        pub log_in_date: i32,
        #[doc = "Point in time (Unix timestamp) when obtained authorization was last used"]
        pub last_active_date: i32,
        #[doc = "IP address from which the user was logged in, in human-readable format"]
        pub ip: String,
        #[doc = "Human-readable description of a country and a region, from which the user was logged in, based on the IP address"]
        pub location: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of websites the current user is logged in with Telegram"]
    pub struct ConnectedWebsites {
        #[doc = "List of connected websites"]
        pub websites: Vec<ConnectedWebsite>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat contains spam messages"]
    pub struct ChatReportReasonSpam {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat promotes violence"]
    pub struct ChatReportReasonViolence {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat contains pornographic messages"]
    pub struct ChatReportReasonPornography {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat has child abuse related content"]
    pub struct ChatReportReasonChildAbuse {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat contains copyrighted content"]
    pub struct ChatReportReasonCopyright {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The location-based chat is unrelated to its stated location"]
    pub struct ChatReportReasonUnrelatedLocation {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A custom reason provided by the user"]
    pub struct ChatReportReasonCustom {
        #[doc = "Report text"]
        pub text: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the reason why a chat is reported"]
    pub enum ChatReportReason {
        ChatReportReasonSpam(ChatReportReasonSpam),
        ChatReportReasonViolence(ChatReportReasonViolence),
        ChatReportReasonPornography(ChatReportReasonPornography),
        ChatReportReasonChildAbuse(ChatReportReasonChildAbuse),
        ChatReportReasonCopyright(ChatReportReasonCopyright),
        ChatReportReasonUnrelatedLocation(ChatReportReasonUnrelatedLocation),
        ChatReportReasonCustom(ChatReportReasonCustom),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a public HTTPS link to a message in a supergroup or channel with a username"]
    pub struct PublicMessageLink {
        #[doc = "Message link"]
        pub link: String,
        #[doc = "HTML-code for embedding the message"]
        pub html: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a link to a message in a chat"]
    pub struct MessageLinkInfo {
        #[doc = "True, if the link is a public link for a message in a chat"]
        pub is_public: bool,
        #[doc = "If found, identifier of the chat to which the message belongs, 0 otherwise"]
        pub chat_id: i64,
        #[serde(default)]
        #[doc = "If found, the linked message; may be null"]
        pub message: Option<Message>,
        #[doc = "True, if the whole media album to which the message belongs is linked"]
        pub for_album: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a part of a file"]
    pub struct FilePart {
        #[doc = "File bytes"]
        pub data: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The data is not a file"]
    pub struct FileTypeNone {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is an animation"]
    pub struct FileTypeAnimation {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is an audio file"]
    pub struct FileTypeAudio {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is a document"]
    pub struct FileTypeDocument {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is a photo"]
    pub struct FileTypePhoto {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is a profile photo"]
    pub struct FileTypeProfilePhoto {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file was sent to a secret chat (the file type is not known to the server)"]
    pub struct FileTypeSecret {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is a thumbnail of a file from a secret chat"]
    pub struct FileTypeSecretThumbnail {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is a file from Secure storage used for storing Telegram Passport files"]
    pub struct FileTypeSecure {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is a sticker"]
    pub struct FileTypeSticker {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is a thumbnail of another file"]
    pub struct FileTypeThumbnail {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file type is not yet known"]
    pub struct FileTypeUnknown {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is a video"]
    pub struct FileTypeVideo {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is a video note"]
    pub struct FileTypeVideoNote {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is a voice note"]
    pub struct FileTypeVoiceNote {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file is a wallpaper or a background pattern"]
    pub struct FileTypeWallpaper {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents the type of a file"]
    pub enum FileType {
        FileTypeNone(FileTypeNone),
        FileTypeAnimation(FileTypeAnimation),
        FileTypeAudio(FileTypeAudio),
        FileTypeDocument(FileTypeDocument),
        FileTypePhoto(FileTypePhoto),
        FileTypeProfilePhoto(FileTypeProfilePhoto),
        FileTypeSecret(FileTypeSecret),
        FileTypeSecretThumbnail(FileTypeSecretThumbnail),
        FileTypeSecure(FileTypeSecure),
        FileTypeSticker(FileTypeSticker),
        FileTypeThumbnail(FileTypeThumbnail),
        FileTypeUnknown(FileTypeUnknown),
        FileTypeVideo(FileTypeVideo),
        FileTypeVideoNote(FileTypeVideoNote),
        FileTypeVoiceNote(FileTypeVoiceNote),
        FileTypeWallpaper(FileTypeWallpaper),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains the storage usage statistics for a specific file type"]
    pub struct StorageStatisticsByFileType {
        #[doc = "File type"]
        pub file_type: FileType,
        #[doc = "Total size of the files"]
        pub size: i64,
        #[doc = "Total number of files"]
        pub count: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains the storage usage statistics for a specific chat"]
    pub struct StorageStatisticsByChat {
        #[doc = "Chat identifier; 0 if none"]
        pub chat_id: i64,
        #[doc = "Total size of the files in the chat"]
        pub size: i64,
        #[doc = "Total number of files in the chat"]
        pub count: i32,
        #[doc = "Statistics split by file types"]
        pub by_file_type: Vec<StorageStatisticsByFileType>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains the exact storage usage statistics split by chats and file type"]
    pub struct StorageStatistics {
        #[doc = "Total size of files"]
        pub size: i64,
        #[doc = "Total number of files"]
        pub count: i32,
        #[doc = "Statistics split by chats"]
        pub by_chat: Vec<StorageStatisticsByChat>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains approximate storage usage statistics, excluding files of unknown file type"]
    pub struct StorageStatisticsFast {
        #[doc = "Approximate total size of files"]
        pub files_size: i64,
        #[doc = "Approximate number of files"]
        pub file_count: i32,
        #[doc = "Size of the database"]
        pub database_size: i64,
        #[doc = "Size of the language pack database"]
        pub language_pack_database_size: i64,
        #[doc = "Size of the TDLib internal log"]
        pub log_size: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains database statistics"]
    pub struct DatabaseStatistics {
        #[doc = "Database statistics in an unspecified human-readable format"]
        pub statistics: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The network is not available"]
    pub struct NetworkTypeNone {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A mobile network"]
    pub struct NetworkTypeMobile {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A mobile roaming network"]
    pub struct NetworkTypeMobileRoaming {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A Wi-Fi network"]
    pub struct NetworkTypeWiFi {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A different network type (e.g., Ethernet network)"]
    pub struct NetworkTypeOther {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents the type of a network"]
    pub enum NetworkType {
        NetworkTypeNone(NetworkTypeNone),
        NetworkTypeMobile(NetworkTypeMobile),
        NetworkTypeMobileRoaming(NetworkTypeMobileRoaming),
        NetworkTypeWiFi(NetworkTypeWiFi),
        NetworkTypeOther(NetworkTypeOther),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about the total amount of data that was used to send and receive files"]
    pub struct NetworkStatisticsEntryFile {
        #[doc = "Type of the file the data is part of"]
        pub file_type: FileType,
        #[doc = "Type of the network the data was sent through. Call setNetworkType to maintain the actual network type"]
        pub network_type: NetworkType,
        #[doc = "Total number of bytes sent"]
        pub sent_bytes: i64,
        #[doc = "Total number of bytes received"]
        pub received_bytes: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about the total amount of data that was used for calls"]
    pub struct NetworkStatisticsEntryCall {
        #[doc = "Type of the network the data was sent through. Call setNetworkType to maintain the actual network type"]
        pub network_type: NetworkType,
        #[doc = "Total number of bytes sent"]
        pub sent_bytes: i64,
        #[doc = "Total number of bytes received"]
        pub received_bytes: i64,
        #[doc = "Total call duration, in seconds"]
        pub duration: f64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains statistics about network usage"]
    pub enum NetworkStatisticsEntry {
        NetworkStatisticsEntryFile(NetworkStatisticsEntryFile),
        NetworkStatisticsEntryCall(NetworkStatisticsEntryCall),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A full list of available network statistic entries"]
    pub struct NetworkStatistics {
        #[doc = "Point in time (Unix timestamp) when the app began collecting statistics"]
        pub since_date: i32,
        #[doc = "Network statistics entries"]
        pub entries: Vec<NetworkStatisticsEntry>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains auto-download settings"]
    pub struct AutoDownloadSettings {
        #[doc = "True, if the auto-download is enabled"]
        pub is_auto_download_enabled: bool,
        #[doc = "The maximum size of a photo file to be auto-downloaded"]
        pub max_photo_file_size: i32,
        #[doc = "The maximum size of a video file to be auto-downloaded"]
        pub max_video_file_size: i32,
        #[doc = "The maximum size of other file types to be auto-downloaded"]
        pub max_other_file_size: i32,
        #[doc = "The maximum suggested bitrate for uploaded videos"]
        pub video_upload_bitrate: i32,
        #[doc = "True, if the beginning of videos needs to be preloaded for instant playback"]
        pub preload_large_videos: bool,
        #[doc = "True, if the next audio track needs to be preloaded while the user is listening to an audio file"]
        pub preload_next_audio: bool,
        #[doc = "True, if \"use less data for calls\" option needs to be enabled"]
        pub use_less_data_for_calls: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains auto-download settings presets for the user"]
    pub struct AutoDownloadSettingsPresets {
        #[doc = "Preset with lowest settings; supposed to be used by default when roaming"]
        pub low: AutoDownloadSettings,
        #[doc = "Preset with medium settings; supposed to be used by default when using mobile data"]
        pub medium: AutoDownloadSettings,
        #[doc = "Preset with highest settings; supposed to be used by default when connected on Wi-Fi"]
        pub high: AutoDownloadSettings,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Currently waiting for the network to become available. Use setNetworkType to change the available network type"]
    pub struct ConnectionStateWaitingForNetwork {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Currently establishing a connection with a proxy server"]
    pub struct ConnectionStateConnectingToProxy {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Currently establishing a connection to the Telegram servers"]
    pub struct ConnectionStateConnecting {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Downloading data received while the client was offline"]
    pub struct ConnectionStateUpdating {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "There is a working connection to the Telegram servers"]
    pub struct ConnectionStateReady {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the current state of the connection to Telegram servers"]
    pub enum ConnectionState {
        ConnectionStateWaitingForNetwork(ConnectionStateWaitingForNetwork),
        ConnectionStateConnectingToProxy(ConnectionStateConnectingToProxy),
        ConnectionStateConnecting(ConnectionStateConnecting),
        ConnectionStateUpdating(ConnectionStateUpdating),
        ConnectionStateReady(ConnectionStateReady),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A category containing frequently used private chats with non-bot users"]
    pub struct TopChatCategoryUsers {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A category containing frequently used private chats with bot users"]
    pub struct TopChatCategoryBots {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A category containing frequently used basic groups and supergroups"]
    pub struct TopChatCategoryGroups {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A category containing frequently used channels"]
    pub struct TopChatCategoryChannels {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A category containing frequently used chats with inline bots sorted by their usage in inline mode"]
    pub struct TopChatCategoryInlineBots {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A category containing frequently used chats used for calls"]
    pub struct TopChatCategoryCalls {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A category containing frequently used chats used to forward messages"]
    pub struct TopChatCategoryForwardChats {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Represents the categories of chats for which a list of frequently used chats can be retrieved"]
    pub enum TopChatCategory {
        TopChatCategoryUsers(TopChatCategoryUsers),
        TopChatCategoryBots(TopChatCategoryBots),
        TopChatCategoryGroups(TopChatCategoryGroups),
        TopChatCategoryChannels(TopChatCategoryChannels),
        TopChatCategoryInlineBots(TopChatCategoryInlineBots),
        TopChatCategoryCalls(TopChatCategoryCalls),
        TopChatCategoryForwardChats(TopChatCategoryForwardChats),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A URL linking to a user"]
    pub struct TMeUrlTypeUser {
        #[doc = "Identifier of the user"]
        pub user_id: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A URL linking to a public supergroup or channel"]
    pub struct TMeUrlTypeSupergroup {
        #[doc = "Identifier of the supergroup or channel"]
        pub supergroup_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat invite link"]
    pub struct TMeUrlTypeChatInvite {
        #[doc = "Chat invite link info"]
        pub info: ChatInviteLinkInfo,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A URL linking to a sticker set"]
    pub struct TMeUrlTypeStickerSet {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the sticker set"]
        pub sticker_set_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the type of a URL linking to an internal Telegram entity"]
    pub enum TMeUrlType {
        TMeUrlTypeUser(TMeUrlTypeUser),
        TMeUrlTypeSupergroup(TMeUrlTypeSupergroup),
        TMeUrlTypeChatInvite(Box<TMeUrlTypeChatInvite>),
        TMeUrlTypeStickerSet(TMeUrlTypeStickerSet),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a URL linking to an internal Telegram entity"]
    pub struct TMeUrl {
        #[doc = "URL"]
        pub url: String,
        #[serde(rename = "type")]
        #[doc = "Type of the URL"]
        pub type_: TMeUrlType,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of t.me URLs"]
    pub struct TMeUrls {
        #[doc = "List of URLs"]
        pub urls: Vec<TMeUrl>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a counter"]
    pub struct Count {
        #[doc = "Count"]
        pub count: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains some text"]
    pub struct Text {
        #[doc = "Text"]
        pub text: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a value representing a number of seconds"]
    pub struct Seconds {
        #[doc = "Number of seconds"]
        pub seconds: f64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a tg:// deep link"]
    pub struct DeepLinkInfo {
        #[doc = "Text to be shown to the user"]
        pub text: FormattedText,
        #[doc = "True, if user should be asked to update the application"]
        pub need_update_application: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The text should be parsed in markdown-style"]
    pub struct TextParseModeMarkdown {
        #[doc = "Version of the parser: 0 or 1 - Bot API Markdown parse mode, 2 - Bot API MarkdownV2 parse mode"]
        pub version: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The text should be parsed in HTML-style"]
    pub struct TextParseModeHTML {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the way the text should be parsed for TextEntities"]
    pub enum TextParseMode {
        TextParseModeMarkdown(TextParseModeMarkdown),
        TextParseModeHTML(TextParseModeHTML),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A SOCKS5 proxy server"]
    pub struct ProxyTypeSocks5 {
        #[doc = "Username for logging in; may be empty"]
        pub username: String,
        #[doc = "Password for logging in; may be empty"]
        pub password: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A HTTP transparent proxy server"]
    pub struct ProxyTypeHttp {
        #[doc = "Username for logging in; may be empty"]
        pub username: String,
        #[doc = "Password for logging in; may be empty"]
        pub password: String,
        #[doc = "Pass true, if the proxy supports only HTTP requests and doesn't support transparent TCP connections via HTTP CONNECT method"]
        pub http_only: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An MTProto proxy server"]
    pub struct ProxyTypeMtproto {
        #[doc = "The proxy's secret in hexadecimal encoding"]
        pub secret: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes the type of a proxy server"]
    pub enum ProxyType {
        ProxyTypeSocks5(ProxyTypeSocks5),
        ProxyTypeHttp(ProxyTypeHttp),
        ProxyTypeMtproto(ProxyTypeMtproto),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains information about a proxy server"]
    pub struct Proxy {
        #[doc = "Unique identifier of the proxy"]
        pub id: i32,
        #[doc = "Proxy server IP address"]
        pub server: String,
        #[doc = "Proxy server port"]
        pub port: i32,
        #[doc = "Point in time (Unix timestamp) when the proxy was last used; 0 if never"]
        pub last_used_date: i32,
        #[doc = "True, if the proxy is enabled now"]
        pub is_enabled: bool,
        #[serde(rename = "type")]
        #[doc = "Type of the proxy"]
        pub type_: ProxyType,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Represents a list of proxy servers"]
    pub struct Proxies {
        #[doc = "List of proxy servers"]
        pub proxies: Vec<Proxy>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes a sticker that should be added to a sticker set"]
    pub struct InputSticker {
        #[doc = "PNG image with the sticker; must be up to 512 kB in size and fit in a 512x512 square"]
        pub png_sticker: InputFile,
        #[doc = "Emoji corresponding to the sticker"]
        pub emojis: String,
        #[serde(default)]
        #[doc = "For masks, position where the mask should be placed; may be null"]
        pub mask_position: Option<MaskPosition>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user authorization state has changed"]
    pub struct UpdateAuthorizationState {
        #[doc = "New authorization state"]
        pub authorization_state: AuthorizationState,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new message was received; can also be an outgoing message"]
    pub struct UpdateNewMessage {
        #[doc = "The new message"]
        pub message: Message,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option \"use_quick_ack\" is set to true. This update may be sent multiple times for the same message"]
    pub struct UpdateMessageSendAcknowledged {
        #[doc = "The chat identifier of the sent message"]
        pub chat_id: i64,
        #[doc = "A temporary message identifier"]
        pub message_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message has been successfully sent"]
    pub struct UpdateMessageSendSucceeded {
        #[doc = "Information about the sent message. Usually only the message identifier, date, and content are changed, but almost all other fields can also change"]
        pub message: Message,
        #[doc = "The previous temporary message identifier"]
        pub old_message_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update"]
    pub struct UpdateMessageSendFailed {
        #[doc = "Contains information about the message which failed to send"]
        pub message: Message,
        #[doc = "The previous temporary message identifier"]
        pub old_message_id: i64,
        #[doc = "An error code"]
        pub error_code: i32,
        #[doc = "Error message"]
        pub error_message: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The message content has changed"]
    pub struct UpdateMessageContent {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Message identifier"]
        pub message_id: i64,
        #[doc = "New message content"]
        pub new_content: MessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message was edited. Changes in the message content will come in a separate updateMessageContent"]
    pub struct UpdateMessageEdited {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Message identifier"]
        pub message_id: i64,
        #[doc = "Point in time (Unix timestamp) when the message was edited"]
        pub edit_date: i32,
        #[serde(default)]
        #[doc = "New message reply markup; may be null"]
        pub reply_markup: Option<ReplyMarkup>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The view count of the message has changed"]
    pub struct UpdateMessageViews {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Message identifier"]
        pub message_id: i64,
        #[doc = "New value of the view count"]
        pub views: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The message content was opened. Updates voice note messages to \"listened\", video note messages to \"viewed\" and starts the TTL timer for self-destructing messages"]
    pub struct UpdateMessageContentOpened {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Message identifier"]
        pub message_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with an unread mention was read"]
    pub struct UpdateMessageMentionRead {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Message identifier"]
        pub message_id: i64,
        #[doc = "The new number of unread mention messages left in the chat"]
        pub unread_mention_count: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A message with a live location was viewed. When the update is received, the client is supposed to update the live location"]
    pub struct UpdateMessageLiveLocationViewed {
        #[doc = "Identifier of the chat with the live location message"]
        pub chat_id: i64,
        #[doc = "Identifier of the message with live location"]
        pub message_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the client. The chat field changes will be reported through separate updates"]
    pub struct UpdateNewChat {
        #[doc = "The chat"]
        pub chat: Chat,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The list to which the chat belongs was changed. This update is guaranteed to be sent only when chat.order == 0 and the current or the new chat list is null"]
    pub struct UpdateChatChatList {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[serde(default)]
        #[doc = "The new chat's chat list; may be null"]
        pub chat_list: Option<ChatList>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The title of a chat was changed"]
    pub struct UpdateChatTitle {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "The new chat title"]
        pub title: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat photo was changed"]
    pub struct UpdateChatPhoto {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[serde(default)]
        #[doc = "The new chat photo; may be null"]
        pub photo: Option<ChatPhoto>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Chat permissions was changed"]
    pub struct UpdateChatPermissions {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "The new chat permissions"]
        pub permissions: ChatPermissions,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The last message of a chat was changed. If last_message is null, then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case"]
    pub struct UpdateChatLastMessage {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[serde(default)]
        #[doc = "The new last message in the chat; may be null"]
        pub last_message: Option<Message>,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "New value of the chat order"]
        pub order: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The order of the chat in the chat list has changed. Instead of this update updateChatLastMessage, updateChatIsPinned, updateChatDraftMessage, or updateChatIsSponsored might be sent"]
    pub struct UpdateChatOrder {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "New value of the order"]
        pub order: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat was pinned or unpinned"]
    pub struct UpdateChatIsPinned {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New value of is_pinned"]
        pub is_pinned: bool,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "New value of the chat order"]
        pub order: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat was marked as unread or was read"]
    pub struct UpdateChatIsMarkedAsUnread {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New value of is_marked_as_unread"]
        pub is_marked_as_unread: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat's is_sponsored field has changed"]
    pub struct UpdateChatIsSponsored {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New value of is_sponsored"]
        pub is_sponsored: bool,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "New value of chat order"]
        pub order: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat's has_scheduled_messages field has changed"]
    pub struct UpdateChatHasScheduledMessages {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New value of has_scheduled_messages"]
        pub has_scheduled_messages: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The value of the default disable_notification parameter, used when a message is sent to the chat, was changed"]
    pub struct UpdateChatDefaultDisableNotification {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "The new default_disable_notification value"]
        pub default_disable_notification: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Incoming messages were read or number of unread messages has been changed"]
    pub struct UpdateChatReadInbox {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Identifier of the last read incoming message"]
        pub last_read_inbox_message_id: i64,
        #[doc = "The number of unread messages left in the chat"]
        pub unread_count: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Outgoing messages were read"]
    pub struct UpdateChatReadOutbox {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Identifier of last read outgoing message"]
        pub last_read_outbox_message_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat unread_mention_count has changed"]
    pub struct UpdateChatUnreadMentionCount {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "The number of unread mention messages left in the chat"]
        pub unread_mention_count: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Notification settings for a chat were changed"]
    pub struct UpdateChatNotificationSettings {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "The new notification settings"]
        pub notification_settings: ChatNotificationSettings,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Notification settings for some type of chats were updated"]
    pub struct UpdateScopeNotificationSettings {
        #[doc = "Types of chats for which notification settings were updated"]
        pub scope: NotificationSettingsScope,
        #[doc = "The new notification settings"]
        pub notification_settings: ScopeNotificationSettings,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat action bar was changed"]
    pub struct UpdateChatActionBar {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[serde(default)]
        #[doc = "The new value of the action bar; may be null"]
        pub action_bar: Option<ChatActionBar>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The chat pinned message was changed"]
    pub struct UpdateChatPinnedMessage {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "The new identifier of the pinned message; 0 if there is no pinned message in the chat"]
        pub pinned_message_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user"]
    pub struct UpdateChatReplyMarkup {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Identifier of the message from which reply markup needs to be used; 0 if there is no default custom reply markup in the chat"]
        pub reply_markup_message_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update shouldn't be applied"]
    pub struct UpdateChatDraftMessage {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[serde(default)]
        #[doc = "The new draft message; may be null"]
        pub draft_message: Option<DraftMessage>,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "New value of the chat order"]
        pub order: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The number of online group members has changed. This update with non-zero count is sent only for currently opened chats. There is no guarantee that it will be sent just after the count has changed"]
    pub struct UpdateChatOnlineMemberCount {
        #[doc = "Identifier of the chat"]
        pub chat_id: i64,
        #[doc = "New number of online members in the chat, or 0 if unknown"]
        pub online_member_count: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A notification was changed"]
    pub struct UpdateNotification {
        #[doc = "Unique notification group identifier"]
        pub notification_group_id: i32,
        #[doc = "Changed notification"]
        pub notification: Notification,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A list of active notifications in a notification group has changed"]
    pub struct UpdateNotificationGroup {
        #[doc = "Unique notification group identifier"]
        pub notification_group_id: i32,
        #[serde(rename = "type")]
        #[doc = "New type of the notification group"]
        pub type_: NotificationGroupType,
        #[doc = "Identifier of a chat to which all notifications in the group belong"]
        pub chat_id: i64,
        #[doc = "Chat identifier, which notification settings must be applied to the added notifications"]
        pub notification_settings_chat_id: i64,
        #[doc = "True, if the notifications should be shown without sound"]
        pub is_silent: bool,
        #[doc = "Total number of unread notifications in the group, can be bigger than number of active notifications"]
        pub total_count: i32,
        #[doc = "List of added group notifications, sorted by notification ID"]
        pub added_notifications: Vec<Notification>,
        #[doc = "Identifiers of removed group notifications, sorted by notification ID"]
        pub removed_notification_ids: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains active notifications that was shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update"]
    pub struct UpdateActiveNotifications {
        #[doc = "Lists of active notification groups"]
        pub groups: Vec<NotificationGroup>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications"]
    pub struct UpdateHavePendingNotifications {
        #[doc = "True, if there are some delayed notification updates, which will be sent soon"]
        pub have_delayed_notifications: bool,
        #[doc = "True, if there can be some yet unreceived notifications, which are being fetched from the server"]
        pub have_unreceived_notifications: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Some messages were deleted"]
    pub struct UpdateDeleteMessages {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Identifiers of the deleted messages"]
        pub message_ids: Vec<i64>,
        #[doc = "True, if the messages are permanently deleted by a user (as opposed to just becoming inaccessible)"]
        pub is_permanent: bool,
        #[doc = "True, if the messages are deleted only from the cache and can possibly be retrieved again in the future"]
        pub from_cache: bool,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "User activity in the chat has changed"]
    pub struct UpdateUserChatAction {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Identifier of a user performing an action"]
        pub user_id: i32,
        #[doc = "The action description"]
        pub action: ChatAction,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user went online or offline"]
    pub struct UpdateUserStatus {
        #[doc = "User identifier"]
        pub user_id: i32,
        #[doc = "New status of the user"]
        pub status: UserStatus,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the client"]
    pub struct UpdateUser {
        #[doc = "New data about the user"]
        pub user: User,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the client"]
    pub struct UpdateBasicGroup {
        #[doc = "New data about the group"]
        pub basic_group: BasicGroup,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the client"]
    pub struct UpdateSupergroup {
        #[doc = "New data about the supergroup"]
        pub supergroup: Supergroup,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the client"]
    pub struct UpdateSecretChat {
        #[doc = "New data about the secret chat"]
        pub secret_chat: SecretChat,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Some data from userFullInfo has been changed"]
    pub struct UpdateUserFullInfo {
        #[doc = "User identifier"]
        pub user_id: i32,
        #[doc = "New full information about the user"]
        pub user_full_info: UserFullInfo,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Some data from basicGroupFullInfo has been changed"]
    pub struct UpdateBasicGroupFullInfo {
        #[doc = "Identifier of a basic group"]
        pub basic_group_id: i32,
        #[doc = "New full information about the group"]
        pub basic_group_full_info: BasicGroupFullInfo,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Some data from supergroupFullInfo has been changed"]
    pub struct UpdateSupergroupFullInfo {
        #[doc = "Identifier of the supergroup or channel"]
        pub supergroup_id: i32,
        #[doc = "New full information about the supergroup"]
        pub supergroup_full_info: SupergroupFullInfo,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Service notification from the server. Upon receiving this the client must show a popup with the content of the notification"]
    pub struct UpdateServiceNotification {
        #[serde(rename = "type")]
        #[doc = "Notification type. If type begins with \"AUTH_KEY_DROP_\", then two buttons \"Cancel\" and \"Log out\" should be shown under notification; if user presses the second, all local data should be destroyed using Destroy method"]
        pub type_: String,
        #[doc = "Notification content"]
        pub content: MessageContent,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Information about a file was updated"]
    pub struct UpdateFile {
        #[doc = "New data about the file"]
        pub file: File,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The file generation process needs to be started by the client"]
    pub struct UpdateFileGenerationStart {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique identifier for the generation process"]
        pub generation_id: i64,
        #[doc = "The path to a file from which a new file is generated; may be empty"]
        pub original_path: String,
        #[doc = "The path to a file that should be created and where the new file should be generated"]
        pub destination_path: String,
        #[doc = "String specifying the conversion applied to the original file. If conversion is \"#url#\" than original_path contains an HTTP/HTTPS URL of a file, which should be downloaded by the client"]
        pub conversion: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "File generation is no longer needed"]
    pub struct UpdateFileGenerationStop {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique identifier for the generation process"]
        pub generation_id: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "New call was created or information about a call was updated"]
    pub struct UpdateCall {
        #[doc = "New data about a call"]
        pub call: Call,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Some privacy setting rules have been changed"]
    pub struct UpdateUserPrivacySettingRules {
        #[doc = "The privacy setting"]
        pub setting: UserPrivacySetting,
        #[doc = "New privacy rules"]
        pub rules: UserPrivacySettingRules,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Number of unread messages in a chat list has changed. This update is sent only if the message database is used"]
    pub struct UpdateUnreadMessageCount {
        #[doc = "The chat list with changed number of unread messages"]
        pub chat_list: ChatList,
        #[doc = "Total number of unread messages"]
        pub unread_count: i32,
        #[doc = "Total number of unread messages in unmuted chats"]
        pub unread_unmuted_count: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used"]
    pub struct UpdateUnreadChatCount {
        #[doc = "The chat list with changed number of unread messages"]
        pub chat_list: ChatList,
        #[doc = "Approximate total number of chats in the chat list"]
        pub total_count: i32,
        #[doc = "Total number of unread chats"]
        pub unread_count: i32,
        #[doc = "Total number of unread unmuted chats"]
        pub unread_unmuted_count: i32,
        #[doc = "Total number of chats marked as unread"]
        pub marked_as_unread_count: i32,
        #[doc = "Total number of unmuted chats marked as unread"]
        pub marked_as_unread_unmuted_count: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "An option changed its value"]
    pub struct UpdateOption {
        #[doc = "The option name"]
        pub name: String,
        #[doc = "The new option value"]
        pub value: OptionValue,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The list of installed sticker sets was updated"]
    pub struct UpdateInstalledStickerSets {
        #[doc = "True, if the list of installed mask sticker sets was updated"]
        pub is_masks: bool,
        #[doc = "The new list of installed ordinary sticker sets"]
        pub sticker_set_ids: Vec<i64>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The list of trending sticker sets was updated or some of them were viewed"]
    pub struct UpdateTrendingStickerSets {
        #[doc = "The new list of trending sticker sets"]
        pub sticker_sets: StickerSets,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The list of recently used stickers was updated"]
    pub struct UpdateRecentStickers {
        #[doc = "True, if the list of stickers attached to photo or video files was updated, otherwise the list of sent stickers is updated"]
        pub is_attached: bool,
        #[doc = "The new list of file identifiers of recently used stickers"]
        pub sticker_ids: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The list of favorite stickers was updated"]
    pub struct UpdateFavoriteStickers {
        #[doc = "The new list of file identifiers of favorite stickers"]
        pub sticker_ids: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The list of saved animations was updated"]
    pub struct UpdateSavedAnimations {
        #[doc = "The new list of file identifiers of saved animations"]
        pub animation_ids: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The selected background has changed"]
    pub struct UpdateSelectedBackground {
        #[doc = "True, if background for dark theme has changed"]
        pub for_dark_theme: bool,
        #[serde(default)]
        #[doc = "The new selected background; may be null"]
        pub background: Option<Background>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Some language pack strings have been updated"]
    pub struct UpdateLanguagePackStrings {
        #[doc = "Localization target to which the language pack belongs"]
        pub localization_target: String,
        #[doc = "Identifier of the updated language pack"]
        pub language_pack_id: String,
        #[doc = "List of changed language pack strings"]
        pub strings: Vec<LanguagePackString>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The connection state has changed"]
    pub struct UpdateConnectionState {
        #[doc = "The new connection state"]
        pub state: ConnectionState,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method should be called with the reason \"Decline ToS update\""]
    pub struct UpdateTermsOfService {
        #[doc = "Identifier of the terms of service"]
        pub terms_of_service_id: String,
        #[doc = "The new terms of service"]
        pub terms_of_service: TermsOfService,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "List of users nearby has changed. The update is sent only 60 seconds after a successful searchChatsNearby request"]
    pub struct UpdateUsersNearby {
        #[doc = "The new list of users nearby"]
        pub users_nearby: Vec<ChatNearby>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new incoming inline query; for bots only"]
    pub struct UpdateNewInlineQuery {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique query identifier"]
        pub id: i64,
        #[doc = "Identifier of the user who sent the query"]
        pub sender_user_id: i32,
        #[serde(default)]
        #[doc = "User location, provided by the client; may be null"]
        pub user_location: Option<Location>,
        #[doc = "Text of the query"]
        pub query: String,
        #[doc = "Offset of the first entry to return"]
        pub offset: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The user has chosen a result of an inline query; for bots only"]
    pub struct UpdateNewChosenInlineResult {
        #[doc = "Identifier of the user who sent the query"]
        pub sender_user_id: i32,
        #[serde(default)]
        #[doc = "User location, provided by the client; may be null"]
        pub user_location: Option<Location>,
        #[doc = "Text of the query"]
        pub query: String,
        #[doc = "Identifier of the chosen result"]
        pub result_id: String,
        #[doc = "Identifier of the sent inline message, if known"]
        pub inline_message_id: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new incoming callback query; for bots only"]
    pub struct UpdateNewCallbackQuery {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique query identifier"]
        pub id: i64,
        #[doc = "Identifier of the user who sent the query"]
        pub sender_user_id: i32,
        #[doc = "Identifier of the chat where the query was sent"]
        pub chat_id: i64,
        #[doc = "Identifier of the message, from which the query originated"]
        pub message_id: i64,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier that uniquely corresponds to the chat to which the message was sent"]
        pub chat_instance: i64,
        #[doc = "Query payload"]
        pub payload: CallbackQueryPayload,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new incoming callback query from a message sent via a bot; for bots only"]
    pub struct UpdateNewInlineCallbackQuery {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique query identifier"]
        pub id: i64,
        #[doc = "Identifier of the user who sent the query"]
        pub sender_user_id: i32,
        #[doc = "Identifier of the inline message, from which the query originated"]
        pub inline_message_id: String,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "An identifier uniquely corresponding to the chat a message was sent to"]
        pub chat_instance: i64,
        #[doc = "Query payload"]
        pub payload: CallbackQueryPayload,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new incoming shipping query; for bots only. Only for invoices with flexible price"]
    pub struct UpdateNewShippingQuery {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique query identifier"]
        pub id: i64,
        #[doc = "Identifier of the user who sent the query"]
        pub sender_user_id: i32,
        #[doc = "Invoice payload"]
        pub invoice_payload: String,
        #[doc = "User shipping address"]
        pub shipping_address: Address,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new incoming pre-checkout query; for bots only. Contains full information about a checkout"]
    pub struct UpdateNewPreCheckoutQuery {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique query identifier"]
        pub id: i64,
        #[doc = "Identifier of the user who sent the query"]
        pub sender_user_id: i32,
        #[doc = "Currency for the product price"]
        pub currency: String,
        #[doc = "Total price for the product, in the minimal quantity of the currency"]
        pub total_amount: i64,
        #[doc = "Invoice payload"]
        pub invoice_payload: String,
        #[doc = "Identifier of a shipping option chosen by the user; may be empty if not applicable"]
        pub shipping_option_id: String,
        #[serde(default)]
        #[doc = "Information about the order; may be null"]
        pub order_info: Option<OrderInfo>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new incoming event; for bots only"]
    pub struct UpdateNewCustomEvent {
        #[doc = "A JSON-serialized event"]
        pub event: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A new incoming query; for bots only"]
    pub struct UpdateNewCustomQuery {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "The query identifier"]
        pub id: i64,
        #[doc = "JSON-serialized query data"]
        pub data: String,
        #[doc = "Query timeout"]
        pub timeout: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A poll was updated; for bots only"]
    pub struct UpdatePoll {
        #[doc = "New data about the poll"]
        pub poll: Poll,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A user changed the answer to a poll; for bots only"]
    pub struct UpdatePollAnswer {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Unique poll identifier"]
        pub poll_id: i64,
        #[doc = "The user, who changed the answer to the poll"]
        pub user_id: i32,
        #[doc = "0-based identifiers of answer options, chosen by the user"]
        pub option_ids: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Contains notifications about data changes"]
    pub enum Update {
        UpdateAuthorizationState(UpdateAuthorizationState),
        UpdateNewMessage(UpdateNewMessage),
        UpdateMessageSendAcknowledged(UpdateMessageSendAcknowledged),
        UpdateMessageSendSucceeded(UpdateMessageSendSucceeded),
        UpdateMessageSendFailed(UpdateMessageSendFailed),
        UpdateMessageContent(UpdateMessageContent),
        UpdateMessageEdited(UpdateMessageEdited),
        UpdateMessageViews(UpdateMessageViews),
        UpdateMessageContentOpened(UpdateMessageContentOpened),
        UpdateMessageMentionRead(UpdateMessageMentionRead),
        UpdateMessageLiveLocationViewed(UpdateMessageLiveLocationViewed),
        UpdateNewChat(Box<UpdateNewChat>),
        UpdateChatChatList(UpdateChatChatList),
        UpdateChatTitle(UpdateChatTitle),
        UpdateChatPhoto(UpdateChatPhoto),
        UpdateChatPermissions(UpdateChatPermissions),
        UpdateChatLastMessage(UpdateChatLastMessage),
        UpdateChatOrder(UpdateChatOrder),
        UpdateChatIsPinned(UpdateChatIsPinned),
        UpdateChatIsMarkedAsUnread(UpdateChatIsMarkedAsUnread),
        UpdateChatIsSponsored(UpdateChatIsSponsored),
        UpdateChatHasScheduledMessages(UpdateChatHasScheduledMessages),
        UpdateChatDefaultDisableNotification(UpdateChatDefaultDisableNotification),
        UpdateChatReadInbox(UpdateChatReadInbox),
        UpdateChatReadOutbox(UpdateChatReadOutbox),
        UpdateChatUnreadMentionCount(UpdateChatUnreadMentionCount),
        UpdateChatNotificationSettings(UpdateChatNotificationSettings),
        UpdateScopeNotificationSettings(UpdateScopeNotificationSettings),
        UpdateChatActionBar(UpdateChatActionBar),
        UpdateChatPinnedMessage(UpdateChatPinnedMessage),
        UpdateChatReplyMarkup(UpdateChatReplyMarkup),
        UpdateChatDraftMessage(UpdateChatDraftMessage),
        UpdateChatOnlineMemberCount(UpdateChatOnlineMemberCount),
        UpdateNotification(UpdateNotification),
        UpdateNotificationGroup(UpdateNotificationGroup),
        UpdateActiveNotifications(UpdateActiveNotifications),
        UpdateHavePendingNotifications(UpdateHavePendingNotifications),
        UpdateDeleteMessages(UpdateDeleteMessages),
        UpdateUserChatAction(UpdateUserChatAction),
        UpdateUserStatus(UpdateUserStatus),
        UpdateUser(UpdateUser),
        UpdateBasicGroup(UpdateBasicGroup),
        UpdateSupergroup(UpdateSupergroup),
        UpdateSecretChat(UpdateSecretChat),
        UpdateUserFullInfo(UpdateUserFullInfo),
        UpdateBasicGroupFullInfo(UpdateBasicGroupFullInfo),
        UpdateSupergroupFullInfo(UpdateSupergroupFullInfo),
        UpdateServiceNotification(UpdateServiceNotification),
        UpdateFile(UpdateFile),
        UpdateFileGenerationStart(UpdateFileGenerationStart),
        UpdateFileGenerationStop(UpdateFileGenerationStop),
        UpdateCall(UpdateCall),
        UpdateUserPrivacySettingRules(UpdateUserPrivacySettingRules),
        UpdateUnreadMessageCount(UpdateUnreadMessageCount),
        UpdateUnreadChatCount(UpdateUnreadChatCount),
        UpdateOption(UpdateOption),
        UpdateInstalledStickerSets(UpdateInstalledStickerSets),
        UpdateTrendingStickerSets(UpdateTrendingStickerSets),
        UpdateRecentStickers(UpdateRecentStickers),
        UpdateFavoriteStickers(UpdateFavoriteStickers),
        UpdateSavedAnimations(UpdateSavedAnimations),
        UpdateSelectedBackground(UpdateSelectedBackground),
        UpdateLanguagePackStrings(UpdateLanguagePackStrings),
        UpdateConnectionState(UpdateConnectionState),
        UpdateTermsOfService(UpdateTermsOfService),
        UpdateUsersNearby(UpdateUsersNearby),
        UpdateNewInlineQuery(UpdateNewInlineQuery),
        UpdateNewChosenInlineResult(UpdateNewChosenInlineResult),
        UpdateNewCallbackQuery(UpdateNewCallbackQuery),
        UpdateNewInlineCallbackQuery(UpdateNewInlineCallbackQuery),
        UpdateNewShippingQuery(UpdateNewShippingQuery),
        UpdateNewPreCheckoutQuery(UpdateNewPreCheckoutQuery),
        UpdateNewCustomEvent(UpdateNewCustomEvent),
        UpdateNewCustomQuery(UpdateNewCustomQuery),
        UpdatePoll(UpdatePoll),
        UpdatePollAnswer(UpdatePollAnswer),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of updates"]
    pub struct Updates {
        #[doc = "List of updates"]
        pub updates: Vec<Update>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The log is written to stderr or an OS specific log"]
    pub struct LogStreamDefault {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The log is written to a file"]
    pub struct LogStreamFile {
        #[doc = "Path to the file to where the internal TDLib log will be written"]
        pub path: String,
        #[doc = "The maximum size of the file to where the internal TDLib log is written before the file will be auto-rotated"]
        pub max_file_size: i64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "The log is written nowhere"]
    pub struct LogStreamEmpty {}
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    #[doc = "Describes a stream to which TDLib internal log is written"]
    pub enum LogStream {
        LogStreamDefault(LogStreamDefault),
        LogStreamFile(LogStreamFile),
        LogStreamEmpty(LogStreamEmpty),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a TDLib internal log verbosity level"]
    pub struct LogVerbosityLevel {
        #[doc = "Log verbosity level"]
        pub verbosity_level: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "Contains a list of available TDLib internal log tags"]
    pub struct LogTags {
        #[doc = "List of log tags"]
        pub tags: Vec<String>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A simple object containing a number; for testing only"]
    pub struct TestInt {
        #[doc = "Number"]
        pub value: i32,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A simple object containing a string; for testing only"]
    pub struct TestString {
        #[doc = "String"]
        pub value: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A simple object containing a sequence of bytes; for testing only"]
    pub struct TestBytes {
        #[doc = "Bytes"]
        pub value: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A simple object containing a vector of numbers; for testing only"]
    pub struct TestVectorInt {
        #[doc = "Vector of numbers"]
        pub value: Vec<i32>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A simple object containing a vector of objects that hold a number; for testing only"]
    pub struct TestVectorIntObject {
        #[doc = "Vector of objects"]
        pub value: Vec<TestInt>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A simple object containing a vector of strings; for testing only"]
    pub struct TestVectorString {
        #[doc = "Vector of strings"]
        pub value: Vec<String>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[doc = "A simple object containing a vector of objects that hold a string; for testing only"]
    pub struct TestVectorStringObject {
        #[doc = "Vector of objects"]
        pub value: Vec<TestString>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "@type")]
    pub enum Response {
        Error(Error),
        Ok(Ok),
        TdlibParameters(TdlibParameters),
        AuthenticationCodeType(AuthenticationCodeType),
        AuthenticationCodeInfo(AuthenticationCodeInfo),
        EmailAddressAuthenticationCodeInfo(EmailAddressAuthenticationCodeInfo),
        TextEntity(TextEntity),
        TextEntities(TextEntities),
        FormattedText(FormattedText),
        TermsOfService(TermsOfService),
        AuthorizationState(AuthorizationState),
        PasswordState(PasswordState),
        RecoveryEmailAddress(RecoveryEmailAddress),
        TemporaryPasswordState(TemporaryPasswordState),
        LocalFile(LocalFile),
        RemoteFile(RemoteFile),
        File(File),
        InputFile(InputFile),
        PhotoSize(PhotoSize),
        Minithumbnail(Minithumbnail),
        MaskPoint(MaskPoint),
        MaskPosition(MaskPosition),
        PollOption(PollOption),
        PollType(PollType),
        Animation(Animation),
        Audio(Audio),
        Document(Document),
        Photo(Photo),
        Sticker(Sticker),
        Video(Video),
        VideoNote(VideoNote),
        VoiceNote(VoiceNote),
        Contact(Contact),
        Location(Location),
        Venue(Venue),
        Game(Game),
        Poll(Poll),
        ProfilePhoto(ProfilePhoto),
        ChatPhoto(ChatPhoto),
        UserType(UserType),
        BotCommand(BotCommand),
        BotInfo(BotInfo),
        ChatLocation(ChatLocation),
        User(User),
        UserFullInfo(UserFullInfo),
        UserProfilePhoto(UserProfilePhoto),
        UserProfilePhotos(UserProfilePhotos),
        Users(Users),
        ChatAdministrator(ChatAdministrator),
        ChatAdministrators(ChatAdministrators),
        ChatPermissions(ChatPermissions),
        ChatMemberStatus(ChatMemberStatus),
        ChatMember(ChatMember),
        ChatMembers(ChatMembers),
        ChatMembersFilter(ChatMembersFilter),
        SupergroupMembersFilter(SupergroupMembersFilter),
        BasicGroup(BasicGroup),
        BasicGroupFullInfo(BasicGroupFullInfo),
        Supergroup(Supergroup),
        SupergroupFullInfo(SupergroupFullInfo),
        SecretChatState(SecretChatState),
        SecretChat(SecretChat),
        MessageForwardOrigin(MessageForwardOrigin),
        MessageForwardInfo(MessageForwardInfo),
        MessageSendingState(MessageSendingState),
        Message(Message),
        Messages(Messages),
        FoundMessages(FoundMessages),
        NotificationSettingsScope(NotificationSettingsScope),
        ChatNotificationSettings(ChatNotificationSettings),
        ScopeNotificationSettings(ScopeNotificationSettings),
        DraftMessage(DraftMessage),
        ChatType(ChatType),
        ChatList(ChatList),
        Chat(Chat),
        Chats(Chats),
        ChatNearby(ChatNearby),
        ChatsNearby(ChatsNearby),
        ChatInviteLink(ChatInviteLink),
        ChatInviteLinkInfo(ChatInviteLinkInfo),
        PublicChatType(PublicChatType),
        ChatActionBar(ChatActionBar),
        KeyboardButtonType(KeyboardButtonType),
        KeyboardButton(KeyboardButton),
        InlineKeyboardButtonType(InlineKeyboardButtonType),
        InlineKeyboardButton(InlineKeyboardButton),
        ReplyMarkup(ReplyMarkup),
        LoginUrlInfo(LoginUrlInfo),
        RichText(RichText),
        PageBlockCaption(PageBlockCaption),
        PageBlockListItem(PageBlockListItem),
        PageBlockHorizontalAlignment(PageBlockHorizontalAlignment),
        PageBlockVerticalAlignment(PageBlockVerticalAlignment),
        PageBlockTableCell(PageBlockTableCell),
        PageBlockRelatedArticle(PageBlockRelatedArticle),
        PageBlock(PageBlock),
        WebPageInstantView(WebPageInstantView),
        WebPage(WebPage),
        Address(Address),
        LabeledPricePart(LabeledPricePart),
        Invoice(Invoice),
        OrderInfo(OrderInfo),
        ShippingOption(ShippingOption),
        SavedCredentials(SavedCredentials),
        InputCredentials(InputCredentials),
        PaymentsProviderStripe(PaymentsProviderStripe),
        PaymentForm(PaymentForm),
        ValidatedOrderInfo(ValidatedOrderInfo),
        PaymentResult(PaymentResult),
        PaymentReceipt(PaymentReceipt),
        DatedFile(DatedFile),
        PassportElementType(PassportElementType),
        Date(Date),
        PersonalDetails(PersonalDetails),
        IdentityDocument(IdentityDocument),
        InputIdentityDocument(InputIdentityDocument),
        PersonalDocument(PersonalDocument),
        InputPersonalDocument(InputPersonalDocument),
        PassportElement(PassportElement),
        InputPassportElement(InputPassportElement),
        PassportElements(PassportElements),
        PassportElementErrorSource(PassportElementErrorSource),
        PassportElementError(PassportElementError),
        PassportSuitableElement(PassportSuitableElement),
        PassportRequiredElement(PassportRequiredElement),
        PassportAuthorizationForm(PassportAuthorizationForm),
        PassportElementsWithErrors(PassportElementsWithErrors),
        EncryptedCredentials(EncryptedCredentials),
        EncryptedPassportElement(EncryptedPassportElement),
        InputPassportElementErrorSource(InputPassportElementErrorSource),
        InputPassportElementError(InputPassportElementError),
        MessageContent(MessageContent),
        TextEntityType(TextEntityType),
        InputThumbnail(InputThumbnail),
        MessageSchedulingState(MessageSchedulingState),
        SendMessageOptions(SendMessageOptions),
        InputMessageContent(InputMessageContent),
        SearchMessagesFilter(SearchMessagesFilter),
        ChatAction(ChatAction),
        UserStatus(UserStatus),
        Stickers(Stickers),
        Emojis(Emojis),
        StickerSet(StickerSet),
        StickerSetInfo(StickerSetInfo),
        StickerSets(StickerSets),
        CallDiscardReason(CallDiscardReason),
        CallProtocol(CallProtocol),
        CallConnection(CallConnection),
        CallId(CallId),
        CallState(CallState),
        CallProblem(CallProblem),
        Call(Call),
        PhoneNumberAuthenticationSettings(PhoneNumberAuthenticationSettings),
        Animations(Animations),
        ImportedContacts(ImportedContacts),
        HttpUrl(HttpUrl),
        InputInlineQueryResult(InputInlineQueryResult),
        InlineQueryResult(InlineQueryResult),
        InlineQueryResults(InlineQueryResults),
        CallbackQueryPayload(CallbackQueryPayload),
        CallbackQueryAnswer(CallbackQueryAnswer),
        CustomRequestResult(CustomRequestResult),
        GameHighScore(GameHighScore),
        GameHighScores(GameHighScores),
        ChatEventAction(ChatEventAction),
        ChatEvent(ChatEvent),
        ChatEvents(ChatEvents),
        ChatEventLogFilters(ChatEventLogFilters),
        LanguagePackStringValue(LanguagePackStringValue),
        LanguagePackString(LanguagePackString),
        LanguagePackStrings(LanguagePackStrings),
        LanguagePackInfo(LanguagePackInfo),
        LocalizationTargetInfo(LocalizationTargetInfo),
        DeviceToken(DeviceToken),
        PushReceiverId(PushReceiverId),
        BackgroundFill(BackgroundFill),
        BackgroundType(BackgroundType),
        Background(Background),
        Backgrounds(Backgrounds),
        InputBackground(InputBackground),
        Hashtags(Hashtags),
        CanTransferOwnershipResult(CanTransferOwnershipResult),
        CheckChatUsernameResult(CheckChatUsernameResult),
        PushMessageContent(PushMessageContent),
        NotificationType(NotificationType),
        NotificationGroupType(NotificationGroupType),
        Notification(Notification),
        NotificationGroup(NotificationGroup),
        OptionValue(OptionValue),
        JsonObjectMember(JsonObjectMember),
        JsonValue(JsonValue),
        UserPrivacySettingRule(UserPrivacySettingRule),
        UserPrivacySettingRules(UserPrivacySettingRules),
        UserPrivacySetting(UserPrivacySetting),
        AccountTtl(AccountTtl),
        Session(Session),
        Sessions(Sessions),
        ConnectedWebsite(ConnectedWebsite),
        ConnectedWebsites(ConnectedWebsites),
        ChatReportReason(ChatReportReason),
        PublicMessageLink(PublicMessageLink),
        MessageLinkInfo(MessageLinkInfo),
        FilePart(FilePart),
        FileType(FileType),
        StorageStatisticsByFileType(StorageStatisticsByFileType),
        StorageStatisticsByChat(StorageStatisticsByChat),
        StorageStatistics(StorageStatistics),
        StorageStatisticsFast(StorageStatisticsFast),
        DatabaseStatistics(DatabaseStatistics),
        NetworkType(NetworkType),
        NetworkStatisticsEntry(NetworkStatisticsEntry),
        NetworkStatistics(NetworkStatistics),
        AutoDownloadSettings(AutoDownloadSettings),
        AutoDownloadSettingsPresets(AutoDownloadSettingsPresets),
        ConnectionState(ConnectionState),
        TopChatCategory(TopChatCategory),
        TMeUrlType(TMeUrlType),
        TMeUrl(TMeUrl),
        TMeUrls(TMeUrls),
        Count(Count),
        Text(Text),
        Seconds(Seconds),
        DeepLinkInfo(DeepLinkInfo),
        TextParseMode(TextParseMode),
        ProxyType(ProxyType),
        Proxy(Proxy),
        Proxies(Proxies),
        InputSticker(InputSticker),
        UpdateAuthorizationState(UpdateAuthorizationState),
        UpdateNewMessage(UpdateNewMessage),
        UpdateMessageSendAcknowledged(UpdateMessageSendAcknowledged),
        UpdateMessageSendSucceeded(UpdateMessageSendSucceeded),
        UpdateMessageSendFailed(UpdateMessageSendFailed),
        UpdateMessageContent(UpdateMessageContent),
        UpdateMessageEdited(UpdateMessageEdited),
        UpdateMessageViews(UpdateMessageViews),
        UpdateMessageContentOpened(UpdateMessageContentOpened),
        UpdateMessageMentionRead(UpdateMessageMentionRead),
        UpdateMessageLiveLocationViewed(UpdateMessageLiveLocationViewed),
        UpdateNewChat(UpdateNewChat),
        UpdateChatChatList(UpdateChatChatList),
        UpdateChatTitle(UpdateChatTitle),
        UpdateChatPhoto(UpdateChatPhoto),
        UpdateChatPermissions(UpdateChatPermissions),
        UpdateChatLastMessage(UpdateChatLastMessage),
        UpdateChatOrder(UpdateChatOrder),
        UpdateChatIsPinned(UpdateChatIsPinned),
        UpdateChatIsMarkedAsUnread(UpdateChatIsMarkedAsUnread),
        UpdateChatIsSponsored(UpdateChatIsSponsored),
        UpdateChatHasScheduledMessages(UpdateChatHasScheduledMessages),
        UpdateChatDefaultDisableNotification(UpdateChatDefaultDisableNotification),
        UpdateChatReadInbox(UpdateChatReadInbox),
        UpdateChatReadOutbox(UpdateChatReadOutbox),
        UpdateChatUnreadMentionCount(UpdateChatUnreadMentionCount),
        UpdateChatNotificationSettings(UpdateChatNotificationSettings),
        UpdateScopeNotificationSettings(UpdateScopeNotificationSettings),
        UpdateChatActionBar(UpdateChatActionBar),
        UpdateChatPinnedMessage(UpdateChatPinnedMessage),
        UpdateChatReplyMarkup(UpdateChatReplyMarkup),
        UpdateChatDraftMessage(UpdateChatDraftMessage),
        UpdateChatOnlineMemberCount(UpdateChatOnlineMemberCount),
        UpdateNotification(UpdateNotification),
        UpdateNotificationGroup(UpdateNotificationGroup),
        UpdateActiveNotifications(UpdateActiveNotifications),
        UpdateHavePendingNotifications(UpdateHavePendingNotifications),
        UpdateDeleteMessages(UpdateDeleteMessages),
        UpdateUserChatAction(UpdateUserChatAction),
        UpdateUserStatus(UpdateUserStatus),
        UpdateUser(UpdateUser),
        UpdateBasicGroup(UpdateBasicGroup),
        UpdateSupergroup(UpdateSupergroup),
        UpdateSecretChat(UpdateSecretChat),
        UpdateUserFullInfo(UpdateUserFullInfo),
        UpdateBasicGroupFullInfo(UpdateBasicGroupFullInfo),
        UpdateSupergroupFullInfo(UpdateSupergroupFullInfo),
        UpdateServiceNotification(UpdateServiceNotification),
        UpdateFile(UpdateFile),
        UpdateFileGenerationStart(UpdateFileGenerationStart),
        UpdateFileGenerationStop(UpdateFileGenerationStop),
        UpdateCall(UpdateCall),
        UpdateUserPrivacySettingRules(UpdateUserPrivacySettingRules),
        UpdateUnreadMessageCount(UpdateUnreadMessageCount),
        UpdateUnreadChatCount(UpdateUnreadChatCount),
        UpdateOption(UpdateOption),
        UpdateInstalledStickerSets(UpdateInstalledStickerSets),
        UpdateTrendingStickerSets(UpdateTrendingStickerSets),
        UpdateRecentStickers(UpdateRecentStickers),
        UpdateFavoriteStickers(UpdateFavoriteStickers),
        UpdateSavedAnimations(UpdateSavedAnimations),
        UpdateSelectedBackground(UpdateSelectedBackground),
        UpdateLanguagePackStrings(UpdateLanguagePackStrings),
        UpdateConnectionState(UpdateConnectionState),
        UpdateTermsOfService(UpdateTermsOfService),
        UpdateUsersNearby(UpdateUsersNearby),
        UpdateNewInlineQuery(UpdateNewInlineQuery),
        UpdateNewChosenInlineResult(UpdateNewChosenInlineResult),
        UpdateNewCallbackQuery(UpdateNewCallbackQuery),
        UpdateNewInlineCallbackQuery(UpdateNewInlineCallbackQuery),
        UpdateNewShippingQuery(UpdateNewShippingQuery),
        UpdateNewPreCheckoutQuery(UpdateNewPreCheckoutQuery),
        UpdateNewCustomEvent(UpdateNewCustomEvent),
        UpdateNewCustomQuery(UpdateNewCustomQuery),
        UpdatePoll(UpdatePoll),
        UpdatePollAnswer(UpdatePollAnswer),
        Updates(Updates),
        LogStream(LogStream),
        LogVerbosityLevel(LogVerbosityLevel),
        LogTags(LogTags),
        TestInt(TestInt),
        TestString(TestString),
        TestBytes(TestBytes),
        TestVectorInt(TestVectorInt),
        TestVectorIntObject(TestVectorIntObject),
        TestVectorString(TestVectorString),
        TestVectorStringObject(TestVectorStringObject),
    }
}
pub mod methods {
    use super::types::*;
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;
    pub trait Method: Serialize + Clone {
        const TYPE: &'static str;
        type Response: DeserializeOwned + Debug;
        fn tag(self) -> MethodType<Self>
        where
            Self: ::std::marker::Sized,
        {
            MethodType {
                type_: Self::TYPE,
                payload: self,
            }
        }
    }
    #[derive(Serialize, Debug, Clone)]
    pub struct MethodType<T: Method> {
        #[serde(rename = "@type")]
        pub type_: &'static str,
        #[serde(flatten)]
        pub payload: T,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the current authorization state; this is an offline request. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state"]
    pub struct GetAuthorizationState {}
    impl Method for GetAuthorizationState {
        const TYPE: &'static str = "getAuthorizationState";
        type Response = AuthorizationState;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sets the parameters for TDLib initialization. Works only when the current authorization state is authorizationStateWaitTdlibParameters"]
    pub struct SetTdlibParameters {
        #[doc = "Parameters"]
        pub parameters: TdlibParameters,
    }
    impl Method for SetTdlibParameters {
        const TYPE: &'static str = "setTdlibParameters";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks the database encryption key for correctness. Works only when the current authorization state is authorizationStateWaitEncryptionKey"]
    pub struct CheckDatabaseEncryptionKey {
        #[doc = "Encryption key to check or set up"]
        pub encryption_key: String,
    }
    impl Method for CheckDatabaseEncryptionKey {
        const TYPE: &'static str = "checkDatabaseEncryptionKey";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sets the phone number of the user and sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitPhoneNumber, or if there is no pending authentication query and the current authorization state is authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword"]
    pub struct SetAuthenticationPhoneNumber {
        #[doc = "The phone number of the user, in international format"]
        pub phone_number: String,
        #[doc = "Settings for the authentication of the user's phone number"]
        pub settings: PhoneNumberAuthenticationSettings,
    }
    impl Method for SetAuthenticationPhoneNumber {
        const TYPE: &'static str = "setAuthenticationPhoneNumber";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Re-sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitCode and the next_code_type of the result is not null"]
    pub struct ResendAuthenticationCode {}
    impl Method for ResendAuthenticationCode {
        const TYPE: &'static str = "resendAuthenticationCode";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks the authentication code. Works only when the current authorization state is authorizationStateWaitCode"]
    pub struct CheckAuthenticationCode {
        #[doc = "The verification code received via SMS, Telegram message, phone call, or flash call"]
        pub code: String,
    }
    impl Method for CheckAuthenticationCode {
        const TYPE: &'static str = "checkAuthenticationCode";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Requests QR code authentication by scanning a QR code on another logged in device. Works only when the current authorization state is authorizationStateWaitPhoneNumber"]
    pub struct RequestQrCodeAuthentication {
        #[doc = "List of user identifiers of other users currently using the client"]
        pub other_user_ids: Vec<i32>,
    }
    impl Method for RequestQrCodeAuthentication {
        const TYPE: &'static str = "requestQrCodeAuthentication";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Finishes user registration. Works only when the current authorization state is authorizationStateWaitRegistration"]
    pub struct RegisterUser {
        #[doc = "The first name of the user; 1-64 characters"]
        pub first_name: String,
        #[doc = "The last name of the user; 0-64 characters"]
        pub last_name: String,
    }
    impl Method for RegisterUser {
        const TYPE: &'static str = "registerUser";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks the authentication password for correctness. Works only when the current authorization state is authorizationStateWaitPassword"]
    pub struct CheckAuthenticationPassword {
        #[doc = "The password to check"]
        pub password: String,
    }
    impl Method for CheckAuthenticationPassword {
        const TYPE: &'static str = "checkAuthenticationPassword";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Requests to send a password recovery code to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword"]
    pub struct RequestAuthenticationPasswordRecovery {}
    impl Method for RequestAuthenticationPasswordRecovery {
        const TYPE: &'static str = "requestAuthenticationPasswordRecovery";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Recovers the password with a password recovery code sent to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword"]
    pub struct RecoverAuthenticationPassword {
        #[doc = "Recovery code to check"]
        pub recovery_code: String,
    }
    impl Method for RecoverAuthenticationPassword {
        const TYPE: &'static str = "recoverAuthenticationPassword";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks the authentication token of a bot; to log in as a bot. Works only when the current authorization state is authorizationStateWaitPhoneNumber. Can be used instead of setAuthenticationPhoneNumber and checkAuthenticationCode to log in"]
    pub struct CheckAuthenticationBotToken {
        #[doc = "The bot token"]
        pub token: String,
    }
    impl Method for CheckAuthenticationBotToken {
        const TYPE: &'static str = "checkAuthenticationBotToken";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Closes the TDLib instance after a proper logout. Requires an available network connection. All local data will be destroyed. After the logout completes, updateAuthorizationState with authorizationStateClosed will be sent"]
    pub struct LogOut {}
    impl Method for LogOut {
        const TYPE: &'static str = "logOut";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Closes the TDLib instance. All databases will be flushed to disk and properly closed. After the close completes, updateAuthorizationState with authorizationStateClosed will be sent"]
    pub struct Close {}
    impl Method for Close {
        const TYPE: &'static str = "close";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Closes the TDLib instance, destroying all local data without a proper logout. The current user session will remain in the list of all active sessions. All local data will be destroyed. After the destruction completes updateAuthorizationState with authorizationStateClosed will be sent"]
    pub struct Destroy {}
    impl Method for Destroy {
        const TYPE: &'static str = "destroy";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Confirms QR code authentication on another device. Returns created session on success"]
    pub struct ConfirmQrCodeAuthentication {
        #[doc = "A link from a QR code. The link must be scanned by the in-app camera"]
        pub link: String,
    }
    impl Method for ConfirmQrCodeAuthentication {
        const TYPE: &'static str = "confirmQrCodeAuthentication";
        type Response = Session;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns all updates needed to restore current TDLib state, i.e. all actual UpdateAuthorizationState/UpdateUser/UpdateNewChat and others. This is especially useful if TDLib is run in a separate process. This is an offline method. Can be called before authorization"]
    pub struct GetCurrentState {}
    impl Method for GetCurrentState {
        const TYPE: &'static str = "getCurrentState";
        type Response = Updates;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the database encryption key. Usually the encryption key is never changed and is stored in some OS keychain"]
    pub struct SetDatabaseEncryptionKey {
        #[doc = "New encryption key"]
        pub new_encryption_key: String,
    }
    impl Method for SetDatabaseEncryptionKey {
        const TYPE: &'static str = "setDatabaseEncryptionKey";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the current state of 2-step verification"]
    pub struct GetPasswordState {}
    impl Method for GetPasswordState {
        const TYPE: &'static str = "getPasswordState";
        type Response = PasswordState;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the password for the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed"]
    pub struct SetPassword {
        #[doc = "Previous password of the user"]
        pub old_password: String,
        #[doc = "New password of the user; may be empty to remove the password"]
        pub new_password: String,
        #[doc = "New password hint; may be empty"]
        pub new_hint: String,
        #[doc = "Pass true if the recovery email address should be changed"]
        pub set_recovery_email_address: bool,
        #[doc = "New recovery email address; may be empty"]
        pub new_recovery_email_address: String,
    }
    impl Method for SetPassword {
        const TYPE: &'static str = "setPassword";
        type Response = PasswordState;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a 2-step verification recovery email address that was previously set up. This method can be used to verify a password provided by the user"]
    pub struct GetRecoveryEmailAddress {
        #[doc = "The password for the current user"]
        pub password: String,
    }
    impl Method for GetRecoveryEmailAddress {
        const TYPE: &'static str = "getRecoveryEmailAddress";
        type Response = RecoveryEmailAddress;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the 2-step verification recovery email address of the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed. If new_recovery_email_address is the same as the email address that is currently set up, this call succeeds immediately and aborts all other requests waiting for an email confirmation @password Password of the current user @new_recovery_email_address New recovery email address"]
    pub struct SetRecoveryEmailAddress {
        pub password: String,
        pub new_recovery_email_address: String,
    }
    impl Method for SetRecoveryEmailAddress {
        const TYPE: &'static str = "setRecoveryEmailAddress";
        type Response = PasswordState;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks the 2-step verification recovery email address verification code"]
    pub struct CheckRecoveryEmailAddressCode {
        #[doc = "Verification code"]
        pub code: String,
    }
    impl Method for CheckRecoveryEmailAddressCode {
        const TYPE: &'static str = "checkRecoveryEmailAddressCode";
        type Response = PasswordState;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Resends the 2-step verification recovery email address verification code"]
    pub struct ResendRecoveryEmailAddressCode {}
    impl Method for ResendRecoveryEmailAddressCode {
        const TYPE: &'static str = "resendRecoveryEmailAddressCode";
        type Response = PasswordState;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Requests to send a password recovery code to an email address that was previously set up"]
    pub struct RequestPasswordRecovery {}
    impl Method for RequestPasswordRecovery {
        const TYPE: &'static str = "requestPasswordRecovery";
        type Response = EmailAddressAuthenticationCodeInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Recovers the password using a recovery code sent to an email address that was previously set up"]
    pub struct RecoverPassword {
        #[doc = "Recovery code to check"]
        pub recovery_code: String,
    }
    impl Method for RecoverPassword {
        const TYPE: &'static str = "recoverPassword";
        type Response = PasswordState;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Creates a new temporary password for processing payments"]
    pub struct CreateTemporaryPassword {
        #[doc = "Persistent user password"]
        pub password: String,
        #[doc = "Time during which the temporary password will be valid, in seconds; should be between 60 and 86400"]
        pub valid_for: i32,
    }
    impl Method for CreateTemporaryPassword {
        const TYPE: &'static str = "createTemporaryPassword";
        type Response = TemporaryPasswordState;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about the current temporary password"]
    pub struct GetTemporaryPasswordState {}
    impl Method for GetTemporaryPasswordState {
        const TYPE: &'static str = "getTemporaryPasswordState";
        type Response = TemporaryPasswordState;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the current user"]
    pub struct GetMe {}
    impl Method for GetMe {
        const TYPE: &'static str = "getMe";
        type Response = User;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a user by their identifier. This is an offline request if the current user is not a bot"]
    pub struct GetUser {
        #[doc = "User identifier"]
        pub user_id: i32,
    }
    impl Method for GetUser {
        const TYPE: &'static str = "getUser";
        type Response = User;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns full information about a user by their identifier"]
    pub struct GetUserFullInfo {
        #[doc = "User identifier"]
        pub user_id: i32,
    }
    impl Method for GetUserFullInfo {
        const TYPE: &'static str = "getUserFullInfo";
        type Response = UserFullInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a basic group by its identifier. This is an offline request if the current user is not a bot"]
    pub struct GetBasicGroup {
        #[doc = "Basic group identifier"]
        pub basic_group_id: i32,
    }
    impl Method for GetBasicGroup {
        const TYPE: &'static str = "getBasicGroup";
        type Response = BasicGroup;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns full information about a basic group by its identifier"]
    pub struct GetBasicGroupFullInfo {
        #[doc = "Basic group identifier"]
        pub basic_group_id: i32,
    }
    impl Method for GetBasicGroupFullInfo {
        const TYPE: &'static str = "getBasicGroupFullInfo";
        type Response = BasicGroupFullInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a supergroup or a channel by its identifier. This is an offline request if the current user is not a bot"]
    pub struct GetSupergroup {
        #[doc = "Supergroup or channel identifier"]
        pub supergroup_id: i32,
    }
    impl Method for GetSupergroup {
        const TYPE: &'static str = "getSupergroup";
        type Response = Supergroup;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns full information about a supergroup or a channel by its identifier, cached for up to 1 minute"]
    pub struct GetSupergroupFullInfo {
        #[doc = "Supergroup or channel identifier"]
        pub supergroup_id: i32,
    }
    impl Method for GetSupergroupFullInfo {
        const TYPE: &'static str = "getSupergroupFullInfo";
        type Response = SupergroupFullInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a secret chat by its identifier. This is an offline request"]
    pub struct GetSecretChat {
        #[doc = "Secret chat identifier"]
        pub secret_chat_id: i32,
    }
    impl Method for GetSecretChat {
        const TYPE: &'static str = "getSecretChat";
        type Response = SecretChat;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a chat by its identifier, this is an offline request if the current user is not a bot"]
    pub struct GetChat {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
    }
    impl Method for GetChat {
        const TYPE: &'static str = "getChat";
        type Response = Chat;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a message"]
    pub struct GetMessage {
        #[doc = "Identifier of the chat the message belongs to"]
        pub chat_id: i64,
        #[doc = "Identifier of the message to get"]
        pub message_id: i64,
    }
    impl Method for GetMessage {
        const TYPE: &'static str = "getMessage";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a message, if it is available locally without sending network request. This is an offline request"]
    pub struct GetMessageLocally {
        #[doc = "Identifier of the chat the message belongs to"]
        pub chat_id: i64,
        #[doc = "Identifier of the message to get"]
        pub message_id: i64,
    }
    impl Method for GetMessageLocally {
        const TYPE: &'static str = "getMessageLocally";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a message that is replied by given message"]
    pub struct GetRepliedMessage {
        #[doc = "Identifier of the chat the message belongs to"]
        pub chat_id: i64,
        #[doc = "Identifier of the message reply to which get"]
        pub message_id: i64,
    }
    impl Method for GetRepliedMessage {
        const TYPE: &'static str = "getRepliedMessage";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a pinned chat message"]
    pub struct GetChatPinnedMessage {
        #[doc = "Identifier of the chat the message belongs to"]
        pub chat_id: i64,
    }
    impl Method for GetChatPinnedMessage {
        const TYPE: &'static str = "getChatPinnedMessage";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about messages. If a message is not found, returns null on the corresponding position of the result"]
    pub struct GetMessages {
        #[doc = "Identifier of the chat the messages belong to"]
        pub chat_id: i64,
        #[doc = "Identifiers of the messages to get"]
        pub message_ids: Vec<i64>,
    }
    impl Method for GetMessages {
        const TYPE: &'static str = "getMessages";
        type Response = Messages;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a file; this is an offline request"]
    pub struct GetFile {
        #[doc = "Identifier of the file to get"]
        pub file_id: i32,
    }
    impl Method for GetFile {
        const TYPE: &'static str = "getFile";
        type Response = File;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a file by its remote ID; this is an offline request. Can be used to register a URL as a file for further uploading, or sending as a message. Even the request succeeds, the file can be used only if it is still accessible to the user. For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the client"]
    pub struct GetRemoteFile {
        #[doc = "Remote identifier of the file to get"]
        pub remote_file_id: String,
        #[doc = "File type, if known"]
        pub file_type: FileType,
    }
    impl Method for GetRemoteFile {
        const TYPE: &'static str = "getRemoteFile";
        type Response = File;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns an ordered list of chats in a chat list. Chats are sorted by the pair (order, chat_id) in decreasing order. (For example, to get a list of chats from the beginning, the offset_order should be equal to a biggest signed 64-bit number 9223372036854775807 == 2^63 - 1). For optimal performance the number of returned chats is chosen by the library"]
    pub struct GetChats {
        #[doc = "The chat list in which to return chats"]
        pub chat_list: ChatList,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Chat order to return chats from"]
        pub offset_order: i64,
        #[doc = "Chat identifier to return chats from"]
        pub offset_chat_id: i64,
        #[doc = "The maximum number of chats to be returned. It is possible that fewer chats than the limit are returned even if the end of the list is not reached"]
        pub limit: i32,
    }
    impl Method for GetChats {
        const TYPE: &'static str = "getChats";
        type Response = Chats;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches a public chat by its username. Currently only private chats, supergroups and channels can be public. Returns the chat if found; otherwise an error is returned"]
    pub struct SearchPublicChat {
        #[doc = "Username to be resolved"]
        pub username: String,
    }
    impl Method for SearchPublicChat {
        const TYPE: &'static str = "searchPublicChat";
        type Response = Chat;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches public chats by looking for specified query in their username and title. Currently only private chats, supergroups and channels can be public. Returns a meaningful number of results. Returns nothing if the length of the searched username prefix is less than 5. Excludes private chats with contacts and chats from the chat list from the results"]
    pub struct SearchPublicChats {
        #[doc = "Query to search for"]
        pub query: String,
    }
    impl Method for SearchPublicChats {
        const TYPE: &'static str = "searchPublicChats";
        type Response = Chats;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for the specified query in the title and username of already known chats, this is an offline request. Returns chats in the order seen in the chat list"]
    pub struct SearchChats {
        #[doc = "Query to search for. If the query is empty, returns up to 20 recently found chats"]
        pub query: String,
        #[doc = "The maximum number of chats to be returned"]
        pub limit: i32,
    }
    impl Method for SearchChats {
        const TYPE: &'static str = "searchChats";
        type Response = Chats;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for the specified query in the title and username of already known chats via request to the server. Returns chats in the order seen in the chat list"]
    pub struct SearchChatsOnServer {
        #[doc = "Query to search for"]
        pub query: String,
        #[doc = "The maximum number of chats to be returned"]
        pub limit: i32,
    }
    impl Method for SearchChatsOnServer {
        const TYPE: &'static str = "searchChatsOnServer";
        type Response = Chats;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of users and location-based supergroups nearby. The list of users nearby will be updated for 60 seconds after the request by the updates updateUsersNearby. The request should be sent again every 25 seconds with adjusted location to not miss new chats"]
    pub struct SearchChatsNearby {
        #[doc = "Current user location"]
        pub location: Location,
    }
    impl Method for SearchChatsNearby {
        const TYPE: &'static str = "searchChatsNearby";
        type Response = ChatsNearby;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of frequently used chats. Supported only if the chat info database is enabled"]
    pub struct GetTopChats {
        #[doc = "Category of chats to be returned"]
        pub category: TopChatCategory,
        #[doc = "The maximum number of chats to be returned; up to 30"]
        pub limit: i32,
    }
    impl Method for GetTopChats {
        const TYPE: &'static str = "getTopChats";
        type Response = Chats;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes a chat from the list of frequently used chats. Supported only if the chat info database is enabled"]
    pub struct RemoveTopChat {
        #[doc = "Category of frequently used chats"]
        pub category: TopChatCategory,
        #[doc = "Chat identifier"]
        pub chat_id: i64,
    }
    impl Method for RemoveTopChat {
        const TYPE: &'static str = "removeTopChat";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds a chat to the list of recently found chats. The chat is added to the beginning of the list. If the chat is already in the list, it will be removed from the list first"]
    pub struct AddRecentlyFoundChat {
        #[doc = "Identifier of the chat to add"]
        pub chat_id: i64,
    }
    impl Method for AddRecentlyFoundChat {
        const TYPE: &'static str = "addRecentlyFoundChat";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes a chat from the list of recently found chats"]
    pub struct RemoveRecentlyFoundChat {
        #[doc = "Identifier of the chat to be removed"]
        pub chat_id: i64,
    }
    impl Method for RemoveRecentlyFoundChat {
        const TYPE: &'static str = "removeRecentlyFoundChat";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Clears the list of recently found chats"]
    pub struct ClearRecentlyFoundChats {}
    impl Method for ClearRecentlyFoundChats {
        const TYPE: &'static str = "clearRecentlyFoundChats";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks whether a username can be set for a chat"]
    pub struct CheckChatUsername {
        #[doc = "Chat identifier; should be identifier of a supergroup chat, or a channel chat, or a private chat with self, or zero if chat is being created"]
        pub chat_id: i64,
        #[doc = "Username to be checked"]
        pub username: String,
    }
    impl Method for CheckChatUsername {
        const TYPE: &'static str = "checkChatUsername";
        type Response = CheckChatUsernameResult;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of public chats of the specified type, owned by the user"]
    pub struct GetCreatedPublicChats {
        #[serde(rename = "type")]
        #[doc = "Type of the public chats to return"]
        pub type_: PublicChatType,
    }
    impl Method for GetCreatedPublicChats {
        const TYPE: &'static str = "getCreatedPublicChats";
        type Response = Chats;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks whether the maximum number of owned public chats has been reached. Returns corresponding error if the limit was reached"]
    pub struct CheckCreatedPublicChatsLimit {
        #[serde(rename = "type")]
        #[doc = "Type of the public chats, for which to check the limit"]
        pub type_: PublicChatType,
    }
    impl Method for CheckCreatedPublicChatsLimit {
        const TYPE: &'static str = "checkCreatedPublicChatsLimit";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of basic group and supergroup chats, which can be used as a discussion group for a channel. Basic group chats need to be first upgraded to supergroups before they can be set as a discussion group"]
    pub struct GetSuitableDiscussionChats {}
    impl Method for GetSuitableDiscussionChats {
        const TYPE: &'static str = "getSuitableDiscussionChats";
        type Response = Chats;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of recently inactive supergroups and channels. Can be used when user reaches limit on the number of joined supergroups and channels and receives CHANNELS_TOO_MUCH error"]
    pub struct GetInactiveSupergroupChats {}
    impl Method for GetInactiveSupergroupChats {
        const TYPE: &'static str = "getInactiveSupergroupChats";
        type Response = Chats;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of common group chats with a given user. Chats are sorted by their type and creation date"]
    pub struct GetGroupsInCommon {
        #[doc = "User identifier"]
        pub user_id: i32,
        #[doc = "Chat identifier starting from which to return chats; use 0 for the first request"]
        pub offset_chat_id: i64,
        #[doc = "The maximum number of chats to be returned; up to 100"]
        pub limit: i32,
    }
    impl Method for GetGroupsInCommon {
        const TYPE: &'static str = "getGroupsInCommon";
        type Response = Chats;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library. This is an offline request if only_local is true"]
    pub struct GetChatHistory {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Identifier of the message starting from which history must be fetched; use 0 to get results from the last message"]
        pub from_message_id: i64,
        #[doc = "Specify 0 to get results from exactly the from_message_id or a negative offset up to 99 to get additionally some newer messages"]
        pub offset: i32,
        #[doc = "The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater or equal to -offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached"]
        pub limit: i32,
        #[doc = "If true, returns only messages that are available locally without sending network requests"]
        pub only_local: bool,
    }
    impl Method for GetChatHistory {
        const TYPE: &'static str = "getChatHistory";
        type Response = Messages;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Deletes all messages in the chat. Use Chat.can_be_deleted_only_for_self and Chat.can_be_deleted_for_all_users fields to find whether and how the method can be applied to the chat"]
    pub struct DeleteChatHistory {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Pass true if the chat should be removed from the chat list"]
        pub remove_from_chat_list: bool,
        #[doc = "Pass true to try to delete chat history for all users"]
        pub revoke: bool,
    }
    impl Method for DeleteChatHistory {
        const TYPE: &'static str = "deleteChatHistory";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query (searchSecretMessages should be used instead), or without an enabled message database. For optimal performance the number of returned messages is chosen by the library"]
    pub struct SearchChatMessages {
        #[doc = "Identifier of the chat in which to search messages"]
        pub chat_id: i64,
        #[doc = "Query to search for"]
        pub query: String,
        #[doc = "If not 0, only messages sent by the specified user will be returned. Not supported in secret chats"]
        pub sender_user_id: i32,
        #[doc = "Identifier of the message starting from which history must be fetched; use 0 to get results from the last message"]
        pub from_message_id: i64,
        #[doc = "Specify 0 to get results from exactly the from_message_id or a negative offset to get the specified message and some newer messages"]
        pub offset: i32,
        #[doc = "The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater than -offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached"]
        pub limit: i32,
        #[doc = "Filter for message content in the search results"]
        pub filter: SearchMessagesFilter,
    }
    impl Method for SearchChatMessages {
        const TYPE: &'static str = "searchChatMessages";
        type Response = Messages;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for messages in all chats except secret chats. Returns the results in reverse chronological order (i.e., in order of decreasing (date, chat_id, message_id)). For optimal performance the number of returned messages is chosen by the library"]
    pub struct SearchMessages {
        #[doc = "Chat list in which to search messages; pass null to search in all chats regardless of their chat list"]
        pub chat_list: ChatList,
        #[doc = "Query to search for"]
        pub query: String,
        #[doc = "The date of the message starting from which the results should be fetched. Use 0 or any date in the future to get results from the last message"]
        pub offset_date: i32,
        #[doc = "The chat identifier of the last found message, or 0 for the first request"]
        pub offset_chat_id: i64,
        #[doc = "The message identifier of the last found message, or 0 for the first request"]
        pub offset_message_id: i64,
        #[doc = "The maximum number of messages to be returned, up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached"]
        pub limit: i32,
    }
    impl Method for SearchMessages {
        const TYPE: &'static str = "searchMessages";
        type Response = Messages;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for messages in secret chats. Returns the results in reverse chronological order. For optimal performance the number of returned messages is chosen by the library"]
    pub struct SearchSecretMessages {
        #[doc = "Identifier of the chat in which to search. Specify 0 to search in all secret chats"]
        pub chat_id: i64,
        #[doc = "Query to search for. If empty, searchChatMessages should be used instead"]
        pub query: String,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "The identifier from the result of a previous request, use 0 to get results from the last message"]
        pub from_search_id: i64,
        #[doc = "The maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached"]
        pub limit: i32,
        #[doc = "A filter for the content of messages in the search results"]
        pub filter: SearchMessagesFilter,
    }
    impl Method for SearchSecretMessages {
        const TYPE: &'static str = "searchSecretMessages";
        type Response = FoundMessages;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for call messages. Returns the results in reverse chronological order (i. e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library"]
    pub struct SearchCallMessages {
        #[doc = "Identifier of the message from which to search; use 0 to get results from the last message"]
        pub from_message_id: i64,
        #[doc = "The maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached"]
        pub limit: i32,
        #[doc = "If true, returns only messages with missed calls"]
        pub only_missed: bool,
    }
    impl Method for SearchCallMessages {
        const TYPE: &'static str = "searchCallMessages";
        type Response = Messages;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about the recent locations of chat members that were sent to the chat. Returns up to 1 location message per user"]
    pub struct SearchChatRecentLocationMessages {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "The maximum number of messages to be returned"]
        pub limit: i32,
    }
    impl Method for SearchChatRecentLocationMessages {
        const TYPE: &'static str = "searchChatRecentLocationMessages";
        type Response = Messages;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns all active live locations that should be updated by the client. The list is persistent across application restarts only if the message database is used"]
    pub struct GetActiveLiveLocationMessages {}
    impl Method for GetActiveLiveLocationMessages {
        const TYPE: &'static str = "getActiveLiveLocationMessages";
        type Response = Messages;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the last message sent in a chat no later than the specified date"]
    pub struct GetChatMessageByDate {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Point in time (Unix timestamp) relative to which to search for messages"]
        pub date: i32,
    }
    impl Method for GetChatMessageByDate {
        const TYPE: &'static str = "getChatMessageByDate";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns approximate number of messages of the specified type in the chat"]
    pub struct GetChatMessageCount {
        #[doc = "Identifier of the chat in which to count messages"]
        pub chat_id: i64,
        #[doc = "Filter for message content; searchMessagesFilterEmpty is unsupported in this function"]
        pub filter: SearchMessagesFilter,
        #[doc = "If true, returns count that is available locally without sending network requests, returning -1 if the number of messages is unknown"]
        pub return_local: bool,
    }
    impl Method for GetChatMessageCount {
        const TYPE: &'static str = "getChatMessageCount";
        type Response = Count;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns all scheduled messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id)"]
    pub struct GetChatScheduledMessages {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
    }
    impl Method for GetChatScheduledMessages {
        const TYPE: &'static str = "getChatScheduledMessages";
        type Response = Messages;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes an active notification from notification list. Needs to be called only if the notification is removed by the current user"]
    pub struct RemoveNotification {
        #[doc = "Identifier of notification group to which the notification belongs"]
        pub notification_group_id: i32,
        #[doc = "Identifier of removed notification"]
        pub notification_id: i32,
    }
    impl Method for RemoveNotification {
        const TYPE: &'static str = "removeNotification";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes a group of active notifications. Needs to be called only if the notification group is removed by the current user"]
    pub struct RemoveNotificationGroup {
        #[doc = "Notification group identifier"]
        pub notification_group_id: i32,
        #[doc = "The maximum identifier of removed notifications"]
        pub max_notification_id: i32,
    }
    impl Method for RemoveNotificationGroup {
        const TYPE: &'static str = "removeNotificationGroup";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a public HTTPS link to a message. Available only for messages in supergroups and channels with a username"]
    pub struct GetPublicMessageLink {
        #[doc = "Identifier of the chat to which the message belongs"]
        pub chat_id: i64,
        #[doc = "Identifier of the message"]
        pub message_id: i64,
        #[doc = "Pass true if a link for a whole media album should be returned"]
        pub for_album: bool,
    }
    impl Method for GetPublicMessageLink {
        const TYPE: &'static str = "getPublicMessageLink";
        type Response = PublicMessageLink;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a private HTTPS link to a message in a chat. Available only for already sent messages in supergroups and channels. The link will work only for members of the chat"]
    pub struct GetMessageLink {
        #[doc = "Identifier of the chat to which the message belongs"]
        pub chat_id: i64,
        #[doc = "Identifier of the message"]
        pub message_id: i64,
    }
    impl Method for GetMessageLink {
        const TYPE: &'static str = "getMessageLink";
        type Response = HttpUrl;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a public or private message link"]
    pub struct GetMessageLinkInfo {
        #[doc = "The message link in the format \"https://t.me/c/...\", or \"tg://privatepost?...\", or \"https://t.me/username/...\", or \"tg://resolve?...\""]
        pub url: String,
    }
    impl Method for GetMessageLinkInfo {
        const TYPE: &'static str = "getMessageLinkInfo";
        type Response = MessageLinkInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends a message. Returns the sent message"]
    pub struct SendMessage {
        #[doc = "Target chat"]
        pub chat_id: i64,
        #[doc = "Identifier of the message to reply to or 0"]
        pub reply_to_message_id: i64,
        #[doc = "Options to be used to send the message"]
        pub options: SendMessageOptions,
        #[serde(default)]
        #[doc = "Markup for replying to the message; for bots only"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "The content of the message to be sent"]
        pub input_message_content: InputMessageContent,
    }
    impl Method for SendMessage {
        const TYPE: &'static str = "sendMessage";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends messages grouped together into an album. Currently only photo and video messages can be grouped into an album. Returns sent messages"]
    pub struct SendMessageAlbum {
        #[doc = "Target chat"]
        pub chat_id: i64,
        #[doc = "Identifier of a message to reply to or 0"]
        pub reply_to_message_id: i64,
        #[doc = "Options to be used to send the messages"]
        pub options: SendMessageOptions,
        #[doc = "Contents of messages to be sent"]
        pub input_message_contents: Vec<InputMessageContent>,
    }
    impl Method for SendMessageAlbum {
        const TYPE: &'static str = "sendMessageAlbum";
        type Response = Messages;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Invites a bot to a chat (if it is not yet a member) and sends it the /start command. Bots can't be invited to a private chat other than the chat with the bot. Bots can't be invited to channels (although they can be added as admins) and secret chats. Returns the sent message"]
    pub struct SendBotStartMessage {
        #[doc = "Identifier of the bot"]
        pub bot_user_id: i32,
        #[doc = "Identifier of the target chat"]
        pub chat_id: i64,
        #[doc = "A hidden parameter sent to the bot for deep linking purposes (https://core.telegram.org/bots#deep-linking)"]
        pub parameter: String,
    }
    impl Method for SendBotStartMessage {
        const TYPE: &'static str = "sendBotStartMessage";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends the result of an inline query as a message. Returns the sent message. Always clears a chat draft message"]
    pub struct SendInlineQueryResultMessage {
        #[doc = "Target chat"]
        pub chat_id: i64,
        #[doc = "Identifier of a message to reply to or 0"]
        pub reply_to_message_id: i64,
        #[doc = "Options to be used to send the message"]
        pub options: SendMessageOptions,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the inline query"]
        pub query_id: i64,
        #[doc = "Identifier of the inline result"]
        pub result_id: String,
        #[doc = "If true, there will be no mention of a bot, via which the message is sent. Can be used only for bots GetOption(\"animation_search_bot_username\"), GetOption(\"photo_search_bot_username\") and GetOption(\"venue_search_bot_username\")"]
        pub hide_via_bot: bool,
    }
    impl Method for SendInlineQueryResultMessage {
        const TYPE: &'static str = "sendInlineQueryResultMessage";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Forwards previously sent messages. Returns the forwarded messages in the same order as the message identifiers passed in message_ids. If a message can't be forwarded, null will be returned instead of the message"]
    pub struct ForwardMessages {
        #[doc = "Identifier of the chat to which to forward messages"]
        pub chat_id: i64,
        #[doc = "Identifier of the chat from which to forward messages"]
        pub from_chat_id: i64,
        #[doc = "Identifiers of the messages to forward"]
        pub message_ids: Vec<i64>,
        #[doc = "Options to be used to send the messages"]
        pub options: SendMessageOptions,
        #[doc = "True, if the messages should be grouped into an album after forwarding. For this to work, no more than 10 messages may be forwarded, and all of them must be photo or video messages"]
        pub as_album: bool,
        #[doc = "True, if content of the messages needs to be copied without links to the original messages. Always true if the messages are forwarded to a secret chat"]
        pub send_copy: bool,
        #[doc = "True, if media captions of message copies needs to be removed. Ignored if send_copy is false"]
        pub remove_caption: bool,
    }
    impl Method for ForwardMessages {
        const TYPE: &'static str = "forwardMessages";
        type Response = Messages;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Resends messages which failed to send. Can be called only for messages for which messageSendingStateFailed.can_retry is true and after specified in messageSendingStateFailed.retry_after time passed. If a message is re-sent, the corresponding failed to send message is deleted. Returns the sent messages in the same order as the message identifiers passed in message_ids. If a message can't be re-sent, null will be returned instead of the message"]
    pub struct ResendMessages {
        #[doc = "Identifier of the chat to send messages"]
        pub chat_id: i64,
        #[doc = "Identifiers of the messages to resend. Message identifiers must be in a strictly increasing order"]
        pub message_ids: Vec<i64>,
    }
    impl Method for ResendMessages {
        const TYPE: &'static str = "resendMessages";
        type Response = Messages;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the current TTL setting (sets a new self-destruct timer) in a secret chat and sends the corresponding message"]
    pub struct SendChatSetTtlMessage {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New TTL value, in seconds"]
        pub ttl: i32,
    }
    impl Method for SendChatSetTtlMessage {
        const TYPE: &'static str = "sendChatSetTtlMessage";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends a notification about a screenshot taken in a chat. Supported only in private and secret chats"]
    pub struct SendChatScreenshotTakenNotification {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
    }
    impl Method for SendChatScreenshotTakenNotification {
        const TYPE: &'static str = "sendChatScreenshotTakenNotification";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds a local message to a chat. The message is persistent across application restarts only if the message database is used. Returns the added message"]
    pub struct AddLocalMessage {
        #[doc = "Target chat"]
        pub chat_id: i64,
        #[doc = "Identifier of the user who will be shown as the sender of the message; may be 0 for channel posts"]
        pub sender_user_id: i32,
        #[doc = "Identifier of the message to reply to or 0"]
        pub reply_to_message_id: i64,
        #[doc = "Pass true to disable notification for the message"]
        pub disable_notification: bool,
        #[doc = "The content of the message to be added"]
        pub input_message_content: InputMessageContent,
    }
    impl Method for AddLocalMessage {
        const TYPE: &'static str = "addLocalMessage";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Deletes messages"]
    pub struct DeleteMessages {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Identifiers of the messages to be deleted"]
        pub message_ids: Vec<i64>,
        #[doc = "Pass true to try to delete messages for all chat members. Always true for supergroups, channels and secret chats"]
        pub revoke: bool,
    }
    impl Method for DeleteMessages {
        const TYPE: &'static str = "deleteMessages";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Deletes all messages sent by the specified user to a chat. Supported only for supergroups; requires can_delete_messages administrator privileges"]
    pub struct DeleteChatMessagesFromUser {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "User identifier"]
        pub user_id: i32,
    }
    impl Method for DeleteChatMessagesFromUser {
        const TYPE: &'static str = "deleteChatMessagesFromUser";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits the text of a message (or a text of a game message). Returns the edited message after the edit is completed on the server side"]
    pub struct EditMessageText {
        #[doc = "The chat the message belongs to"]
        pub chat_id: i64,
        #[doc = "Identifier of the message"]
        pub message_id: i64,
        #[serde(default)]
        #[doc = "The new message reply markup; for bots only"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "New text content of the message. Should be of type InputMessageText"]
        pub input_message_content: InputMessageContent,
    }
    impl Method for EditMessageText {
        const TYPE: &'static str = "editMessageText";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits the message content of a live location. Messages can be edited for a limited period of time specified in the live location. Returns the edited message after the edit is completed on the server side"]
    pub struct EditMessageLiveLocation {
        #[doc = "The chat the message belongs to"]
        pub chat_id: i64,
        #[doc = "Identifier of the message"]
        pub message_id: i64,
        #[serde(default)]
        #[doc = "The new message reply markup; for bots only"]
        pub reply_markup: Option<ReplyMarkup>,
        #[serde(default)]
        #[doc = "New location content of the message; may be null. Pass null to stop sharing the live location"]
        pub location: Option<Location>,
    }
    impl Method for EditMessageLiveLocation {
        const TYPE: &'static str = "editMessageLiveLocation";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits the content of a message with an animation, an audio, a document, a photo or a video. The media in the message can't be replaced if the message was set to self-destruct. Media can't be replaced by self-destructing media. Media in an album can be edited only to contain a photo or a video. Returns the edited message after the edit is completed on the server side"]
    pub struct EditMessageMedia {
        #[doc = "The chat the message belongs to"]
        pub chat_id: i64,
        #[doc = "Identifier of the message"]
        pub message_id: i64,
        #[serde(default)]
        #[doc = "The new message reply markup; for bots only"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "New content of the message. Must be one of the following types: InputMessageAnimation, InputMessageAudio, InputMessageDocument, InputMessagePhoto or InputMessageVideo"]
        pub input_message_content: InputMessageContent,
    }
    impl Method for EditMessageMedia {
        const TYPE: &'static str = "editMessageMedia";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits the message content caption. Returns the edited message after the edit is completed on the server side"]
    pub struct EditMessageCaption {
        #[doc = "The chat the message belongs to"]
        pub chat_id: i64,
        #[doc = "Identifier of the message"]
        pub message_id: i64,
        #[serde(default)]
        #[doc = "The new message reply markup; for bots only"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "New message content caption; 0-GetOption(\"message_caption_length_max\") characters"]
        pub caption: FormattedText,
    }
    impl Method for EditMessageCaption {
        const TYPE: &'static str = "editMessageCaption";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits the message reply markup; for bots only. Returns the edited message after the edit is completed on the server side"]
    pub struct EditMessageReplyMarkup {
        #[doc = "The chat the message belongs to"]
        pub chat_id: i64,
        #[doc = "Identifier of the message"]
        pub message_id: i64,
        #[doc = "The new message reply markup"]
        pub reply_markup: ReplyMarkup,
    }
    impl Method for EditMessageReplyMarkup {
        const TYPE: &'static str = "editMessageReplyMarkup";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits the text of an inline text or game message sent via a bot; for bots only"]
    pub struct EditInlineMessageText {
        #[doc = "Inline message identifier"]
        pub inline_message_id: String,
        #[doc = "The new message reply markup"]
        pub reply_markup: ReplyMarkup,
        #[doc = "New text content of the message. Should be of type InputMessageText"]
        pub input_message_content: InputMessageContent,
    }
    impl Method for EditInlineMessageText {
        const TYPE: &'static str = "editInlineMessageText";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits the content of a live location in an inline message sent via a bot; for bots only"]
    pub struct EditInlineMessageLiveLocation {
        #[doc = "Inline message identifier"]
        pub inline_message_id: String,
        #[doc = "The new message reply markup"]
        pub reply_markup: ReplyMarkup,
        #[serde(default)]
        #[doc = "New location content of the message; may be null. Pass null to stop sharing the live location"]
        pub location: Option<Location>,
    }
    impl Method for EditInlineMessageLiveLocation {
        const TYPE: &'static str = "editInlineMessageLiveLocation";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits the content of a message with an animation, an audio, a document, a photo or a video in an inline message sent via a bot; for bots only"]
    pub struct EditInlineMessageMedia {
        #[doc = "Inline message identifier"]
        pub inline_message_id: String,
        #[serde(default)]
        #[doc = "The new message reply markup; for bots only"]
        pub reply_markup: Option<ReplyMarkup>,
        #[doc = "New content of the message. Must be one of the following types: InputMessageAnimation, InputMessageAudio, InputMessageDocument, InputMessagePhoto or InputMessageVideo"]
        pub input_message_content: InputMessageContent,
    }
    impl Method for EditInlineMessageMedia {
        const TYPE: &'static str = "editInlineMessageMedia";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits the caption of an inline message sent via a bot; for bots only"]
    pub struct EditInlineMessageCaption {
        #[doc = "Inline message identifier"]
        pub inline_message_id: String,
        #[doc = "The new message reply markup"]
        pub reply_markup: ReplyMarkup,
        #[doc = "New message content caption; 0-GetOption(\"message_caption_length_max\") characters"]
        pub caption: FormattedText,
    }
    impl Method for EditInlineMessageCaption {
        const TYPE: &'static str = "editInlineMessageCaption";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits the reply markup of an inline message sent via a bot; for bots only"]
    pub struct EditInlineMessageReplyMarkup {
        #[doc = "Inline message identifier"]
        pub inline_message_id: String,
        #[doc = "The new message reply markup"]
        pub reply_markup: ReplyMarkup,
    }
    impl Method for EditInlineMessageReplyMarkup {
        const TYPE: &'static str = "editInlineMessageReplyMarkup";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits the time when a scheduled message will be sent. Scheduling state of all messages in the same album or forwarded together with the message will be also changed"]
    pub struct EditMessageSchedulingState {
        #[doc = "The chat the message belongs to"]
        pub chat_id: i64,
        #[doc = "Identifier of the message"]
        pub message_id: i64,
        #[doc = "The new message scheduling state. Pass null to send the message immediately"]
        pub scheduling_state: MessageSchedulingState,
    }
    impl Method for EditMessageSchedulingState {
        const TYPE: &'static str = "editMessageSchedulingState";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns all entities (mentions, hashtags, cashtags, bot commands, URLs, and email addresses) contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct GetTextEntities {
        #[doc = "The text in which to look for entites"]
        pub text: String,
    }
    impl Method for GetTextEntities {
        const TYPE: &'static str = "getTextEntities";
        type Response = TextEntities;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Parses Bold, Italic, Underline, Strikethrough, Code, Pre, PreCode, TextUrl and MentionName entities contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct ParseTextEntities {
        #[doc = "The text which should be parsed"]
        pub text: String,
        #[doc = "Text parse mode"]
        pub parse_mode: TextParseMode,
    }
    impl Method for ParseTextEntities {
        const TYPE: &'static str = "parseTextEntities";
        type Response = FormattedText;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the MIME type of a file, guessed by its extension. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct GetFileMimeType {
        #[doc = "The name of the file or path to the file"]
        pub file_name: String,
    }
    impl Method for GetFileMimeType {
        const TYPE: &'static str = "getFileMimeType";
        type Response = Text;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the extension of a file, guessed by its MIME type. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct GetFileExtension {
        #[doc = "The MIME type of the file"]
        pub mime_type: String,
    }
    impl Method for GetFileExtension {
        const TYPE: &'static str = "getFileExtension";
        type Response = Text;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes potentially dangerous characters from the name of a file. The encoding of the file name is supposed to be UTF-8. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct CleanFileName {
        #[doc = "File name or path to the file"]
        pub file_name: String,
    }
    impl Method for CleanFileName {
        const TYPE: &'static str = "cleanFileName";
        type Response = Text;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct GetLanguagePackString {
        #[doc = "Path to the language pack database in which strings are stored"]
        pub language_pack_database_path: String,
        #[doc = "Localization target to which the language pack belongs"]
        pub localization_target: String,
        #[doc = "Language pack identifier"]
        pub language_pack_id: String,
        #[doc = "Language pack key of the string to be returned"]
        pub key: String,
    }
    impl Method for GetLanguagePackString {
        const TYPE: &'static str = "getLanguagePackString";
        type Response = LanguagePackStringValue;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Converts a JSON-serialized string to corresponding JsonValue object. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct GetJsonValue {
        #[doc = "The JSON-serialized string"]
        pub json: String,
    }
    impl Method for GetJsonValue {
        const TYPE: &'static str = "getJsonValue";
        type Response = JsonValue;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Converts a JsonValue object to corresponding JSON-serialized string. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct GetJsonString {
        #[doc = "The JsonValue object"]
        pub json_value: JsonValue,
    }
    impl Method for GetJsonString {
        const TYPE: &'static str = "getJsonString";
        type Response = Text;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the user answer to a poll. A poll in quiz mode can be answered only once"]
    pub struct SetPollAnswer {
        #[doc = "Identifier of the chat to which the poll belongs"]
        pub chat_id: i64,
        #[doc = "Identifier of the message containing the poll"]
        pub message_id: i64,
        #[doc = "0-based identifiers of answer options, chosen by the user. User can choose more than 1 answer option only is the poll allows multiple answers"]
        pub option_ids: Vec<i32>,
    }
    impl Method for SetPollAnswer {
        const TYPE: &'static str = "setPollAnswer";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns users voted for the specified option in a non-anonymous polls. For the optimal performance the number of returned users is chosen by the library"]
    pub struct GetPollVoters {
        #[doc = "Identifier of the chat to which the poll belongs"]
        pub chat_id: i64,
        #[doc = "Identifier of the message containing the poll"]
        pub message_id: i64,
        #[doc = "0-based identifier of the answer option"]
        pub option_id: i32,
        #[doc = "Number of users to skip in the result; must be non-negative"]
        pub offset: i32,
        #[doc = "The maximum number of users to be returned; must be positive and can't be greater than 50. Fewer users may be returned than specified by the limit, even if the end of the voter list has not been reached"]
        pub limit: i32,
    }
    impl Method for GetPollVoters {
        const TYPE: &'static str = "getPollVoters";
        type Response = Users;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Stops a poll. A poll in a message can be stopped when the message has can_be_edited flag set"]
    pub struct StopPoll {
        #[doc = "Identifier of the chat to which the poll belongs"]
        pub chat_id: i64,
        #[doc = "Identifier of the message containing the poll"]
        pub message_id: i64,
        #[serde(default)]
        #[doc = "The new message reply markup; for bots only"]
        pub reply_markup: Option<ReplyMarkup>,
    }
    impl Method for StopPoll {
        const TYPE: &'static str = "stopPoll";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a button of type inlineKeyboardButtonTypeLoginUrl. The method needs to be called when the user presses the button"]
    pub struct GetLoginUrlInfo {
        #[doc = "Chat identifier of the message with the button"]
        pub chat_id: i64,
        #[doc = "Message identifier of the message with the button"]
        pub message_id: i64,
        #[doc = "Button identifier"]
        pub button_id: i32,
    }
    impl Method for GetLoginUrlInfo {
        const TYPE: &'static str = "getLoginUrlInfo";
        type Response = LoginUrlInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns an HTTP URL which can be used to automatically authorize the user on a website after clicking an inline button of type inlineKeyboardButtonTypeLoginUrl. Use the method getLoginUrlInfo to find whether a prior user confirmation is needed. If an error is returned, then the button must be handled as an ordinary URL button"]
    pub struct GetLoginUrl {
        #[doc = "Chat identifier of the message with the button"]
        pub chat_id: i64,
        #[doc = "Message identifier of the message with the button"]
        pub message_id: i64,
        #[doc = "Button identifier"]
        pub button_id: i32,
        #[doc = "True, if the user allowed the bot to send them messages"]
        pub allow_write_access: bool,
    }
    impl Method for GetLoginUrl {
        const TYPE: &'static str = "getLoginUrl";
        type Response = HttpUrl;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends an inline query to a bot and returns its results. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires"]
    pub struct GetInlineQueryResults {
        #[doc = "The identifier of the target bot"]
        pub bot_user_id: i32,
        #[doc = "Identifier of the chat where the query was sent"]
        pub chat_id: i64,
        #[doc = "Location of the user, only if needed"]
        pub user_location: Location,
        #[doc = "Text of the query"]
        pub query: String,
        #[doc = "Offset of the first entry to return"]
        pub offset: String,
    }
    impl Method for GetInlineQueryResults {
        const TYPE: &'static str = "getInlineQueryResults";
        type Response = InlineQueryResults;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sets the result of an inline query; for bots only"]
    pub struct AnswerInlineQuery {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the inline query"]
        pub inline_query_id: i64,
        #[doc = "True, if the result of the query can be cached for the specified user"]
        pub is_personal: bool,
        #[doc = "The results of the query"]
        pub results: Vec<InputInlineQueryResult>,
        #[doc = "Allowed time to cache the results of the query, in seconds"]
        pub cache_time: i32,
        #[doc = "Offset for the next inline query; pass an empty string if there are no more results"]
        pub next_offset: String,
        #[doc = "If non-empty, this text should be shown on the button that opens a private chat with the bot and sends a start message to the bot with the parameter switch_pm_parameter"]
        pub switch_pm_text: String,
        #[doc = "The parameter for the bot start message"]
        pub switch_pm_parameter: String,
    }
    impl Method for AnswerInlineQuery {
        const TYPE: &'static str = "answerInlineQuery";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends a callback query to a bot and returns an answer. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires"]
    pub struct GetCallbackQueryAnswer {
        #[doc = "Identifier of the chat with the message"]
        pub chat_id: i64,
        #[doc = "Identifier of the message from which the query originated"]
        pub message_id: i64,
        #[doc = "Query payload"]
        pub payload: CallbackQueryPayload,
    }
    impl Method for GetCallbackQueryAnswer {
        const TYPE: &'static str = "getCallbackQueryAnswer";
        type Response = CallbackQueryAnswer;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sets the result of a callback query; for bots only"]
    pub struct AnswerCallbackQuery {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the callback query"]
        pub callback_query_id: i64,
        #[doc = "Text of the answer"]
        pub text: String,
        #[doc = "If true, an alert should be shown to the user instead of a toast notification"]
        pub show_alert: bool,
        #[doc = "URL to be opened"]
        pub url: String,
        #[doc = "Time during which the result of the query can be cached, in seconds"]
        pub cache_time: i32,
    }
    impl Method for AnswerCallbackQuery {
        const TYPE: &'static str = "answerCallbackQuery";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sets the result of a shipping query; for bots only"]
    pub struct AnswerShippingQuery {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the shipping query"]
        pub shipping_query_id: i64,
        #[doc = "Available shipping options"]
        pub shipping_options: Vec<ShippingOption>,
        #[doc = "An error message, empty on success"]
        pub error_message: String,
    }
    impl Method for AnswerShippingQuery {
        const TYPE: &'static str = "answerShippingQuery";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sets the result of a pre-checkout query; for bots only"]
    pub struct AnswerPreCheckoutQuery {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the pre-checkout query"]
        pub pre_checkout_query_id: i64,
        #[doc = "An error message, empty on success"]
        pub error_message: String,
    }
    impl Method for AnswerPreCheckoutQuery {
        const TYPE: &'static str = "answerPreCheckoutQuery";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Updates the game score of the specified user in the game; for bots only"]
    pub struct SetGameScore {
        #[doc = "The chat to which the message with the game belongs"]
        pub chat_id: i64,
        #[doc = "Identifier of the message"]
        pub message_id: i64,
        #[doc = "True, if the message should be edited"]
        pub edit_message: bool,
        #[doc = "User identifier"]
        pub user_id: i32,
        #[doc = "The new score"]
        pub score: i32,
        #[doc = "Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table"]
        pub force: bool,
    }
    impl Method for SetGameScore {
        const TYPE: &'static str = "setGameScore";
        type Response = Message;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Updates the game score of the specified user in a game; for bots only"]
    pub struct SetInlineGameScore {
        #[doc = "Inline message identifier"]
        pub inline_message_id: String,
        #[doc = "True, if the message should be edited"]
        pub edit_message: bool,
        #[doc = "User identifier"]
        pub user_id: i32,
        #[doc = "The new score"]
        pub score: i32,
        #[doc = "Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table"]
        pub force: bool,
    }
    impl Method for SetInlineGameScore {
        const TYPE: &'static str = "setInlineGameScore";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the high scores for a game and some part of the high score table in the range of the specified user; for bots only"]
    pub struct GetGameHighScores {
        #[doc = "The chat that contains the message with the game"]
        pub chat_id: i64,
        #[doc = "Identifier of the message"]
        pub message_id: i64,
        #[doc = "User identifier"]
        pub user_id: i32,
    }
    impl Method for GetGameHighScores {
        const TYPE: &'static str = "getGameHighScores";
        type Response = GameHighScores;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns game high scores and some part of the high score table in the range of the specified user; for bots only"]
    pub struct GetInlineGameHighScores {
        #[doc = "Inline message identifier"]
        pub inline_message_id: String,
        #[doc = "User identifier"]
        pub user_id: i32,
    }
    impl Method for GetInlineGameHighScores {
        const TYPE: &'static str = "getInlineGameHighScores";
        type Response = GameHighScores;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Deletes the default reply markup from a chat. Must be called after a one-time keyboard or a ForceReply reply markup has been used. UpdateChatReplyMarkup will be sent if the reply markup will be changed"]
    pub struct DeleteChatReplyMarkup {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "The message identifier of the used keyboard"]
        pub message_id: i64,
    }
    impl Method for DeleteChatReplyMarkup {
        const TYPE: &'static str = "deleteChatReplyMarkup";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends a notification about user activity in a chat"]
    pub struct SendChatAction {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "The action description"]
        pub action: ChatAction,
    }
    impl Method for SendChatAction {
        const TYPE: &'static str = "sendChatAction";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Informs TDLib that the chat is opened by the user. Many useful activities depend on the chat being opened or closed (e.g., in supergroups and channels all updates are received only for opened chats)"]
    pub struct OpenChat {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
    }
    impl Method for OpenChat {
        const TYPE: &'static str = "openChat";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Informs TDLib that the chat is closed by the user. Many useful activities depend on the chat being opened or closed"]
    pub struct CloseChat {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
    }
    impl Method for CloseChat {
        const TYPE: &'static str = "closeChat";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Informs TDLib that messages are being viewed by the user. Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels)"]
    pub struct ViewMessages {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "The identifiers of the messages being viewed"]
        pub message_ids: Vec<i64>,
        #[doc = "True, if messages in closed chats should be marked as read"]
        pub force_read: bool,
    }
    impl Method for ViewMessages {
        const TYPE: &'static str = "viewMessages";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Informs TDLib that the message content has been opened (e.g., the user has opened a photo, video, document, location or venue, or has listened to an audio file or voice note message). An updateMessageContentOpened update will be generated if something has changed"]
    pub struct OpenMessageContent {
        #[doc = "Chat identifier of the message"]
        pub chat_id: i64,
        #[doc = "Identifier of the message with the opened content"]
        pub message_id: i64,
    }
    impl Method for OpenMessageContent {
        const TYPE: &'static str = "openMessageContent";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Marks all mentions in a chat as read"]
    pub struct ReadAllChatMentions {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
    }
    impl Method for ReadAllChatMentions {
        const TYPE: &'static str = "readAllChatMentions";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns an existing chat corresponding to a given user"]
    pub struct CreatePrivateChat {
        #[doc = "User identifier"]
        pub user_id: i32,
        #[doc = "If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect"]
        pub force: bool,
    }
    impl Method for CreatePrivateChat {
        const TYPE: &'static str = "createPrivateChat";
        type Response = Chat;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns an existing chat corresponding to a known basic group"]
    pub struct CreateBasicGroupChat {
        #[doc = "Basic group identifier"]
        pub basic_group_id: i32,
        #[doc = "If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect"]
        pub force: bool,
    }
    impl Method for CreateBasicGroupChat {
        const TYPE: &'static str = "createBasicGroupChat";
        type Response = Chat;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns an existing chat corresponding to a known supergroup or channel"]
    pub struct CreateSupergroupChat {
        #[doc = "Supergroup or channel identifier"]
        pub supergroup_id: i32,
        #[doc = "If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect"]
        pub force: bool,
    }
    impl Method for CreateSupergroupChat {
        const TYPE: &'static str = "createSupergroupChat";
        type Response = Chat;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns an existing chat corresponding to a known secret chat"]
    pub struct CreateSecretChat {
        #[doc = "Secret chat identifier"]
        pub secret_chat_id: i32,
    }
    impl Method for CreateSecretChat {
        const TYPE: &'static str = "createSecretChat";
        type Response = Chat;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Creates a new basic group and sends a corresponding messageBasicGroupChatCreate. Returns the newly created chat"]
    pub struct CreateNewBasicGroupChat {
        #[doc = "Identifiers of users to be added to the basic group"]
        pub user_ids: Vec<i32>,
        #[doc = "Title of the new basic group; 1-128 characters"]
        pub title: String,
    }
    impl Method for CreateNewBasicGroupChat {
        const TYPE: &'static str = "createNewBasicGroupChat";
        type Response = Chat;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat"]
    pub struct CreateNewSupergroupChat {
        #[doc = "Title of the new chat; 1-128 characters"]
        pub title: String,
        #[doc = "True, if a channel chat should be created"]
        pub is_channel: bool,
        #[doc = "Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat"]
        pub description: String,
        #[doc = "Chat location if a location-based supergroup is being created"]
        pub location: ChatLocation,
    }
    impl Method for CreateNewSupergroupChat {
        const TYPE: &'static str = "createNewSupergroupChat";
        type Response = Chat;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Creates a new secret chat. Returns the newly created chat"]
    pub struct CreateNewSecretChat {
        #[doc = "Identifier of the target user"]
        pub user_id: i32,
    }
    impl Method for CreateNewSecretChat {
        const TYPE: &'static str = "createNewSecretChat";
        type Response = Chat;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Creates a new supergroup from an existing basic group and sends a corresponding messageChatUpgradeTo and messageChatUpgradeFrom; requires creator privileges. Deactivates the original basic group"]
    pub struct UpgradeBasicGroupChatToSupergroupChat {
        #[doc = "Identifier of the chat to upgrade"]
        pub chat_id: i64,
    }
    impl Method for UpgradeBasicGroupChatToSupergroupChat {
        const TYPE: &'static str = "upgradeBasicGroupChatToSupergroupChat";
        type Response = Chat;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Moves a chat to a different chat list. Current chat list of the chat must ne non-null"]
    pub struct SetChatChatList {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New chat list of the chat"]
        pub chat_list: ChatList,
    }
    impl Method for SetChatChatList {
        const TYPE: &'static str = "setChatChatList";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the chat title. Supported only for basic groups, supergroups and channels. Requires can_change_info rights. The title will not be changed until the request to the server has been completed"]
    pub struct SetChatTitle {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New title of the chat; 1-128 characters"]
        pub title: String,
    }
    impl Method for SetChatTitle {
        const TYPE: &'static str = "setChatTitle";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the photo of a chat. Supported only for basic groups, supergroups and channels. Requires can_change_info rights. The photo will not be changed before request to the server has been completed"]
    pub struct SetChatPhoto {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New chat photo. You can use a zero InputFileId to delete the chat photo. Files that are accessible only by HTTP URL are not acceptable"]
        pub photo: InputFile,
    }
    impl Method for SetChatPhoto {
        const TYPE: &'static str = "setChatPhoto";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the chat members permissions. Supported only for basic groups and supergroups. Requires can_restrict_members administrator right"]
    pub struct SetChatPermissions {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New non-administrator members permissions in the chat"]
        pub permissions: ChatPermissions,
    }
    impl Method for SetChatPermissions {
        const TYPE: &'static str = "setChatPermissions";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the draft message in a chat"]
    pub struct SetChatDraftMessage {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[serde(default)]
        #[doc = "New draft message; may be null"]
        pub draft_message: Option<DraftMessage>,
    }
    impl Method for SetChatDraftMessage {
        const TYPE: &'static str = "setChatDraftMessage";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the notification settings of a chat. Notification settings of a chat with the current user (Saved Messages) can't be changed"]
    pub struct SetChatNotificationSettings {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New notification settings for the chat. If the chat is muted for more than 1 week, it is considered to be muted forever"]
        pub notification_settings: ChatNotificationSettings,
    }
    impl Method for SetChatNotificationSettings {
        const TYPE: &'static str = "setChatNotificationSettings";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the pinned state of a chat. You can pin up to GetOption(\"pinned_chat_count_max\")/GetOption(\"pinned_archived_chat_count_max\") non-secret chats and the same number of secret chats in the main/archive chat list"]
    pub struct ToggleChatIsPinned {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New value of is_pinned"]
        pub is_pinned: bool,
    }
    impl Method for ToggleChatIsPinned {
        const TYPE: &'static str = "toggleChatIsPinned";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the marked as unread state of a chat"]
    pub struct ToggleChatIsMarkedAsUnread {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New value of is_marked_as_unread"]
        pub is_marked_as_unread: bool,
    }
    impl Method for ToggleChatIsMarkedAsUnread {
        const TYPE: &'static str = "toggleChatIsMarkedAsUnread";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the value of the default disable_notification parameter, used when a message is sent to a chat"]
    pub struct ToggleChatDefaultDisableNotification {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New value of default_disable_notification"]
        pub default_disable_notification: bool,
    }
    impl Method for ToggleChatDefaultDisableNotification {
        const TYPE: &'static str = "toggleChatDefaultDisableNotification";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes client data associated with a chat"]
    pub struct SetChatClientData {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New value of client_data"]
        pub client_data: String,
    }
    impl Method for SetChatClientData {
        const TYPE: &'static str = "setChatClientData";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes information about a chat. Available for basic groups, supergroups, and channels. Requires can_change_info rights"]
    pub struct SetChatDescription {
        #[doc = "Identifier of the chat"]
        pub chat_id: i64,
        #[doc = "Changes information about a chat. Available for basic groups, supergroups, and channels. Requires can_change_info rights"]
        pub description: String,
    }
    impl Method for SetChatDescription {
        const TYPE: &'static str = "setChatDescription";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the discussion group of a channel chat; requires can_change_info rights in the channel if it is specified"]
    pub struct SetChatDiscussionGroup {
        #[doc = "Identifier of the channel chat. Pass 0 to remove a link from the supergroup passed in the second argument to a linked channel chat (requires can_pin_messages rights in the supergroup)"]
        pub chat_id: i64,
        #[doc = "Identifier of a new channel's discussion group. Use 0 to remove the discussion group. Use the method getSuitableDiscussionChats to find all suitable groups. Basic group chats needs to be first upgraded to supergroup chats. If new chat members don't have access to old messages in the supergroup, then toggleSupergroupIsAllHistoryAvailable needs to be used first to change that"]
        pub discussion_chat_id: i64,
    }
    impl Method for SetChatDiscussionGroup {
        const TYPE: &'static str = "setChatDiscussionGroup";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the location of a chat. Available only for some location-based supergroups, use supergroupFullInfo.can_set_location to check whether the method is allowed to use"]
    pub struct SetChatLocation {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New location for the chat; must be valid and not null"]
        pub location: ChatLocation,
    }
    impl Method for SetChatLocation {
        const TYPE: &'static str = "setChatLocation";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the slow mode delay of a chat. Available only for supergroups; requires can_restrict_members rights"]
    pub struct SetChatSlowModeDelay {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "New slow mode delay for the chat; must be one of 0, 10, 30, 60, 300, 900, 3600"]
        pub slow_mode_delay: i32,
    }
    impl Method for SetChatSlowModeDelay {
        const TYPE: &'static str = "setChatSlowModeDelay";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Pins a message in a chat; requires can_pin_messages rights"]
    pub struct PinChatMessage {
        #[doc = "Identifier of the chat"]
        pub chat_id: i64,
        #[doc = "Identifier of the new pinned message"]
        pub message_id: i64,
        #[doc = "True, if there should be no notification about the pinned message"]
        pub disable_notification: bool,
    }
    impl Method for PinChatMessage {
        const TYPE: &'static str = "pinChatMessage";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes the pinned message from a chat; requires can_pin_messages rights in the group or channel"]
    pub struct UnpinChatMessage {
        #[doc = "Identifier of the chat"]
        pub chat_id: i64,
    }
    impl Method for UnpinChatMessage {
        const TYPE: &'static str = "unpinChatMessage";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds current user as a new member to a chat. Private and secret chats can't be joined using this method"]
    pub struct JoinChat {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
    }
    impl Method for JoinChat {
        const TYPE: &'static str = "joinChat";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes current user from chat members. Private and secret chats can't be left using this method"]
    pub struct LeaveChat {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
    }
    impl Method for LeaveChat {
        const TYPE: &'static str = "leaveChat";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds a new member to a chat. Members can't be added to private or secret chats. Members will not be added until the chat state has been synchronized with the server"]
    pub struct AddChatMember {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Identifier of the user"]
        pub user_id: i32,
        #[doc = "The number of earlier messages from the chat to be forwarded to the new member; up to 100. Ignored for supergroups and channels"]
        pub forward_limit: i32,
    }
    impl Method for AddChatMember {
        const TYPE: &'static str = "addChatMember";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds multiple new members to a chat. Currently this option is only available for supergroups and channels. This option can't be used to join a chat. Members can't be added to a channel if it has more than 200 members. Members will not be added until the chat state has been synchronized with the server"]
    pub struct AddChatMembers {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Identifiers of the users to be added to the chat"]
        pub user_ids: Vec<i32>,
    }
    impl Method for AddChatMembers {
        const TYPE: &'static str = "addChatMembers";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the status of a chat member, needs appropriate privileges. This function is currently not suitable for adding new members to the chat and transferring chat ownership; instead, use addChatMember or transferChatOwnership. The chat member status will not be changed until it has been synchronized with the server"]
    pub struct SetChatMemberStatus {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "User identifier"]
        pub user_id: i32,
        #[doc = "The new status of the member in the chat"]
        pub status: ChatMemberStatus,
    }
    impl Method for SetChatMemberStatus {
        const TYPE: &'static str = "setChatMemberStatus";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks whether the current session can be used to transfer a chat ownership to another user"]
    pub struct CanTransferOwnership {}
    impl Method for CanTransferOwnership {
        const TYPE: &'static str = "canTransferOwnership";
        type Response = CanTransferOwnershipResult;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the owner of a chat. The current user must be a current owner of the chat. Use the method canTransferOwnership to check whether the ownership can be transferred from the current session. Available only for supergroups and channel chats"]
    pub struct TransferChatOwnership {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Identifier of the user to which transfer the ownership. The ownership can't be transferred to a bot or to a deleted user"]
        pub user_id: i32,
        #[doc = "The password of the current user"]
        pub password: String,
    }
    impl Method for TransferChatOwnership {
        const TYPE: &'static str = "transferChatOwnership";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a single member of a chat"]
    pub struct GetChatMember {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "User identifier"]
        pub user_id: i32,
    }
    impl Method for GetChatMember {
        const TYPE: &'static str = "getChatMember";
        type Response = ChatMember;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for a specified query in the first name, last name and username of the members of a specified chat. Requires administrator rights in channels"]
    pub struct SearchChatMembers {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Query to search for"]
        pub query: String,
        #[doc = "The maximum number of users to be returned"]
        pub limit: i32,
        #[doc = "The type of users to return. By default, chatMembersFilterMembers"]
        pub filter: ChatMembersFilter,
    }
    impl Method for SearchChatMembers {
        const TYPE: &'static str = "searchChatMembers";
        type Response = ChatMembers;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of administrators of the chat with their custom titles"]
    pub struct GetChatAdministrators {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
    }
    impl Method for GetChatAdministrators {
        const TYPE: &'static str = "getChatAdministrators";
        type Response = ChatAdministrators;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Clears draft messages in all chats"]
    pub struct ClearAllDraftMessages {
        #[doc = "If true, local draft messages in secret chats will not be cleared"]
        pub exclude_secret_chats: bool,
    }
    impl Method for ClearAllDraftMessages {
        const TYPE: &'static str = "clearAllDraftMessages";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns list of chats with non-default notification settings"]
    pub struct GetChatNotificationSettingsExceptions {
        #[doc = "If specified, only chats from the specified scope will be returned"]
        pub scope: NotificationSettingsScope,
        #[doc = "If true, also chats with non-default sound will be returned"]
        pub compare_sound: bool,
    }
    impl Method for GetChatNotificationSettingsExceptions {
        const TYPE: &'static str = "getChatNotificationSettingsExceptions";
        type Response = Chats;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the notification settings for chats of a given type"]
    pub struct GetScopeNotificationSettings {
        #[doc = "Types of chats for which to return the notification settings information"]
        pub scope: NotificationSettingsScope,
    }
    impl Method for GetScopeNotificationSettings {
        const TYPE: &'static str = "getScopeNotificationSettings";
        type Response = ScopeNotificationSettings;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes notification settings for chats of a given type"]
    pub struct SetScopeNotificationSettings {
        #[doc = "Types of chats for which to change the notification settings"]
        pub scope: NotificationSettingsScope,
        #[doc = "The new notification settings for the given scope"]
        pub notification_settings: ScopeNotificationSettings,
    }
    impl Method for SetScopeNotificationSettings {
        const TYPE: &'static str = "setScopeNotificationSettings";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Resets all notification settings to their default values. By default, all chats are unmuted, the sound is set to \"default\" and message previews are shown"]
    pub struct ResetAllNotificationSettings {}
    impl Method for ResetAllNotificationSettings {
        const TYPE: &'static str = "resetAllNotificationSettings";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the order of pinned chats"]
    pub struct SetPinnedChats {
        #[doc = "Chat list in which to change the order of pinned chats"]
        pub chat_list: ChatList,
        #[doc = "The new list of pinned chats"]
        pub chat_ids: Vec<i64>,
    }
    impl Method for SetPinnedChats {
        const TYPE: &'static str = "setPinnedChats";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Downloads a file from the cloud. Download progress and completion of the download will be notified through updateFile updates"]
    pub struct DownloadFile {
        #[doc = "Identifier of the file to download"]
        pub file_id: i32,
        #[doc = "Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which downloadFile was called will be downloaded first"]
        pub priority: i32,
        #[doc = "The starting position from which the file should be downloaded"]
        pub offset: i32,
        #[doc = "Number of bytes which should be downloaded starting from the \"offset\" position before the download will be automatically cancelled; use 0 to download without a limit"]
        pub limit: i32,
        #[doc = "If false, this request returns file state just after the download has been started. If true, this request returns file state only after the download has succeeded, has failed, has been cancelled or a new downloadFile request with different offset/limit parameters was sent"]
        pub synchronous: bool,
    }
    impl Method for DownloadFile {
        const TYPE: &'static str = "downloadFile";
        type Response = File;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns file downloaded prefix size from a given offset"]
    pub struct GetFileDownloadedPrefixSize {
        #[doc = "Identifier of the file"]
        pub file_id: i32,
        #[doc = "Offset from which downloaded prefix size should be calculated"]
        pub offset: i32,
    }
    impl Method for GetFileDownloadedPrefixSize {
        const TYPE: &'static str = "getFileDownloadedPrefixSize";
        type Response = Count;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Stops the downloading of a file. If a file has already been downloaded, does nothing"]
    pub struct CancelDownloadFile {
        #[doc = "Identifier of a file to stop downloading"]
        pub file_id: i32,
        #[doc = "Pass true to stop downloading only if it hasn't been started, i.e. request hasn't been sent to server"]
        pub only_if_pending: bool,
    }
    impl Method for CancelDownloadFile {
        const TYPE: &'static str = "cancelDownloadFile";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Asynchronously uploads a file to the cloud without sending it in a message. updateFile will be used to notify about upload progress and successful completion of the upload. The file will not have a persistent remote identifier until it will be sent in a message"]
    pub struct UploadFile {
        #[doc = "File to upload"]
        pub file: InputFile,
        #[doc = "File type"]
        pub file_type: FileType,
        #[doc = "Priority of the upload (1-32). The higher the priority, the earlier the file will be uploaded. If the priorities of two files are equal, then the first one for which uploadFile was called will be uploaded first"]
        pub priority: i32,
    }
    impl Method for UploadFile {
        const TYPE: &'static str = "uploadFile";
        type Response = File;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Stops the uploading of a file. Supported only for files uploaded by using uploadFile. For other files the behavior is undefined"]
    pub struct CancelUploadFile {
        #[doc = "Identifier of the file to stop uploading"]
        pub file_id: i32,
    }
    impl Method for CancelUploadFile {
        const TYPE: &'static str = "cancelUploadFile";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Writes a part of a generated file. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct write to the destination file"]
    pub struct WriteGeneratedFilePart {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "The identifier of the generation process"]
        pub generation_id: i64,
        #[doc = "The offset from which to write the data to the file"]
        pub offset: i32,
        #[doc = "The data to write"]
        pub data: String,
    }
    impl Method for WriteGeneratedFilePart {
        const TYPE: &'static str = "writeGeneratedFilePart";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Informs TDLib on a file generation progress"]
    pub struct SetFileGenerationProgress {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "The identifier of the generation process"]
        pub generation_id: i64,
        #[doc = "Expected size of the generated file, in bytes; 0 if unknown"]
        pub expected_size: i32,
        #[doc = "The number of bytes already generated"]
        pub local_prefix_size: i32,
    }
    impl Method for SetFileGenerationProgress {
        const TYPE: &'static str = "setFileGenerationProgress";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Finishes the file generation"]
    pub struct FinishFileGeneration {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "The identifier of the generation process"]
        pub generation_id: i64,
        #[doc = "If set, means that file generation has failed and should be terminated"]
        pub error: Error,
    }
    impl Method for FinishFileGeneration {
        const TYPE: &'static str = "finishFileGeneration";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Reads a part of a file from the TDLib file cache and returns read bytes. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct read from the file"]
    pub struct ReadFilePart {
        #[doc = "Identifier of the file. The file must be located in the TDLib file cache"]
        pub file_id: i32,
        #[doc = "The offset from which to read the file"]
        pub offset: i32,
        #[doc = "Number of bytes to read. An error will be returned if there are not enough bytes available in the file from the specified position. Pass 0 to read all available data from the specified position"]
        pub count: i32,
    }
    impl Method for ReadFilePart {
        const TYPE: &'static str = "readFilePart";
        type Response = FilePart;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Deletes a file from the TDLib file cache"]
    pub struct DeleteFile {
        #[doc = "Identifier of the file to delete"]
        pub file_id: i32,
    }
    impl Method for DeleteFile {
        const TYPE: &'static str = "deleteFile";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Generates a new invite link for a chat; the previously generated link is revoked. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right"]
    pub struct GenerateChatInviteLink {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
    }
    impl Method for GenerateChatInviteLink {
        const TYPE: &'static str = "generateChatInviteLink";
        type Response = ChatInviteLink;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks the validity of an invite link for a chat and returns information about the corresponding chat"]
    pub struct CheckChatInviteLink {
        #[doc = "Invite link to be checked; should begin with \"https://t.me/joinchat/\", \"https://telegram.me/joinchat/\", or \"https://telegram.dog/joinchat/\""]
        pub invite_link: String,
    }
    impl Method for CheckChatInviteLink {
        const TYPE: &'static str = "checkChatInviteLink";
        type Response = ChatInviteLinkInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Uses an invite link to add the current user to the chat if possible. The new member will not be added until the chat state has been synchronized with the server"]
    pub struct JoinChatByInviteLink {
        #[doc = "Invite link to import; should begin with \"https://t.me/joinchat/\", \"https://telegram.me/joinchat/\", or \"https://telegram.dog/joinchat/\""]
        pub invite_link: String,
    }
    impl Method for JoinChatByInviteLink {
        const TYPE: &'static str = "joinChatByInviteLink";
        type Response = Chat;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Creates a new call"]
    pub struct CreateCall {
        #[doc = "Identifier of the user to be called"]
        pub user_id: i32,
        #[doc = "Description of the call protocols supported by the client"]
        pub protocol: CallProtocol,
    }
    impl Method for CreateCall {
        const TYPE: &'static str = "createCall";
        type Response = CallId;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Accepts an incoming call"]
    pub struct AcceptCall {
        #[doc = "Call identifier"]
        pub call_id: i32,
        #[doc = "Description of the call protocols supported by the client"]
        pub protocol: CallProtocol,
    }
    impl Method for AcceptCall {
        const TYPE: &'static str = "acceptCall";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Discards a call"]
    pub struct DiscardCall {
        #[doc = "Call identifier"]
        pub call_id: i32,
        #[doc = "True, if the user was disconnected"]
        pub is_disconnected: bool,
        #[doc = "The call duration, in seconds"]
        pub duration: i32,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the connection used during the call"]
        pub connection_id: i64,
    }
    impl Method for DiscardCall {
        const TYPE: &'static str = "discardCall";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends a call rating"]
    pub struct SendCallRating {
        #[doc = "Call identifier"]
        pub call_id: i32,
        #[doc = "Call rating; 1-5"]
        pub rating: i32,
        #[doc = "An optional user comment if the rating is less than 5"]
        pub comment: String,
        #[doc = "List of the exact types of problems with the call, specified by the user"]
        pub problems: Vec<CallProblem>,
    }
    impl Method for SendCallRating {
        const TYPE: &'static str = "sendCallRating";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends debug information for a call"]
    pub struct SendCallDebugInformation {
        #[doc = "Call identifier"]
        pub call_id: i32,
        #[doc = "Debug information in application-specific format"]
        pub debug_information: String,
    }
    impl Method for SendCallDebugInformation {
        const TYPE: &'static str = "sendCallDebugInformation";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds a user to the blacklist"]
    pub struct BlockUser {
        #[doc = "User identifier"]
        pub user_id: i32,
    }
    impl Method for BlockUser {
        const TYPE: &'static str = "blockUser";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes a user from the blacklist"]
    pub struct UnblockUser {
        #[doc = "User identifier"]
        pub user_id: i32,
    }
    impl Method for UnblockUser {
        const TYPE: &'static str = "unblockUser";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns users that were blocked by the current user"]
    pub struct GetBlockedUsers {
        #[doc = "Number of users to skip in the result; must be non-negative"]
        pub offset: i32,
        #[doc = "The maximum number of users to return; up to 100"]
        pub limit: i32,
    }
    impl Method for GetBlockedUsers {
        const TYPE: &'static str = "getBlockedUsers";
        type Response = Users;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds a user to the contact list or edits an existing contact by their user identifier"]
    pub struct AddContact {
        #[doc = "The contact to add or edit; phone number can be empty and needs to be specified only if known, vCard is ignored"]
        pub contact: Contact,
        #[doc = "True, if the new contact needs to be allowed to see current user's phone number. A corresponding rule to userPrivacySettingShowPhoneNumber will be added if needed. Use the field UserFullInfo.need_phone_number_privacy_exception to check whether the current user needs to be asked to share their phone number"]
        pub share_phone_number: bool,
    }
    impl Method for AddContact {
        const TYPE: &'static str = "addContact";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds new contacts or edits existing contacts by their phone numbers; contacts' user identifiers are ignored"]
    pub struct ImportContacts {
        #[doc = "The list of contacts to import or edit; contacts' vCard are ignored and are not imported"]
        pub contacts: Vec<Contact>,
    }
    impl Method for ImportContacts {
        const TYPE: &'static str = "importContacts";
        type Response = ImportedContacts;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns all user contacts"]
    pub struct GetContacts {}
    impl Method for GetContacts {
        const TYPE: &'static str = "getContacts";
        type Response = Users;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for the specified query in the first names, last names and usernames of the known user contacts"]
    pub struct SearchContacts {
        #[doc = "Query to search for; may be empty to return all contacts"]
        pub query: String,
        #[doc = "The maximum number of users to be returned"]
        pub limit: i32,
    }
    impl Method for SearchContacts {
        const TYPE: &'static str = "searchContacts";
        type Response = Users;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes users from the contact list"]
    pub struct RemoveContacts {
        #[doc = "Identifiers of users to be deleted"]
        pub user_ids: Vec<i32>,
    }
    impl Method for RemoveContacts {
        const TYPE: &'static str = "removeContacts";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the total number of imported contacts"]
    pub struct GetImportedContactCount {}
    impl Method for GetImportedContactCount {
        const TYPE: &'static str = "getImportedContactCount";
        type Response = Count;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes imported contacts using the list of current user contacts saved on the device. Imports newly added contacts and, if at least the file database is enabled, deletes recently deleted contacts. Query result depends on the result of the previous query, so only one query is possible at the same time @contacts The new list of contacts, contact's vCard are ignored and are not imported"]
    pub struct ChangeImportedContacts {
        pub contacts: Vec<Contact>,
    }
    impl Method for ChangeImportedContacts {
        const TYPE: &'static str = "changeImportedContacts";
        type Response = ImportedContacts;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Clears all imported contacts, contact list remains unchanged"]
    pub struct ClearImportedContacts {}
    impl Method for ClearImportedContacts {
        const TYPE: &'static str = "clearImportedContacts";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Shares the phone number of the current user with a mutual contact. Supposed to be called when the user clicks on chatActionBarSharePhoneNumber"]
    pub struct SharePhoneNumber {
        #[doc = "Identifier of the user with whom to share the phone number. The user must be a mutual contact"]
        pub user_id: i32,
    }
    impl Method for SharePhoneNumber {
        const TYPE: &'static str = "sharePhoneNumber";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the profile photos of a user. The result of this query may be outdated: some photos might have been deleted already"]
    pub struct GetUserProfilePhotos {
        #[doc = "User identifier"]
        pub user_id: i32,
        #[doc = "The number of photos to skip; must be non-negative"]
        pub offset: i32,
        #[doc = "The maximum number of photos to be returned; up to 100"]
        pub limit: i32,
    }
    impl Method for GetUserProfilePhotos {
        const TYPE: &'static str = "getUserProfilePhotos";
        type Response = UserProfilePhotos;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns stickers from the installed sticker sets that correspond to a given emoji. If the emoji is not empty, favorite and recently used stickers may also be returned"]
    pub struct GetStickers {
        #[doc = "String representation of emoji. If empty, returns all known installed stickers"]
        pub emoji: String,
        #[doc = "The maximum number of stickers to be returned"]
        pub limit: i32,
    }
    impl Method for GetStickers {
        const TYPE: &'static str = "getStickers";
        type Response = Stickers;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for stickers from public sticker sets that correspond to a given emoji"]
    pub struct SearchStickers {
        #[doc = "String representation of emoji; must be non-empty"]
        pub emoji: String,
        #[doc = "The maximum number of stickers to be returned"]
        pub limit: i32,
    }
    impl Method for SearchStickers {
        const TYPE: &'static str = "searchStickers";
        type Response = Stickers;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of installed sticker sets"]
    pub struct GetInstalledStickerSets {
        #[doc = "Pass true to return mask sticker sets; pass false to return ordinary sticker sets"]
        pub is_masks: bool,
    }
    impl Method for GetInstalledStickerSets {
        const TYPE: &'static str = "getInstalledStickerSets";
        type Response = StickerSets;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of archived sticker sets"]
    pub struct GetArchivedStickerSets {
        #[doc = "Pass true to return mask stickers sets; pass false to return ordinary sticker sets"]
        pub is_masks: bool,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the sticker set from which to return the result"]
        pub offset_sticker_set_id: i64,
        #[doc = "The maximum number of sticker sets to return"]
        pub limit: i32,
    }
    impl Method for GetArchivedStickerSets {
        const TYPE: &'static str = "getArchivedStickerSets";
        type Response = StickerSets;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of trending sticker sets"]
    pub struct GetTrendingStickerSets {}
    impl Method for GetTrendingStickerSets {
        const TYPE: &'static str = "getTrendingStickerSets";
        type Response = StickerSets;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of sticker sets attached to a file. Currently only photos and videos can have attached sticker sets"]
    pub struct GetAttachedStickerSets {
        #[doc = "File identifier"]
        pub file_id: i32,
    }
    impl Method for GetAttachedStickerSets {
        const TYPE: &'static str = "getAttachedStickerSets";
        type Response = StickerSets;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a sticker set by its identifier"]
    pub struct GetStickerSet {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the sticker set"]
        pub set_id: i64,
    }
    impl Method for GetStickerSet {
        const TYPE: &'static str = "getStickerSet";
        type Response = StickerSet;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for a sticker set by its name"]
    pub struct SearchStickerSet {
        #[doc = "Name of the sticker set"]
        pub name: String,
    }
    impl Method for SearchStickerSet {
        const TYPE: &'static str = "searchStickerSet";
        type Response = StickerSet;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for installed sticker sets by looking for specified query in their title and name"]
    pub struct SearchInstalledStickerSets {
        #[doc = "Pass true to return mask sticker sets; pass false to return ordinary sticker sets"]
        pub is_masks: bool,
        #[doc = "Query to search for"]
        pub query: String,
        #[doc = "The maximum number of sticker sets to return"]
        pub limit: i32,
    }
    impl Method for SearchInstalledStickerSets {
        const TYPE: &'static str = "searchInstalledStickerSets";
        type Response = StickerSets;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for ordinary sticker sets by looking for specified query in their title and name. Excludes installed sticker sets from the results"]
    pub struct SearchStickerSets {
        #[doc = "Query to search for"]
        pub query: String,
    }
    impl Method for SearchStickerSets {
        const TYPE: &'static str = "searchStickerSets";
        type Response = StickerSets;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Installs/uninstalls or activates/archives a sticker set"]
    pub struct ChangeStickerSet {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the sticker set"]
        pub set_id: i64,
        #[doc = "The new value of is_installed"]
        pub is_installed: bool,
        #[doc = "The new value of is_archived. A sticker set can't be installed and archived simultaneously"]
        pub is_archived: bool,
    }
    impl Method for ChangeStickerSet {
        const TYPE: &'static str = "changeStickerSet";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Informs the server that some trending sticker sets have been viewed by the user"]
    pub struct ViewTrendingStickerSets {
        #[doc = "Identifiers of viewed trending sticker sets"]
        pub sticker_set_ids: Vec<i64>,
    }
    impl Method for ViewTrendingStickerSets {
        const TYPE: &'static str = "viewTrendingStickerSets";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the order of installed sticker sets"]
    pub struct ReorderInstalledStickerSets {
        #[doc = "Pass true to change the order of mask sticker sets; pass false to change the order of ordinary sticker sets"]
        pub is_masks: bool,
        #[doc = "Identifiers of installed sticker sets in the new correct order"]
        pub sticker_set_ids: Vec<i64>,
    }
    impl Method for ReorderInstalledStickerSets {
        const TYPE: &'static str = "reorderInstalledStickerSets";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of recently used stickers"]
    pub struct GetRecentStickers {
        #[doc = "Pass true to return stickers and masks that were recently attached to photos or video files; pass false to return recently sent stickers"]
        pub is_attached: bool,
    }
    impl Method for GetRecentStickers {
        const TYPE: &'static str = "getRecentStickers";
        type Response = Stickers;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Manually adds a new sticker to the list of recently used stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list"]
    pub struct AddRecentSticker {
        #[doc = "Pass true to add the sticker to the list of stickers recently attached to photo or video files; pass false to add the sticker to the list of recently sent stickers"]
        pub is_attached: bool,
        #[doc = "Sticker file to add"]
        pub sticker: InputFile,
    }
    impl Method for AddRecentSticker {
        const TYPE: &'static str = "addRecentSticker";
        type Response = Stickers;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes a sticker from the list of recently used stickers"]
    pub struct RemoveRecentSticker {
        #[doc = "Pass true to remove the sticker from the list of stickers recently attached to photo or video files; pass false to remove the sticker from the list of recently sent stickers"]
        pub is_attached: bool,
        #[doc = "Sticker file to delete"]
        pub sticker: InputFile,
    }
    impl Method for RemoveRecentSticker {
        const TYPE: &'static str = "removeRecentSticker";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Clears the list of recently used stickers"]
    pub struct ClearRecentStickers {
        #[doc = "Pass true to clear the list of stickers recently attached to photo or video files; pass false to clear the list of recently sent stickers"]
        pub is_attached: bool,
    }
    impl Method for ClearRecentStickers {
        const TYPE: &'static str = "clearRecentStickers";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns favorite stickers"]
    pub struct GetFavoriteStickers {}
    impl Method for GetFavoriteStickers {
        const TYPE: &'static str = "getFavoriteStickers";
        type Response = Stickers;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds a new sticker to the list of favorite stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list"]
    pub struct AddFavoriteSticker {
        #[doc = "Sticker file to add"]
        pub sticker: InputFile,
    }
    impl Method for AddFavoriteSticker {
        const TYPE: &'static str = "addFavoriteSticker";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes a sticker from the list of favorite stickers"]
    pub struct RemoveFavoriteSticker {
        #[doc = "Sticker file to delete from the list"]
        pub sticker: InputFile,
    }
    impl Method for RemoveFavoriteSticker {
        const TYPE: &'static str = "removeFavoriteSticker";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns emoji corresponding to a sticker. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object"]
    pub struct GetStickerEmojis {
        #[doc = "Sticker file identifier"]
        pub sticker: InputFile,
    }
    impl Method for GetStickerEmojis {
        const TYPE: &'static str = "getStickerEmojis";
        type Response = Emojis;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for emojis by keywords. Supported only if the file database is enabled"]
    pub struct SearchEmojis {
        #[doc = "Text to search for"]
        pub text: String,
        #[doc = "True, if only emojis, which exactly match text needs to be returned"]
        pub exact_match: bool,
        #[doc = "IETF language tag of the user's input language; may be empty if unknown"]
        pub input_language_code: String,
    }
    impl Method for SearchEmojis {
        const TYPE: &'static str = "searchEmojis";
        type Response = Emojis;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns an HTTP URL which can be used to automatically log in to the translation platform and suggest new emoji replacements. The URL will be valid for 30 seconds after generation"]
    pub struct GetEmojiSuggestionsUrl {
        #[doc = "Language code for which the emoji replacements will be suggested"]
        pub language_code: String,
    }
    impl Method for GetEmojiSuggestionsUrl {
        const TYPE: &'static str = "getEmojiSuggestionsUrl";
        type Response = HttpUrl;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns saved animations"]
    pub struct GetSavedAnimations {}
    impl Method for GetSavedAnimations {
        const TYPE: &'static str = "getSavedAnimations";
        type Response = Animations;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Manually adds a new animation to the list of saved animations. The new animation is added to the beginning of the list. If the animation was already in the list, it is removed first. Only non-secret video animations with MIME type \"video/mp4\" can be added to the list"]
    pub struct AddSavedAnimation {
        #[doc = "The animation file to be added. Only animations known to the server (i.e. successfully sent via a message) can be added to the list"]
        pub animation: InputFile,
    }
    impl Method for AddSavedAnimation {
        const TYPE: &'static str = "addSavedAnimation";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes an animation from the list of saved animations"]
    pub struct RemoveSavedAnimation {
        #[doc = "Animation file to be removed"]
        pub animation: InputFile,
    }
    impl Method for RemoveSavedAnimation {
        const TYPE: &'static str = "removeSavedAnimation";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns up to 20 recently used inline bots in the order of their last usage"]
    pub struct GetRecentInlineBots {}
    impl Method for GetRecentInlineBots {
        const TYPE: &'static str = "getRecentInlineBots";
        type Response = Users;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for recently used hashtags by their prefix"]
    pub struct SearchHashtags {
        #[doc = "Hashtag prefix to search for"]
        pub prefix: String,
        #[doc = "The maximum number of hashtags to be returned"]
        pub limit: i32,
    }
    impl Method for SearchHashtags {
        const TYPE: &'static str = "searchHashtags";
        type Response = Hashtags;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes a hashtag from the list of recently used hashtags"]
    pub struct RemoveRecentHashtag {
        #[doc = "Hashtag to delete"]
        pub hashtag: String,
    }
    impl Method for RemoveRecentHashtag {
        const TYPE: &'static str = "removeRecentHashtag";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a web page preview by the text of the message. Do not call this function too often. Returns a 404 error if the web page has no preview"]
    pub struct GetWebPagePreview {
        #[doc = "Message text with formatting"]
        pub text: FormattedText,
    }
    impl Method for GetWebPagePreview {
        const TYPE: &'static str = "getWebPagePreview";
        type Response = WebPage;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns an instant view version of a web page if available. Returns a 404 error if the web page has no instant view page"]
    pub struct GetWebPageInstantView {
        #[doc = "The web page URL"]
        pub url: String,
        #[doc = "If true, the full instant view for the web page will be returned"]
        pub force_full: bool,
    }
    impl Method for GetWebPageInstantView {
        const TYPE: &'static str = "getWebPageInstantView";
        type Response = WebPageInstantView;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Uploads a new profile photo for the current user. If something changes, updateUser will be sent"]
    pub struct SetProfilePhoto {
        #[doc = "Profile photo to set. inputFileId and inputFileRemote may still be unsupported"]
        pub photo: InputFile,
    }
    impl Method for SetProfilePhoto {
        const TYPE: &'static str = "setProfilePhoto";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Deletes a profile photo. If something changes, updateUser will be sent"]
    pub struct DeleteProfilePhoto {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of the profile photo to delete"]
        pub profile_photo_id: i64,
    }
    impl Method for DeleteProfilePhoto {
        const TYPE: &'static str = "deleteProfilePhoto";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the first and last name of the current user. If something changes, updateUser will be sent"]
    pub struct SetName {
        #[doc = "The new value of the first name for the user; 1-64 characters"]
        pub first_name: String,
        #[doc = "The new value of the optional last name for the user; 0-64 characters"]
        pub last_name: String,
    }
    impl Method for SetName {
        const TYPE: &'static str = "setName";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the bio of the current user"]
    pub struct SetBio {
        #[doc = "The new value of the user bio; 0-70 characters without line feeds"]
        pub bio: String,
    }
    impl Method for SetBio {
        const TYPE: &'static str = "setBio";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the username of the current user. If something changes, updateUser will be sent"]
    pub struct SetUsername {
        #[doc = "The new value of the username. Use an empty string to remove the username"]
        pub username: String,
    }
    impl Method for SetUsername {
        const TYPE: &'static str = "setUsername";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the phone number of the user and sends an authentication code to the user's new phone number. On success, returns information about the sent code"]
    pub struct ChangePhoneNumber {
        #[doc = "The new phone number of the user in international format"]
        pub phone_number: String,
        #[doc = "Settings for the authentication of the user's phone number"]
        pub settings: PhoneNumberAuthenticationSettings,
    }
    impl Method for ChangePhoneNumber {
        const TYPE: &'static str = "changePhoneNumber";
        type Response = AuthenticationCodeInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Re-sends the authentication code sent to confirm a new phone number for the user. Works only if the previously received authenticationCodeInfo next_code_type was not null"]
    pub struct ResendChangePhoneNumberCode {}
    impl Method for ResendChangePhoneNumberCode {
        const TYPE: &'static str = "resendChangePhoneNumberCode";
        type Response = AuthenticationCodeInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks the authentication code sent to confirm a new phone number of the user"]
    pub struct CheckChangePhoneNumberCode {
        #[doc = "Verification code received by SMS, phone call or flash call"]
        pub code: String,
    }
    impl Method for CheckChangePhoneNumberCode {
        const TYPE: &'static str = "checkChangePhoneNumberCode";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns all active sessions of the current user"]
    pub struct GetActiveSessions {}
    impl Method for GetActiveSessions {
        const TYPE: &'static str = "getActiveSessions";
        type Response = Sessions;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Terminates a session of the current user"]
    pub struct TerminateSession {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Session identifier"]
        pub session_id: i64,
    }
    impl Method for TerminateSession {
        const TYPE: &'static str = "terminateSession";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Terminates all other sessions of the current user"]
    pub struct TerminateAllOtherSessions {}
    impl Method for TerminateAllOtherSessions {
        const TYPE: &'static str = "terminateAllOtherSessions";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns all website where the current user used Telegram to log in"]
    pub struct GetConnectedWebsites {}
    impl Method for GetConnectedWebsites {
        const TYPE: &'static str = "getConnectedWebsites";
        type Response = ConnectedWebsites;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Disconnects website from the current user's Telegram account"]
    pub struct DisconnectWebsite {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Website identifier"]
        pub website_id: i64,
    }
    impl Method for DisconnectWebsite {
        const TYPE: &'static str = "disconnectWebsite";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Disconnects all websites from the current user's Telegram account"]
    pub struct DisconnectAllWebsites {}
    impl Method for DisconnectAllWebsites {
        const TYPE: &'static str = "disconnectAllWebsites";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the username of a supergroup or channel, requires owner privileges in the supergroup or channel"]
    pub struct SetSupergroupUsername {
        #[doc = "Identifier of the supergroup or channel"]
        pub supergroup_id: i32,
        #[doc = "New value of the username. Use an empty string to remove the username"]
        pub username: String,
    }
    impl Method for SetSupergroupUsername {
        const TYPE: &'static str = "setSupergroupUsername";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the sticker set of a supergroup; requires can_change_info rights"]
    pub struct SetSupergroupStickerSet {
        #[doc = "Identifier of the supergroup"]
        pub supergroup_id: i32,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "New value of the supergroup sticker set identifier. Use 0 to remove the supergroup sticker set"]
        pub sticker_set_id: i64,
    }
    impl Method for SetSupergroupStickerSet {
        const TYPE: &'static str = "setSupergroupStickerSet";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Toggles sender signatures messages sent in a channel; requires can_change_info rights"]
    pub struct ToggleSupergroupSignMessages {
        #[doc = "Identifier of the channel"]
        pub supergroup_id: i32,
        #[doc = "New value of sign_messages"]
        pub sign_messages: bool,
    }
    impl Method for ToggleSupergroupSignMessages {
        const TYPE: &'static str = "toggleSupergroupSignMessages";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Toggles whether the message history of a supergroup is available to new members; requires can_change_info rights"]
    pub struct ToggleSupergroupIsAllHistoryAvailable {
        #[doc = "The identifier of the supergroup"]
        pub supergroup_id: i32,
        #[doc = "The new value of is_all_history_available"]
        pub is_all_history_available: bool,
    }
    impl Method for ToggleSupergroupIsAllHistoryAvailable {
        const TYPE: &'static str = "toggleSupergroupIsAllHistoryAvailable";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Reports some messages from a user in a supergroup as spam; requires administrator rights in the supergroup"]
    pub struct ReportSupergroupSpam {
        #[doc = "Supergroup identifier"]
        pub supergroup_id: i32,
        #[doc = "User identifier"]
        pub user_id: i32,
        #[doc = "Identifiers of messages sent in the supergroup by the user. This list must be non-empty"]
        pub message_ids: Vec<i64>,
    }
    impl Method for ReportSupergroupSpam {
        const TYPE: &'static str = "reportSupergroupSpam";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about members or banned users in a supergroup or channel. Can be used only if SupergroupFullInfo.can_get_members == true; additionally, administrator privileges may be required for some filters"]
    pub struct GetSupergroupMembers {
        #[doc = "Identifier of the supergroup or channel"]
        pub supergroup_id: i32,
        #[doc = "The type of users to return. By default, supergroupMembersRecent"]
        pub filter: SupergroupMembersFilter,
        #[doc = "Number of users to skip"]
        pub offset: i32,
        #[doc = "The maximum number of users be returned; up to 200"]
        pub limit: i32,
    }
    impl Method for GetSupergroupMembers {
        const TYPE: &'static str = "getSupergroupMembers";
        type Response = ChatMembers;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Deletes a supergroup or channel along with all messages in the corresponding chat. This will release the supergroup or channel username and remove all members; requires owner privileges in the supergroup or channel. Chats with more than 1000 members can't be deleted using this method"]
    pub struct DeleteSupergroup {
        #[doc = "Identifier of the supergroup or channel"]
        pub supergroup_id: i32,
    }
    impl Method for DeleteSupergroup {
        const TYPE: &'static str = "deleteSupergroup";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Closes a secret chat, effectively transferring its state to secretChatStateClosed"]
    pub struct CloseSecretChat {
        #[doc = "Secret chat identifier"]
        pub secret_chat_id: i32,
    }
    impl Method for CloseSecretChat {
        const TYPE: &'static str = "closeSecretChat";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a list of service actions taken by chat members and administrators in the last 48 hours. Available only for supergroups and channels. Requires administrator rights. Returns results in reverse chronological order (i. e., in order of decreasing event_id)"]
    pub struct GetChatEventLog {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Search query by which to filter events"]
        pub query: String,
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of an event from which to return results. Use 0 to get results from the latest events"]
        pub from_event_id: i64,
        #[doc = "The maximum number of events to return; up to 100"]
        pub limit: i32,
        #[doc = "The types of events to return. By default, all types will be returned"]
        pub filters: ChatEventLogFilters,
        #[doc = "User identifiers by which to filter events. By default, events relating to all users will be returned"]
        pub user_ids: Vec<i32>,
    }
    impl Method for GetChatEventLog {
        const TYPE: &'static str = "getChatEventLog";
        type Response = ChatEvents;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns an invoice payment form. This method should be called when the user presses inlineKeyboardButtonBuy"]
    pub struct GetPaymentForm {
        #[doc = "Chat identifier of the Invoice message"]
        pub chat_id: i64,
        #[doc = "Message identifier"]
        pub message_id: i64,
    }
    impl Method for GetPaymentForm {
        const TYPE: &'static str = "getPaymentForm";
        type Response = PaymentForm;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Validates the order information provided by a user and returns the available shipping options for a flexible invoice"]
    pub struct ValidateOrderInfo {
        #[doc = "Chat identifier of the Invoice message"]
        pub chat_id: i64,
        #[doc = "Message identifier"]
        pub message_id: i64,
        #[doc = "The order information, provided by the user"]
        pub order_info: OrderInfo,
        #[doc = "True, if the order information can be saved"]
        pub allow_save: bool,
    }
    impl Method for ValidateOrderInfo {
        const TYPE: &'static str = "validateOrderInfo";
        type Response = ValidatedOrderInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends a filled-out payment form to the bot for final verification"]
    pub struct SendPaymentForm {
        #[doc = "Chat identifier of the Invoice message"]
        pub chat_id: i64,
        #[doc = "Message identifier"]
        pub message_id: i64,
        #[doc = "Identifier returned by ValidateOrderInfo, or an empty string"]
        pub order_info_id: String,
        #[doc = "Identifier of a chosen shipping option, if applicable"]
        pub shipping_option_id: String,
        #[doc = "The credentials chosen by user for payment"]
        pub credentials: InputCredentials,
    }
    impl Method for SendPaymentForm {
        const TYPE: &'static str = "sendPaymentForm";
        type Response = PaymentResult;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a successful payment"]
    pub struct GetPaymentReceipt {
        #[doc = "Chat identifier of the PaymentSuccessful message"]
        pub chat_id: i64,
        #[doc = "Message identifier"]
        pub message_id: i64,
    }
    impl Method for GetPaymentReceipt {
        const TYPE: &'static str = "getPaymentReceipt";
        type Response = PaymentReceipt;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns saved order info, if any"]
    pub struct GetSavedOrderInfo {}
    impl Method for GetSavedOrderInfo {
        const TYPE: &'static str = "getSavedOrderInfo";
        type Response = OrderInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Deletes saved order info"]
    pub struct DeleteSavedOrderInfo {}
    impl Method for DeleteSavedOrderInfo {
        const TYPE: &'static str = "deleteSavedOrderInfo";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Deletes saved credentials for all payment provider bots"]
    pub struct DeleteSavedCredentials {}
    impl Method for DeleteSavedCredentials {
        const TYPE: &'static str = "deleteSavedCredentials";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a user that can be contacted to get support"]
    pub struct GetSupportUser {}
    impl Method for GetSupportUser {
        const TYPE: &'static str = "getSupportUser";
        type Response = User;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns backgrounds installed by the user"]
    pub struct GetBackgrounds {
        #[doc = "True, if the backgrounds needs to be ordered for dark theme"]
        pub for_dark_theme: bool,
    }
    impl Method for GetBackgrounds {
        const TYPE: &'static str = "getBackgrounds";
        type Response = Backgrounds;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Constructs a persistent HTTP URL for a background"]
    pub struct GetBackgroundUrl {
        #[doc = "Background name"]
        pub name: String,
        #[serde(rename = "type")]
        #[doc = "Background type"]
        pub type_: BackgroundType,
    }
    impl Method for GetBackgroundUrl {
        const TYPE: &'static str = "getBackgroundUrl";
        type Response = HttpUrl;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Searches for a background by its name"]
    pub struct SearchBackground {
        #[doc = "The name of the background"]
        pub name: String,
    }
    impl Method for SearchBackground {
        const TYPE: &'static str = "searchBackground";
        type Response = Background;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the background selected by the user; adds background to the list of installed backgrounds"]
    pub struct SetBackground {
        #[doc = "The input background to use, null for filled backgrounds"]
        pub background: InputBackground,
        #[serde(rename = "type")]
        #[doc = "Background type; null for default background. The method will return error 404 if type is null"]
        pub type_: BackgroundType,
        #[doc = "True, if the background is chosen for dark theme"]
        pub for_dark_theme: bool,
    }
    impl Method for SetBackground {
        const TYPE: &'static str = "setBackground";
        type Response = Background;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes background from the list of installed backgrounds"]
    pub struct RemoveBackground {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "The background identifier"]
        pub background_id: i64,
    }
    impl Method for RemoveBackground {
        const TYPE: &'static str = "removeBackground";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Resets list of installed backgrounds to its default value"]
    pub struct ResetBackgrounds {}
    impl Method for ResetBackgrounds {
        const TYPE: &'static str = "resetBackgrounds";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about the current localization target. This is an offline request if only_local is true. Can be called before authorization"]
    pub struct GetLocalizationTargetInfo {
        #[doc = "If true, returns only locally available information without sending network requests"]
        pub only_local: bool,
    }
    impl Method for GetLocalizationTargetInfo {
        const TYPE: &'static str = "getLocalizationTargetInfo";
        type Response = LocalizationTargetInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a language pack. Returned language pack identifier may be different from a provided one. Can be called before authorization"]
    pub struct GetLanguagePackInfo {
        #[doc = "Language pack identifier"]
        pub language_pack_id: String,
    }
    impl Method for GetLanguagePackInfo {
        const TYPE: &'static str = "getLanguagePackInfo";
        type Response = LanguagePackInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns strings from a language pack in the current localization target by their keys. Can be called before authorization"]
    pub struct GetLanguagePackStrings {
        #[doc = "Language pack identifier of the strings to be returned"]
        pub language_pack_id: String,
        #[doc = "Language pack keys of the strings to be returned; leave empty to request all available strings"]
        pub keys: Vec<String>,
    }
    impl Method for GetLanguagePackStrings {
        const TYPE: &'static str = "getLanguagePackStrings";
        type Response = LanguagePackStrings;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Fetches the latest versions of all strings from a language pack in the current localization target from the server. This method doesn't need to be called explicitly for the current used/base language packs. Can be called before authorization"]
    pub struct SynchronizeLanguagePack {
        #[doc = "Language pack identifier"]
        pub language_pack_id: String,
    }
    impl Method for SynchronizeLanguagePack {
        const TYPE: &'static str = "synchronizeLanguagePack";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds a custom server language pack to the list of installed language packs in current localization target. Can be called before authorization"]
    pub struct AddCustomServerLanguagePack {
        #[doc = "Identifier of a language pack to be added; may be different from a name that is used in an \"https://t.me/setlanguage/\" link"]
        pub language_pack_id: String,
    }
    impl Method for AddCustomServerLanguagePack {
        const TYPE: &'static str = "addCustomServerLanguagePack";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds or changes a custom local language pack to the current localization target"]
    pub struct SetCustomLanguagePack {
        #[doc = "Information about the language pack. Language pack ID must start with 'X', consist only of English letters, digits and hyphens, and must not exceed 64 characters. Can be called before authorization"]
        pub info: LanguagePackInfo,
        #[doc = "Strings of the new language pack"]
        pub strings: Vec<LanguagePackString>,
    }
    impl Method for SetCustomLanguagePack {
        const TYPE: &'static str = "setCustomLanguagePack";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits information about a custom local language pack in the current localization target. Can be called before authorization"]
    pub struct EditCustomLanguagePackInfo {
        #[doc = "New information about the custom local language pack"]
        pub info: LanguagePackInfo,
    }
    impl Method for EditCustomLanguagePackInfo {
        const TYPE: &'static str = "editCustomLanguagePackInfo";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds, edits or deletes a string in a custom local language pack. Can be called before authorization"]
    pub struct SetCustomLanguagePackString {
        #[doc = "Identifier of a previously added custom local language pack in the current localization target"]
        pub language_pack_id: String,
        #[doc = "New language pack string"]
        pub new_string: LanguagePackString,
    }
    impl Method for SetCustomLanguagePackString {
        const TYPE: &'static str = "setCustomLanguagePackString";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Deletes all information about a language pack in the current localization target. The language pack which is currently in use (including base language pack) or is being synchronized can't be deleted. Can be called before authorization"]
    pub struct DeleteLanguagePack {
        #[doc = "Identifier of the language pack to delete"]
        pub language_pack_id: String,
    }
    impl Method for DeleteLanguagePack {
        const TYPE: &'static str = "deleteLanguagePack";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Registers the currently used device for receiving push notifications. Returns a globally unique identifier of the push notification subscription"]
    pub struct RegisterDevice {
        #[doc = "Device token"]
        pub device_token: DeviceToken,
        #[doc = "List of user identifiers of other users currently using the client"]
        pub other_user_ids: Vec<i32>,
    }
    impl Method for RegisterDevice {
        const TYPE: &'static str = "registerDevice";
        type Response = PushReceiverId;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Handles a push notification. Returns error with code 406 if the push notification is not supported and connection to the server is required to fetch new data. Can be called before authorization"]
    pub struct ProcessPushNotification {
        #[doc = "JSON-encoded push notification payload with all fields sent by the server, and \"google.sent_time\" and \"google.notification.sound\" fields added"]
        pub payload: String,
    }
    impl Method for ProcessPushNotification {
        const TYPE: &'static str = "processPushNotification";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a globally unique push notification subscription identifier for identification of an account, which has received a push notification. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct GetPushReceiverId {
        #[doc = "JSON-encoded push notification payload"]
        pub payload: String,
    }
    impl Method for GetPushReceiverId {
        const TYPE: &'static str = "getPushReceiverId";
        type Response = PushReceiverId;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns t.me URLs recently visited by a newly registered user"]
    pub struct GetRecentlyVisitedTMeUrls {
        #[doc = "Google Play referrer to identify the user"]
        pub referrer: String,
    }
    impl Method for GetRecentlyVisitedTMeUrls {
        const TYPE: &'static str = "getRecentlyVisitedTMeUrls";
        type Response = TMeUrls;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes user privacy settings"]
    pub struct SetUserPrivacySettingRules {
        #[doc = "The privacy setting"]
        pub setting: UserPrivacySetting,
        #[doc = "The new privacy rules"]
        pub rules: UserPrivacySettingRules,
    }
    impl Method for SetUserPrivacySettingRules {
        const TYPE: &'static str = "setUserPrivacySettingRules";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the current privacy settings"]
    pub struct GetUserPrivacySettingRules {
        #[doc = "The privacy setting"]
        pub setting: UserPrivacySetting,
    }
    impl Method for GetUserPrivacySettingRules {
        const TYPE: &'static str = "getUserPrivacySettingRules";
        type Response = UserPrivacySettingRules;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the value of an option by its name. (Check the list of available options on https://core.telegram.org/tdlib/options.) Can be called before authorization"]
    pub struct GetOption {
        #[doc = "The name of the option"]
        pub name: String,
    }
    impl Method for GetOption {
        const TYPE: &'static str = "getOption";
        type Response = OptionValue;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sets the value of an option. (Check the list of available options on https://core.telegram.org/tdlib/options.) Only writable options can be set. Can be called before authorization"]
    pub struct SetOption {
        #[doc = "The name of the option"]
        pub name: String,
        #[doc = "The new value of the option"]
        pub value: OptionValue,
    }
    impl Method for SetOption {
        const TYPE: &'static str = "setOption";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the period of inactivity after which the account of the current user will automatically be deleted"]
    pub struct SetAccountTtl {
        #[doc = "New account TTL"]
        pub ttl: AccountTtl,
    }
    impl Method for SetAccountTtl {
        const TYPE: &'static str = "setAccountTtl";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the period of inactivity after which the account of the current user will automatically be deleted"]
    pub struct GetAccountTtl {}
    impl Method for GetAccountTtl {
        const TYPE: &'static str = "getAccountTtl";
        type Response = AccountTtl;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Deletes the account of the current user, deleting all information associated with the user from the server. The phone number of the account can be used to create a new account. Can be called before authorization when the current authorization state is authorizationStateWaitPassword"]
    pub struct DeleteAccount {
        #[doc = "The reason why the account was deleted; optional"]
        pub reason: String,
    }
    impl Method for DeleteAccount {
        const TYPE: &'static str = "deleteAccount";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes a chat action bar without any other action"]
    pub struct RemoveChatActionBar {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
    }
    impl Method for RemoveChatActionBar {
        const TYPE: &'static str = "removeChatActionBar";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Reports a chat to the Telegram moderators. Supported only for supergroups, channels, or private chats with bots, since other chats can't be checked by moderators, or when the report is done from the chat action bar"]
    pub struct ReportChat {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "The reason for reporting the chat"]
        pub reason: ChatReportReason,
        #[doc = "Identifiers of reported messages, if any"]
        pub message_ids: Vec<i64>,
    }
    impl Method for ReportChat {
        const TYPE: &'static str = "reportChat";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns an HTTP URL with the chat statistics. Currently this method can be used only for channels. Can be used only if SupergroupFullInfo.can_view_statistics == true"]
    pub struct GetChatStatisticsUrl {
        #[doc = "Chat identifier"]
        pub chat_id: i64,
        #[doc = "Parameters from \"tg://statsrefresh?params=******\" link"]
        pub parameters: String,
        #[doc = "Pass true if a URL with the dark theme must be returned"]
        pub is_dark: bool,
    }
    impl Method for GetChatStatisticsUrl {
        const TYPE: &'static str = "getChatStatisticsUrl";
        type Response = HttpUrl;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns storage usage statistics. Can be called before authorization"]
    pub struct GetStorageStatistics {
        #[doc = "The maximum number of chats with the largest storage usage for which separate statistics should be returned. All other chats will be grouped in entries with chat_id == 0. If the chat info database is not used, the chat_limit is ignored and is always set to 0"]
        pub chat_limit: i32,
    }
    impl Method for GetStorageStatistics {
        const TYPE: &'static str = "getStorageStatistics";
        type Response = StorageStatistics;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Quickly returns approximate storage usage statistics. Can be called before authorization"]
    pub struct GetStorageStatisticsFast {}
    impl Method for GetStorageStatisticsFast {
        const TYPE: &'static str = "getStorageStatisticsFast";
        type Response = StorageStatisticsFast;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns database statistics"]
    pub struct GetDatabaseStatistics {}
    impl Method for GetDatabaseStatistics {
        const TYPE: &'static str = "getDatabaseStatistics";
        type Response = DatabaseStatistics;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Optimizes storage usage, i.e. deletes some files and returns new storage usage statistics. Secret thumbnails can't be deleted"]
    pub struct OptimizeStorage {
        #[doc = "Limit on the total size of files after deletion. Pass -1 to use the default limit"]
        pub size: i64,
        #[doc = "Limit on the time that has passed since the last time a file was accessed (or creation time for some filesystems). Pass -1 to use the default limit"]
        pub ttl: i32,
        #[doc = "Limit on the total count of files after deletion. Pass -1 to use the default limit"]
        pub count: i32,
        #[doc = "The amount of time after the creation of a file during which it can't be deleted, in seconds. Pass -1 to use the default value"]
        pub immunity_delay: i32,
        #[doc = "If not empty, only files with the given type(s) are considered. By default, all types except thumbnails, profile photos, stickers and wallpapers are deleted"]
        pub file_types: Vec<FileType>,
        #[doc = "If not empty, only files from the given chats are considered. Use 0 as chat identifier to delete files not belonging to any chat (e.g., profile photos)"]
        pub chat_ids: Vec<i64>,
        #[doc = "If not empty, files from the given chats are excluded. Use 0 as chat identifier to exclude all files not belonging to any chat (e.g., profile photos)"]
        pub exclude_chat_ids: Vec<i64>,
        #[doc = "Same as in getStorageStatistics. Affects only returned statistics"]
        pub chat_limit: i32,
    }
    impl Method for OptimizeStorage {
        const TYPE: &'static str = "optimizeStorage";
        type Response = StorageStatistics;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sets the current network type. Can be called before authorization. Calling this method forces all network connections to reopen, mitigating the delay in switching between different networks, so it should be called whenever the network is changed, even if the network type remains the same. Network type is used to check whether the library can use the network at all and also for collecting detailed network data usage statistics @type The new network type. By default, networkTypeOther"]
    pub struct SetNetworkType {
        #[serde(rename = "type")]
        pub type_: NetworkType,
    }
    impl Method for SetNetworkType {
        const TYPE: &'static str = "setNetworkType";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns network data usage statistics. Can be called before authorization"]
    pub struct GetNetworkStatistics {
        #[doc = "If true, returns only data for the current library launch"]
        pub only_current: bool,
    }
    impl Method for GetNetworkStatistics {
        const TYPE: &'static str = "getNetworkStatistics";
        type Response = NetworkStatistics;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds the specified data to data usage statistics. Can be called before authorization"]
    pub struct AddNetworkStatistics {
        #[doc = "The network statistics entry with the data to be added to statistics"]
        pub entry: NetworkStatisticsEntry,
    }
    impl Method for AddNetworkStatistics {
        const TYPE: &'static str = "addNetworkStatistics";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Resets all network data usage statistics to zero. Can be called before authorization"]
    pub struct ResetNetworkStatistics {}
    impl Method for ResetNetworkStatistics {
        const TYPE: &'static str = "resetNetworkStatistics";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns auto-download settings presets for the currently logged in user"]
    pub struct GetAutoDownloadSettingsPresets {}
    impl Method for GetAutoDownloadSettingsPresets {
        const TYPE: &'static str = "getAutoDownloadSettingsPresets";
        type Response = AutoDownloadSettingsPresets;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sets auto-download settings"]
    pub struct SetAutoDownloadSettings {
        #[doc = "New user auto-download settings"]
        pub settings: AutoDownloadSettings,
        #[serde(rename = "type")]
        #[doc = "Type of the network for which the new settings are applied"]
        pub type_: NetworkType,
    }
    impl Method for SetAutoDownloadSettings {
        const TYPE: &'static str = "setAutoDownloadSettings";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns one of the available Telegram Passport elements"]
    pub struct GetPassportElement {
        #[serde(rename = "type")]
        #[doc = "Telegram Passport element type"]
        pub type_: PassportElementType,
        #[doc = "Password of the current user"]
        pub password: String,
    }
    impl Method for GetPassportElement {
        const TYPE: &'static str = "getPassportElement";
        type Response = PassportElement;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns all available Telegram Passport elements"]
    pub struct GetAllPassportElements {
        #[doc = "Password of the current user"]
        pub password: String,
    }
    impl Method for GetAllPassportElements {
        const TYPE: &'static str = "getAllPassportElements";
        type Response = PassportElements;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds an element to the user's Telegram Passport. May return an error with a message \"PHONE_VERIFICATION_NEEDED\" or \"EMAIL_VERIFICATION_NEEDED\" if the chosen phone number or the chosen email address must be verified first"]
    pub struct SetPassportElement {
        #[doc = "Input Telegram Passport element"]
        pub element: InputPassportElement,
        #[doc = "Password of the current user"]
        pub password: String,
    }
    impl Method for SetPassportElement {
        const TYPE: &'static str = "setPassportElement";
        type Response = PassportElement;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Deletes a Telegram Passport element"]
    pub struct DeletePassportElement {
        #[serde(rename = "type")]
        #[doc = "Element type"]
        pub type_: PassportElementType,
    }
    impl Method for DeletePassportElement {
        const TYPE: &'static str = "deletePassportElement";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Informs the user that some of the elements in their Telegram Passport contain errors; for bots only. The user will not be able to resend the elements, until the errors are fixed"]
    pub struct SetPassportElementErrors {
        #[doc = "User identifier"]
        pub user_id: i32,
        #[doc = "The errors"]
        pub errors: Vec<InputPassportElementError>,
    }
    impl Method for SetPassportElementErrors {
        const TYPE: &'static str = "setPassportElementErrors";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns an IETF language tag of the language preferred in the country, which should be used to fill native fields in Telegram Passport personal details. Returns a 404 error if unknown"]
    pub struct GetPreferredCountryLanguage {
        #[doc = "A two-letter ISO 3166-1 alpha-2 country code"]
        pub country_code: String,
    }
    impl Method for GetPreferredCountryLanguage {
        const TYPE: &'static str = "getPreferredCountryLanguage";
        type Response = Text;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends a code to verify a phone number to be added to a user's Telegram Passport"]
    pub struct SendPhoneNumberVerificationCode {
        #[doc = "The phone number of the user, in international format"]
        pub phone_number: String,
        #[doc = "Settings for the authentication of the user's phone number"]
        pub settings: PhoneNumberAuthenticationSettings,
    }
    impl Method for SendPhoneNumberVerificationCode {
        const TYPE: &'static str = "sendPhoneNumberVerificationCode";
        type Response = AuthenticationCodeInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Re-sends the code to verify a phone number to be added to a user's Telegram Passport"]
    pub struct ResendPhoneNumberVerificationCode {}
    impl Method for ResendPhoneNumberVerificationCode {
        const TYPE: &'static str = "resendPhoneNumberVerificationCode";
        type Response = AuthenticationCodeInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks the phone number verification code for Telegram Passport"]
    pub struct CheckPhoneNumberVerificationCode {
        #[doc = "Verification code"]
        pub code: String,
    }
    impl Method for CheckPhoneNumberVerificationCode {
        const TYPE: &'static str = "checkPhoneNumberVerificationCode";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends a code to verify an email address to be added to a user's Telegram Passport"]
    pub struct SendEmailAddressVerificationCode {
        #[doc = "Email address"]
        pub email_address: String,
    }
    impl Method for SendEmailAddressVerificationCode {
        const TYPE: &'static str = "sendEmailAddressVerificationCode";
        type Response = EmailAddressAuthenticationCodeInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Re-sends the code to verify an email address to be added to a user's Telegram Passport"]
    pub struct ResendEmailAddressVerificationCode {}
    impl Method for ResendEmailAddressVerificationCode {
        const TYPE: &'static str = "resendEmailAddressVerificationCode";
        type Response = EmailAddressAuthenticationCodeInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks the email address verification code for Telegram Passport"]
    pub struct CheckEmailAddressVerificationCode {
        #[doc = "Verification code"]
        pub code: String,
    }
    impl Method for CheckEmailAddressVerificationCode {
        const TYPE: &'static str = "checkEmailAddressVerificationCode";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns a Telegram Passport authorization form for sharing data with a service"]
    pub struct GetPassportAuthorizationForm {
        #[doc = "User identifier of the service's bot"]
        pub bot_user_id: i32,
        #[doc = "Telegram Passport element types requested by the service"]
        pub scope: String,
        #[doc = "Service's public_key"]
        pub public_key: String,
        #[doc = "Authorization form nonce provided by the service"]
        pub nonce: String,
    }
    impl Method for GetPassportAuthorizationForm {
        const TYPE: &'static str = "getPassportAuthorizationForm";
        type Response = PassportAuthorizationForm;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns already available Telegram Passport elements suitable for completing a Telegram Passport authorization form. Result can be received only once for each authorization form"]
    pub struct GetPassportAuthorizationFormAvailableElements {
        #[doc = "Authorization form identifier"]
        pub autorization_form_id: i32,
        #[doc = "Password of the current user"]
        pub password: String,
    }
    impl Method for GetPassportAuthorizationFormAvailableElements {
        const TYPE: &'static str = "getPassportAuthorizationFormAvailableElements";
        type Response = PassportElementsWithErrors;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends a Telegram Passport authorization form, effectively sharing data with the service. This method must be called after getPassportAuthorizationFormAvailableElements if some previously available elements need to be used"]
    pub struct SendPassportAuthorizationForm {
        #[doc = "Authorization form identifier"]
        pub autorization_form_id: i32,
        #[doc = "Types of Telegram Passport elements chosen by user to complete the authorization form"]
        pub types: Vec<PassportElementType>,
    }
    impl Method for SendPassportAuthorizationForm {
        const TYPE: &'static str = "sendPassportAuthorizationForm";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends phone number confirmation code. Should be called when user presses \"https://t.me/confirmphone?phone=*******&hash=**********\" or \"tg://confirmphone?phone=*******&hash=**********\" link"]
    pub struct SendPhoneNumberConfirmationCode {
        #[doc = "Value of the \"hash\" parameter from the link"]
        pub hash: String,
        #[doc = "Value of the \"phone\" parameter from the link"]
        pub phone_number: String,
        #[doc = "Settings for the authentication of the user's phone number"]
        pub settings: PhoneNumberAuthenticationSettings,
    }
    impl Method for SendPhoneNumberConfirmationCode {
        const TYPE: &'static str = "sendPhoneNumberConfirmationCode";
        type Response = AuthenticationCodeInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Resends phone number confirmation code"]
    pub struct ResendPhoneNumberConfirmationCode {}
    impl Method for ResendPhoneNumberConfirmationCode {
        const TYPE: &'static str = "resendPhoneNumberConfirmationCode";
        type Response = AuthenticationCodeInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Checks phone number confirmation code"]
    pub struct CheckPhoneNumberConfirmationCode {
        #[doc = "The phone number confirmation code"]
        pub code: String,
    }
    impl Method for CheckPhoneNumberConfirmationCode {
        const TYPE: &'static str = "checkPhoneNumberConfirmationCode";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Informs the server about the number of pending bot updates if they haven't been processed for a long time; for bots only"]
    pub struct SetBotUpdatesStatus {
        #[doc = "The number of pending updates"]
        pub pending_update_count: i32,
        #[doc = "The last error message"]
        pub error_message: String,
    }
    impl Method for SetBotUpdatesStatus {
        const TYPE: &'static str = "setBotUpdatesStatus";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Uploads a PNG image with a sticker; for bots only; returns the uploaded file"]
    pub struct UploadStickerFile {
        #[doc = "Sticker file owner"]
        pub user_id: i32,
        #[doc = "PNG image with the sticker; must be up to 512 kB in size and fit in 512x512 square"]
        pub png_sticker: InputFile,
    }
    impl Method for UploadStickerFile {
        const TYPE: &'static str = "uploadStickerFile";
        type Response = File;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Creates a new sticker set; for bots only. Returns the newly created sticker set"]
    pub struct CreateNewStickerSet {
        #[doc = "Sticker set owner"]
        pub user_id: i32,
        #[doc = "Sticker set title; 1-64 characters"]
        pub title: String,
        #[doc = "Sticker set name. Can contain only English letters, digits and underscores. Must end with *\"_by_<bot username>\"* (*<bot_username>* is case insensitive); 1-64 characters"]
        pub name: String,
        #[doc = "True, if stickers are masks"]
        pub is_masks: bool,
        #[doc = "List of stickers to be added to the set"]
        pub stickers: Vec<InputSticker>,
    }
    impl Method for CreateNewStickerSet {
        const TYPE: &'static str = "createNewStickerSet";
        type Response = StickerSet;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds a new sticker to a set; for bots only. Returns the sticker set"]
    pub struct AddStickerToSet {
        #[doc = "Sticker set owner"]
        pub user_id: i32,
        #[doc = "Sticker set name"]
        pub name: String,
        #[doc = "Sticker to add to the set"]
        pub sticker: InputSticker,
    }
    impl Method for AddStickerToSet {
        const TYPE: &'static str = "addStickerToSet";
        type Response = StickerSet;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Changes the position of a sticker in the set to which it belongs; for bots only. The sticker set must have been created by the bot"]
    pub struct SetStickerPositionInSet {
        #[doc = "Sticker"]
        pub sticker: InputFile,
        #[doc = "New position of the sticker in the set, zero-based"]
        pub position: i32,
    }
    impl Method for SetStickerPositionInSet {
        const TYPE: &'static str = "setStickerPositionInSet";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes a sticker from the set to which it belongs; for bots only. The sticker set must have been created by the bot"]
    pub struct RemoveStickerFromSet {
        #[doc = "Sticker"]
        pub sticker: InputFile,
    }
    impl Method for RemoveStickerFromSet {
        const TYPE: &'static str = "removeStickerFromSet";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a file with a map thumbnail in PNG format. Only map thumbnail files with size less than 1MB can be downloaded"]
    pub struct GetMapThumbnailFile {
        #[doc = "Location of the map center"]
        pub location: Location,
        #[doc = "Map zoom level; 13-20"]
        pub zoom: i32,
        #[doc = "Map width in pixels before applying scale; 16-1024"]
        pub width: i32,
        #[doc = "Map height in pixels before applying scale; 16-1024"]
        pub height: i32,
        #[doc = "Map scale; 1-3"]
        pub scale: i32,
        #[doc = "Identifier of a chat, in which the thumbnail will be shown. Use 0 if unknown"]
        pub chat_id: i64,
    }
    impl Method for GetMapThumbnailFile {
        const TYPE: &'static str = "getMapThumbnailFile";
        type Response = File;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Accepts Telegram terms of services"]
    pub struct AcceptTermsOfService {
        #[doc = "Terms of service identifier"]
        pub terms_of_service_id: String,
    }
    impl Method for AcceptTermsOfService {
        const TYPE: &'static str = "acceptTermsOfService";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends a custom request; for bots only"]
    pub struct SendCustomRequest {
        #[doc = "The method name"]
        pub method: String,
        #[doc = "JSON-serialized method parameters"]
        pub parameters: String,
    }
    impl Method for SendCustomRequest {
        const TYPE: &'static str = "sendCustomRequest";
        type Response = CustomRequestResult;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Answers a custom query; for bots only"]
    pub struct AnswerCustomQuery {
        #[serde(deserialize_with = "super::utils::from_str_to_t")]
        #[doc = "Identifier of a custom query"]
        pub custom_query_id: i64,
        #[doc = "JSON-serialized answer to the query"]
        pub data: String,
    }
    impl Method for AnswerCustomQuery {
        const TYPE: &'static str = "answerCustomQuery";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Succeeds after a specified amount of time has passed. Can be called before authorization. Can be called before initialization"]
    pub struct SetAlarm {
        #[doc = "Number of seconds before the function returns"]
        pub seconds: f64,
    }
    impl Method for SetAlarm {
        const TYPE: &'static str = "setAlarm";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Uses current user IP to found their country. Returns two-letter ISO 3166-1 alpha-2 country code. Can be called before authorization"]
    pub struct GetCountryCode {}
    impl Method for GetCountryCode {
        const TYPE: &'static str = "getCountryCode";
        type Response = Text;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the default text for invitation messages to be used as a placeholder when the current user invites friends to Telegram"]
    pub struct GetInviteText {}
    impl Method for GetInviteText {
        const TYPE: &'static str = "getInviteText";
        type Response = Text;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about a tg:// deep link. Use \"tg://need_update_for_some_feature\" or \"tg:some_unsupported_feature\" for testing. Returns a 404 error for unknown links. Can be called before authorization"]
    pub struct GetDeepLinkInfo {
        #[doc = "The link"]
        pub link: String,
    }
    impl Method for GetDeepLinkInfo {
        const TYPE: &'static str = "getDeepLinkInfo";
        type Response = DeepLinkInfo;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns application config, provided by the server. Can be called before authorization"]
    pub struct GetApplicationConfig {}
    impl Method for GetApplicationConfig {
        const TYPE: &'static str = "getApplicationConfig";
        type Response = JsonValue;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Saves application log event on the server. Can be called before authorization"]
    pub struct SaveApplicationLogEvent {
        #[serde(rename = "type")]
        #[doc = "Event type"]
        pub type_: String,
        #[doc = "Optional chat identifier, associated with the event"]
        pub chat_id: i64,
        #[doc = "The log event data"]
        pub data: JsonValue,
    }
    impl Method for SaveApplicationLogEvent {
        const TYPE: &'static str = "saveApplicationLogEvent";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds a proxy server for network requests. Can be called before authorization"]
    pub struct AddProxy {
        #[doc = "Proxy server IP address"]
        pub server: String,
        #[doc = "Proxy server port"]
        pub port: i32,
        #[doc = "True, if the proxy should be enabled"]
        pub enable: bool,
        #[serde(rename = "type")]
        #[doc = "Proxy type"]
        pub type_: ProxyType,
    }
    impl Method for AddProxy {
        const TYPE: &'static str = "addProxy";
        type Response = Proxy;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Edits an existing proxy server for network requests. Can be called before authorization"]
    pub struct EditProxy {
        #[doc = "Proxy identifier"]
        pub proxy_id: i32,
        #[doc = "Proxy server IP address"]
        pub server: String,
        #[doc = "Proxy server port"]
        pub port: i32,
        #[doc = "True, if the proxy should be enabled"]
        pub enable: bool,
        #[serde(rename = "type")]
        #[doc = "Proxy type"]
        pub type_: ProxyType,
    }
    impl Method for EditProxy {
        const TYPE: &'static str = "editProxy";
        type Response = Proxy;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Enables a proxy. Only one proxy can be enabled at a time. Can be called before authorization"]
    pub struct EnableProxy {
        #[doc = "Proxy identifier"]
        pub proxy_id: i32,
    }
    impl Method for EnableProxy {
        const TYPE: &'static str = "enableProxy";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Disables the currently enabled proxy. Can be called before authorization"]
    pub struct DisableProxy {}
    impl Method for DisableProxy {
        const TYPE: &'static str = "disableProxy";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Removes a proxy server. Can be called before authorization"]
    pub struct RemoveProxy {
        #[doc = "Proxy identifier"]
        pub proxy_id: i32,
    }
    impl Method for RemoveProxy {
        const TYPE: &'static str = "removeProxy";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns list of proxies that are currently set up. Can be called before authorization"]
    pub struct GetProxies {}
    impl Method for GetProxies {
        const TYPE: &'static str = "getProxies";
        type Response = Proxies;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns an HTTPS link, which can be used to add a proxy. Available only for SOCKS5 and MTProto proxies. Can be called before authorization"]
    pub struct GetProxyLink {
        #[doc = "Proxy identifier"]
        pub proxy_id: i32,
    }
    impl Method for GetProxyLink {
        const TYPE: &'static str = "getProxyLink";
        type Response = Text;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Computes time needed to receive a response from a Telegram server through a proxy. Can be called before authorization"]
    pub struct PingProxy {
        #[doc = "Proxy identifier. Use 0 to ping a Telegram server without a proxy"]
        pub proxy_id: i32,
    }
    impl Method for PingProxy {
        const TYPE: &'static str = "pingProxy";
        type Response = Seconds;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sets new log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct SetLogStream {
        #[doc = "New log stream"]
        pub log_stream: LogStream,
    }
    impl Method for SetLogStream {
        const TYPE: &'static str = "setLogStream";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns information about currently used log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct GetLogStream {}
    impl Method for GetLogStream {
        const TYPE: &'static str = "getLogStream";
        type Response = LogStream;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sets the verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct SetLogVerbosityLevel {
        #[doc = "New value of the verbosity level for logging. Value 0 corresponds to fatal errors, value 1 corresponds to errors, value 2 corresponds to warnings and debug warnings, value 3 corresponds to informational, value 4 corresponds to debug, value 5 corresponds to verbose debug, value greater than 5 and up to 1023 can be used to enable even more logging"]
        pub new_verbosity_level: i32,
    }
    impl Method for SetLogVerbosityLevel {
        const TYPE: &'static str = "setLogVerbosityLevel";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns current verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct GetLogVerbosityLevel {}
    impl Method for GetLogVerbosityLevel {
        const TYPE: &'static str = "getLogVerbosityLevel";
        type Response = LogVerbosityLevel;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns list of available TDLib internal log tags, for example, [\"actor\", \"binlog\", \"connections\", \"notifications\", \"proxy\"]. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct GetLogTags {}
    impl Method for GetLogTags {
        const TYPE: &'static str = "getLogTags";
        type Response = LogTags;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sets the verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct SetLogTagVerbosityLevel {
        #[doc = "Logging tag to change verbosity level"]
        pub tag: String,
        #[doc = "New verbosity level; 1-1024"]
        pub new_verbosity_level: i32,
    }
    impl Method for SetLogTagVerbosityLevel {
        const TYPE: &'static str = "setLogTagVerbosityLevel";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns current verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct GetLogTagVerbosityLevel {
        #[doc = "Logging tag to change verbosity level"]
        pub tag: String,
    }
    impl Method for GetLogTagVerbosityLevel {
        const TYPE: &'static str = "getLogTagVerbosityLevel";
        type Response = LogVerbosityLevel;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Adds a message to TDLib internal log. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct AddLogMessage {
        #[doc = "The minimum verbosity level needed for the message to be logged, 0-1023"]
        pub verbosity_level: i32,
        #[doc = "Text of a message to log"]
        pub text: String,
    }
    impl Method for AddLogMessage {
        const TYPE: &'static str = "addLogMessage";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Does nothing; for testing only. This is an offline method. Can be called before authorization"]
    pub struct TestCallEmpty {}
    impl Method for TestCallEmpty {
        const TYPE: &'static str = "testCallEmpty";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the received string; for testing only. This is an offline method. Can be called before authorization"]
    pub struct TestCallString {
        #[doc = "String to return"]
        pub x: String,
    }
    impl Method for TestCallString {
        const TYPE: &'static str = "testCallString";
        type Response = TestString;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the received bytes; for testing only. This is an offline method. Can be called before authorization"]
    pub struct TestCallBytes {
        #[doc = "Bytes to return"]
        pub x: String,
    }
    impl Method for TestCallBytes {
        const TYPE: &'static str = "testCallBytes";
        type Response = TestBytes;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the received vector of numbers; for testing only. This is an offline method. Can be called before authorization"]
    pub struct TestCallVectorInt {
        #[doc = "Vector of numbers to return"]
        pub x: Vec<i32>,
    }
    impl Method for TestCallVectorInt {
        const TYPE: &'static str = "testCallVectorInt";
        type Response = TestVectorInt;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the received vector of objects containing a number; for testing only. This is an offline method. Can be called before authorization"]
    pub struct TestCallVectorIntObject {
        #[doc = "Vector of objects to return"]
        pub x: Vec<TestInt>,
    }
    impl Method for TestCallVectorIntObject {
        const TYPE: &'static str = "testCallVectorIntObject";
        type Response = TestVectorIntObject;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the received vector of strings; for testing only. This is an offline method. Can be called before authorization"]
    pub struct TestCallVectorString {
        #[doc = "Vector of strings to return"]
        pub x: Vec<String>,
    }
    impl Method for TestCallVectorString {
        const TYPE: &'static str = "testCallVectorString";
        type Response = TestVectorString;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the received vector of objects containing a string; for testing only. This is an offline method. Can be called before authorization"]
    pub struct TestCallVectorStringObject {
        #[doc = "Vector of objects to return"]
        pub x: Vec<TestString>,
    }
    impl Method for TestCallVectorStringObject {
        const TYPE: &'static str = "testCallVectorStringObject";
        type Response = TestVectorStringObject;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the squared received number; for testing only. This is an offline method. Can be called before authorization"]
    pub struct TestSquareInt {
        #[doc = "Number to square"]
        pub x: i32,
    }
    impl Method for TestSquareInt {
        const TYPE: &'static str = "testSquareInt";
        type Response = TestInt;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends a simple network request to the Telegram servers; for testing only. Can be called before authorization"]
    pub struct TestNetwork {}
    impl Method for TestNetwork {
        const TYPE: &'static str = "testNetwork";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Sends a simple network request to the Telegram servers via proxy; for testing only. Can be called before authorization"]
    pub struct TestProxy {
        #[doc = "Proxy server IP address"]
        pub server: String,
        #[doc = "Proxy server port"]
        pub port: i32,
        #[serde(rename = "type")]
        #[doc = "Proxy type"]
        pub type_: ProxyType,
        #[doc = "Identifier of a datacenter, with which to test connection"]
        pub dc_id: i32,
        #[doc = "The maximum overall timeout for the request"]
        pub timeout: f64,
    }
    impl Method for TestProxy {
        const TYPE: &'static str = "testProxy";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Forces an updates.getDifference call to the Telegram servers; for testing only"]
    pub struct TestGetDifference {}
    impl Method for TestGetDifference {
        const TYPE: &'static str = "testGetDifference";
        type Response = Ok;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization"]
    pub struct TestUseUpdate {}
    impl Method for TestUseUpdate {
        const TYPE: &'static str = "testUseUpdate";
        type Response = Update;
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[doc = "Returns the specified error and ensures that the Error object is used; for testing only. This is an offline method. Can be called before authorization. Can be called synchronously"]
    pub struct TestReturnError {
        #[doc = "The error to be returned"]
        pub error: Error,
    }
    impl Method for TestReturnError {
        const TYPE: &'static str = "testReturnError";
        type Response = Error;
    }
}
