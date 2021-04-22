// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Gmail* crate version *2.0.5+20210322*, where *20210322* is the exact revision of the *gmail:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v2.0.5*.
//! 
//! Everything else about the *Gmail* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/gmail/api/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/gmail1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Gmail) ... 
//! 
//! * users
//!  * [*drafts create*](api::UserDraftCreateCall), [*drafts delete*](api::UserDraftDeleteCall), [*drafts get*](api::UserDraftGetCall), [*drafts list*](api::UserDraftListCall), [*drafts send*](api::UserDraftSendCall), [*drafts update*](api::UserDraftUpdateCall), [*get profile*](api::UserGetProfileCall), [*history list*](api::UserHistoryListCall), [*labels create*](api::UserLabelCreateCall), [*labels delete*](api::UserLabelDeleteCall), [*labels get*](api::UserLabelGetCall), [*labels list*](api::UserLabelListCall), [*labels patch*](api::UserLabelPatchCall), [*labels update*](api::UserLabelUpdateCall), [*messages attachments get*](api::UserMessageAttachmentGetCall), [*messages batch delete*](api::UserMessageBatchDeleteCall), [*messages batch modify*](api::UserMessageBatchModifyCall), [*messages delete*](api::UserMessageDeleteCall), [*messages get*](api::UserMessageGetCall), [*messages import*](api::UserMessageImportCall), [*messages insert*](api::UserMessageInsertCall), [*messages list*](api::UserMessageListCall), [*messages modify*](api::UserMessageModifyCall), [*messages send*](api::UserMessageSendCall), [*messages trash*](api::UserMessageTrashCall), [*messages untrash*](api::UserMessageUntrashCall), [*settings delegates create*](api::UserSettingDelegateCreateCall), [*settings delegates delete*](api::UserSettingDelegateDeleteCall), [*settings delegates get*](api::UserSettingDelegateGetCall), [*settings delegates list*](api::UserSettingDelegateListCall), [*settings filters create*](api::UserSettingFilterCreateCall), [*settings filters delete*](api::UserSettingFilterDeleteCall), [*settings filters get*](api::UserSettingFilterGetCall), [*settings filters list*](api::UserSettingFilterListCall), [*settings forwarding addresses create*](api::UserSettingForwardingAddresseCreateCall), [*settings forwarding addresses delete*](api::UserSettingForwardingAddresseDeleteCall), [*settings forwarding addresses get*](api::UserSettingForwardingAddresseGetCall), [*settings forwarding addresses list*](api::UserSettingForwardingAddresseListCall), [*settings get auto forwarding*](api::UserSettingGetAutoForwardingCall), [*settings get imap*](api::UserSettingGetImapCall), [*settings get language*](api::UserSettingGetLanguageCall), [*settings get pop*](api::UserSettingGetPopCall), [*settings get vacation*](api::UserSettingGetVacationCall), [*settings send as create*](api::UserSettingSendACreateCall), [*settings send as delete*](api::UserSettingSendADeleteCall), [*settings send as get*](api::UserSettingSendAGetCall), [*settings send as list*](api::UserSettingSendAListCall), [*settings send as patch*](api::UserSettingSendAPatchCall), [*settings send as smime info delete*](api::UserSettingSendASmimeInfoDeleteCall), [*settings send as smime info get*](api::UserSettingSendASmimeInfoGetCall), [*settings send as smime info insert*](api::UserSettingSendASmimeInfoInsertCall), [*settings send as smime info list*](api::UserSettingSendASmimeInfoListCall), [*settings send as smime info set default*](api::UserSettingSendASmimeInfoSetDefaultCall), [*settings send as update*](api::UserSettingSendAUpdateCall), [*settings send as verify*](api::UserSettingSendAVerifyCall), [*settings update auto forwarding*](api::UserSettingUpdateAutoForwardingCall), [*settings update imap*](api::UserSettingUpdateImapCall), [*settings update language*](api::UserSettingUpdateLanguageCall), [*settings update pop*](api::UserSettingUpdatePopCall), [*settings update vacation*](api::UserSettingUpdateVacationCall), [*stop*](api::UserStopCall), [*threads delete*](api::UserThreadDeleteCall), [*threads get*](api::UserThreadGetCall), [*threads list*](api::UserThreadListCall), [*threads modify*](api::UserThreadModifyCall), [*threads trash*](api::UserThreadTrashCall), [*threads untrash*](api::UserThreadUntrashCall) and [*watch*](api::UserWatchCall)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*drafts create users*](api::UserDraftCreateCall)
//! * [*drafts send users*](api::UserDraftSendCall)
//! * [*drafts update users*](api::UserDraftUpdateCall)
//! * [*messages import users*](api::UserMessageImportCall)
//! * [*messages insert users*](api::UserMessageInsertCall)
//! * [*messages send users*](api::UserMessageSendCall)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](Gmail)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](client::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](client::CallBuilder)
//! * **[Resources](client::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](client::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](client::CallBuilder)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.users().drafts_send(...).doit().await
//! let r = hub.users().messages_get(...).doit().await
//! let r = hub.users().messages_import(...).doit().await
//! let r = hub.users().messages_insert(...).doit().await
//! let r = hub.users().messages_modify(...).doit().await
//! let r = hub.users().messages_send(...).doit().await
//! let r = hub.users().messages_trash(...).doit().await
//! let r = hub.users().messages_untrash(...).doit().await
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-gmail1 = "*"
//! hyper = "^0.14"
//! hyper-rustls = "^0.22"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^5.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_gmail1 as gmail1;
//! use gmail1::api::Message;
//! use gmail1::{Result, Error};
//! use std::fs;
//! # async fn dox() {
//! use std::default::Default;
//! use oauth2;
//! use gmail1::Gmail;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = Gmail::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Message::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `upload_resumable(...)`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.users().messages_import(req, "userId")
//!              .process_for_calendar(true)
//!              .never_mark_spam(true)
//!              .internal_date_source("Lorem")
//!              .deleted(false)
//!              .upload_resumable(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](client::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](client::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](client::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](client::Delegate) to the 
//! [Method Builder](client::CallBuilder) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](client::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](client::RequestValue) and 
//! [decodable](client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](client::Part) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](client::RequestValue) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

pub mod api;
pub mod client;

// Re-export the hub type and some basic client structs
pub use api::Gmail;
pub use client::{Result, Error, Delegate};
