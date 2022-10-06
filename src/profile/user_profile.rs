use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserInfo {
    stats: Stats,
    user: User,
}

#[derive(Debug, Serialize)]
pub struct User {
    avatar_larger: String,
    avatar_medium: String,
    avatar_thumb: String,
    bio_link: Option<BioLink>,
    ftc: bool,
    id: String,
    is_ad_virtual: bool,
    is_under_age_18: bool,
    nickname_modify_time: u64, //unix timestamp
    nickname: String,
    open_favorite: bool,
    private_account: bool,
    profile_tab: ProfileTab,
    relation: u32,
    sec_uid: String,
    secret: bool,
    signature: String,
    tt_seller: bool,
    unique_id: String,
    unique_id_modify_time: u64,
    verified: bool,
}

#[derive(Debug, Serialize)]
pub struct ProfileTab {
    show_play_list_tab: bool,
}

#[derive(Debug, Serialize)]
pub struct BioLink {
    link: String,
    risk: String,
}

#[derive(Debug, Serialize)]
pub struct Stats {
    digg_count: u64,
    follower_count: u64,
    following_count: u64,
    heart: u64,
    heart_count: u64,
    video_count: u32,
}

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    user_info: UserInfo,
}
