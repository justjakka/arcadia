use std::collections::HashSet;

use reqwest::Url;
pub mod api_doc;
pub mod handlers;
pub mod models;
pub mod periodic_tasks;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod tracker;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum OpenSignups {
    Disabled,
    Enabled,
}

#[derive(Clone)]
pub struct Arcadia {
    pub pool: sqlx::PgPool,
    pub jwt_secret: String,
    pub open_signups: OpenSignups,
    pub tracker_name: String,
    pub frontend_url: Url,
    pub tracker_url: Url,
    pub tracker_announce_interval: u32,
    pub tracker_announce_interval_grace_period: u32,
    pub allowed_torrent_clients: HashSet<Vec<u8>>,
    pub global_upload_factor: f64,
    pub global_download_factor: f64,
    pub tmdb_api_key: Option<String>,
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    pub smtp_from_email: Option<String>,
    pub smtp_from_name: Option<String>,
}

impl Arcadia {
    #[inline]
    pub fn is_open_signups(&self) -> bool {
        self.open_signups == OpenSignups::Enabled
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error: {0}")]
    GenericDatabaseError(#[from] sqlx::Error),

    #[error("{0}")]
    BadRequest(String),

    #[error("account banned")]
    AccountBanned,

    #[error("could not create user application")]
    CouldNotCreateUserApplication(#[source] sqlx::Error),

    #[error("could not get user applications")]
    CouldNotGetUserApplications(#[source] sqlx::Error),

    #[error("could not update user application")]
    CouldNotUpdateUserApplication(#[source] sqlx::Error),

    #[error("could not create artist")]
    CouldNotCreateArtist(#[source] sqlx::Error),

    #[error("could not create artist affiliation")]
    CouldNotCreateArtistAffiliation(#[source] sqlx::Error),

    #[error("could not search for artists")]
    CouldNotSearchForArtists(#[source] sqlx::Error),

    #[error("could not create user")]
    CouldNotCreateUser(#[source] sqlx::Error),

    #[error("username already exists")]
    UsernameAlreadyExists,

    #[error("could not create edition group")]
    CouldNotCreateEditionGroup(#[source] sqlx::Error),

    #[error("could not create invitation")]
    CouldNotCreateInvitation(#[source] sqlx::Error),

    #[error("could not create master group")]
    CouldNotCreateMasterGroup(#[source] sqlx::Error),

    #[error("could not create notification")]
    CouldNotCreateNotification(#[source] sqlx::Error),

    #[error("could not create title group subscription")]
    CouldNotCreateTitleGroupSubscription(#[source] sqlx::Error),

    #[error("could not create title group subscription")]
    CouldNotCreateTitleGroupComment(#[source] sqlx::Error),

    #[error("could not create title group")]
    CouldNotCreateTitleGroup(#[source] sqlx::Error),

    #[error("could not create torrent")]
    CouldNotCreateTorrent(#[source] sqlx::Error),

    #[error("could not create torrent request")]
    CouldNotCreateTorrentRequest(#[source] sqlx::Error),

    #[error("this torrent isn't in the title group of the request")]
    TorrentTitleGroupNotMatchingRequestedOne,

    #[error("this torrent request is already filled")]
    TorrentRequestAlreadyFilled,

    #[error("could not create torrent request vote")]
    CouldNotCreateTorrentRequestVote(#[source] sqlx::Error),

    #[error("could not create torrent report")]
    CouldNotCreateTorrentReport(#[source] sqlx::Error),

    #[error("could not create series")]
    CouldNotCreateSeries(#[source] sqlx::Error),

    #[error("series with id '{0}' not found")]
    SeriesWithIdNotFound(i64),

    #[error("invalid invitation key")]
    InvitationKeyInvalid,

    #[error("email configuration error: {0}")]
    EmailConfigurationError(String),

    #[error("failed to send email: {0}")]
    EmailSendError(String),

    #[error("invitation key required")]
    InvitationKeyRequired,

    #[error("invitation key already used")]
    InvitationKeyAlreadyUsed,

    #[error("no invitations available")]
    NoInvitationsAvailable,

    #[error("user '{0}' not found")]
    UserNotFound(String),

    #[error("user with id '{0}' not found")]
    UserWithIdNotFound(i64),

    #[error("wrong username or password")]
    WrongUsernameOrPassword,

    #[error("invalid or expired refresh token")]
    InvalidOrExpiredRefreshToken,

    #[error("unsupported notification type '{0}'")]
    UnsupportedNotification(String),

    #[error("unsupported subscription type '{0}'")]
    UnsupportedSubscription(String),

    #[error("not enough bonus points to place this bounty")]
    InsufficientBonusPointsForBounty,

    #[error("not enough upload to place this bounty")]
    InsufficientUploadForBounty,

    #[error("torrent file invalid")]
    TorrentFileInvalid,

    #[error("dottorrent file not found")]
    DottorrentFileNotFound,

    #[error("torrent not found")]
    TorrentNotFound,

    #[error("error while updating torrent: '{0}'")]
    ErrorWhileUpdatingTorrent(String),

    #[error("could not save torrent file in path: '{0}'\n'{1}'")]
    CouldNotSaveTorrentFile(String, String),

    #[error("error while searching for torrents: '{0}'")]
    ErrorSearchingForTorrents(String),

    #[error("error while deleting torrent: '{0}'")]
    ErrorDeletingTorrent(String),

    #[error("unexpected third party response")]
    UnexpectedThirdPartyResponse(#[from] reqwest::Error),

    #[error("not enough bonus points available")]
    NotEnoughBonusPointsAvailable,

    #[error("not enough freeleech tokens available")]
    NotEnoughFreeleechTokensAvailable,

    #[error("could not create gift")]
    CouldNotCreateGift(#[source] sqlx::Error),

    #[error("could not create forum post")]
    CouldNotCreateForumPost(#[source] sqlx::Error),

    #[error("could not create forum thread")]
    CouldNotCreateForumThread(#[source] sqlx::Error),

    #[error("could not find forum sub-category")]
    CouldNotFindForumSubCategory(#[source] sqlx::Error),

    #[error("could not find forum thread")]
    CouldNotFindForumThread(#[source] sqlx::Error),

    #[error("could not find first posts in threads of forum sub category")]
    CouldNotFindForumThreadsFirstPost(#[source] sqlx::Error),

    #[error("insufficient privileges")]
    InsufficientPrivileges,

    #[error("could not warn user: '{0}'")]
    CouldNotWarnUser(String),

    #[error("invalid user id or torrent id")]
    InvalidUserIdOrTorrentId,

    #[error("could not create wiki article")]
    CouldNotCreateWikiArticle(#[source] sqlx::Error),

    #[error("could not find wiki article")]
    CouldNotFindWikiArticle(#[source] sqlx::Error),

    #[error("could not create conversation")]
    CouldNotCreateConversation(#[source] sqlx::Error),

    #[error("could not create message")]
    CouldNotCreateConversationMessage(#[source] sqlx::Error),

    #[error("could not find conversation")]
    CouldNotFindConversation(#[source] sqlx::Error),

    #[error("could not find conversations")]
    CouldNotFindConversations(#[source] sqlx::Error),

    #[error("error getting musicbrainz data")]
    ErrorGettingMusicbrainzData(#[source] musicbrainz_rs::Error),

    #[error("invalid musicbrainz url")]
    InvalidMusicbrainzUrl,

    #[error("invalid comic vine url")]
    InvalidComicVineUrl,

    #[error("tmdb data fetching not available")]
    TMDBDataFetchingNotAvailable,

    #[error("invalid tmdb url")]
    InvalidTMDBUrl,
}

pub type Result<T> = std::result::Result<T, Error>;

impl actix_web::ResponseError for Error {
    #[inline]
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            // 400 Bad Request
            Error::BadRequest(_)
            | Error::UsernameAlreadyExists
            | Error::InvitationKeyInvalid
            | Error::InvitationKeyRequired
            | Error::InvitationKeyAlreadyUsed
            | Error::WrongUsernameOrPassword
            | Error::TorrentFileInvalid
            | Error::InvalidUserIdOrTorrentId => StatusCode::BAD_REQUEST,

            // 401 Unauthorized
            Error::InvalidOrExpiredRefreshToken => StatusCode::UNAUTHORIZED,

            // 403 Forbidden
            Error::AccountBanned | Error::InsufficientPrivileges => StatusCode::FORBIDDEN,

            // 404 Not Found
            Error::UserNotFound(_)
            | Error::UserWithIdNotFound(_)
            | Error::SeriesWithIdNotFound(_)
            | Error::DottorrentFileNotFound => StatusCode::NOT_FOUND,

            // 409 Conflict
            Error::NoInvitationsAvailable
            | Error::NotEnoughBonusPointsAvailable
            | Error::NotEnoughFreeleechTokensAvailable
            | Error::TorrentRequestAlreadyFilled
            | Error::TorrentTitleGroupNotMatchingRequestedOne
            | Error::InsufficientBonusPointsForBounty
            | Error::InsufficientUploadForBounty => StatusCode::CONFLICT,

            // 500 Internal Server Error
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        log::error!("The request generated this error: {self}");
        actix_web::HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": format!("{self}"),
        }))
    }
}
