use actix_web::web;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::constants;
use crate::pattern::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Run query using Diesel to find the pattern by uid and return the public
/// facing pattern information.
pub fn find_pattern_info_by_uid(
    uid: Uuid,
    conn: &SqliteConnection,
) -> Result<Option<models::Pattern>, DbError> {
    use crate::schema::patterns::dsl::*;

    let pattern = patterns
        .filter(id.eq(uid.to_string()))
        .first::<models::Pattern>(conn)
        .optional()?
        .unwrap();

    Ok(Some(pattern))
}

pub fn insert_new_pattern(
    form: &web::Json<models::NewPattern>,
    conn: &SqliteConnection
) -> Result<Option<models::Pattern>, DbError> {
    use crate::schema::patterns::dsl::*;

    let utc = Utc::now();
    let timestamp = utc.timestamp();

    let pattern = models::Pattern {
        id: Uuid::new_v4().to_string(),
        user_id: form.user_id.to_owned(),
        name: form.name.to_owned(),
        homepage_url: generate_homepage_url(&form.name),
        highlight_image_url: Some("https://randomuser.me/api/portraits/thumb/men/94.jpg".to_string()),
        times_visited_in_24_hours: 0,
        num_favorites: 0,
        created_at: NaiveDateTime::from_timestamp(timestamp, 0),
    };

    diesel::insert_into(patterns).values(&pattern).execute(conn)?;

    Ok(Some(pattern))
}

pub fn find_hottest_patterns(conn: &SqliteConnection) -> Result<Option<Vec<models::PatternWithUserInfo>>, DbError> {
    use crate::schema::patterns::dsl::*;
    use crate::user::actions::find_user_info_by_uid;

    let hottest_patterns = patterns.select(
        (
            id,
            user_id,
            name,
            homepage_url,
            highlight_image_url,
            created_at,
            num_favorites,
            times_visited_in_24_hours,
        )
    )
    .order_by(times_visited_in_24_hours.desc())
    .limit(20)
    .load::<models::Pattern>(conn)?;

    let mut hottest_pattern_with_user = vec![];
    
    for hot_pattern in hottest_patterns.iter() {
        let user_uid = Uuid::parse_str(&hot_pattern.user_id)?;
        let user = find_user_info_by_uid(user_uid, &conn)?.unwrap();

        hottest_pattern_with_user.push(models::PatternWithUserInfo {
            user_first_name: user.first_name,
            user_last_name: user.last_name,
            name: hot_pattern.name.to_owned(),
            homepage_url: hot_pattern.homepage_url.to_owned(),
            highlight_image_url: hot_pattern.highlight_image_url.to_owned(),
            num_favorites: hot_pattern.num_favorites.to_owned(),
            times_visited_in_24_hours: hot_pattern.times_visited_in_24_hours,
        });
    }

    Ok(Some(hottest_pattern_with_user))
}

pub fn find_newest_patterns(conn: &SqliteConnection) -> Result<Option<Vec<models::PatternWithUserInfo>>, DbError> {
    use crate::schema::patterns::dsl::*;
    use crate::user::actions::find_user_info_by_uid;

    let newest_pattern = patterns.select(
        (
            id,
            user_id,
            name,
            homepage_url,
            highlight_image_url,
            created_at,
            num_favorites,
            times_visited_in_24_hours,
        )
    )
    .order_by(created_at.desc())
    .limit(4)
    .load::<models::Pattern>(conn)?;

    let mut newest_pattern_with_user = vec![];
    
    for new_pattern in newest_pattern.iter() {
        let user_uid = Uuid::parse_str(&new_pattern.user_id)?;
        let user = find_user_info_by_uid(user_uid, &conn)?.unwrap();

        newest_pattern_with_user.push(models::PatternWithUserInfo {
            user_first_name: user.first_name,
            user_last_name: user.last_name,
            name: new_pattern.name.to_owned(),
            homepage_url: new_pattern.homepage_url.to_owned(),
            highlight_image_url: new_pattern.highlight_image_url.to_owned(),
            num_favorites: new_pattern.num_favorites.to_owned(),
            times_visited_in_24_hours: new_pattern.times_visited_in_24_hours,
        });
    }

    Ok(Some(newest_pattern_with_user))
}

pub fn find_newest_first_patterns(conn: &SqliteConnection) -> Result<Option<Vec<models::PatternWithUserInfo>>, DbError> {
    // Filter patterns by those from users with only a single post
    //      Find user of pattern
    //      Find all patterns by user ID
    //      Get Count
    use crate::schema::patterns::dsl::*;
    use crate::schema::users::dsl::users;

    use crate::user;

    let newest_patterns = patterns.select(
        (
            id,
            user_id,
            name,
            homepage_url,
            highlight_image_url,
            created_at,
            num_favorites,
            times_visited_in_24_hours,
        )
    )
    .order_by(created_at.desc())
    .load::<models::Pattern>(conn)?;

    let mut newest_first_patterns = vec![];

    for newest_pattern in newest_patterns.iter() {
        let user_uid = newest_pattern.user_id.to_owned();
        let user = users.find(user_uid).get_result::<user::models::User>(conn)?;
        let user_patterns = models::Pattern::belonging_to(&user).load::<models::Pattern>(conn)?;
        
        if user_patterns.len() == 1 {
            newest_first_patterns.push(models::PatternWithUserInfo {
                user_first_name: user.first_name,
                user_last_name: user.last_name,
                name: newest_pattern.name.to_owned(),
                homepage_url: newest_pattern.homepage_url.to_owned(),
                highlight_image_url: newest_pattern.highlight_image_url.to_owned(),
                num_favorites: newest_pattern.num_favorites.to_owned(),
                times_visited_in_24_hours: newest_pattern.times_visited_in_24_hours,
            });
        }

        if newest_first_patterns.len() == 3 {
            break;
        }
    }

    Ok(Some(newest_first_patterns))
}

pub fn get_favorite_count(uid: Uuid, conn: &SqliteConnection) -> Result<i32, DbError> {
    use crate::schema::favorites::dsl::*;

    use crate::favorite;

    let favorites_count = favorites
        .filter(pattern_id.eq(uid.to_string()))
        .load::<favorite::models::Favorite>(conn)
        .unwrap()
        .len();


    Ok(favorites_count as i32)
}

pub fn update_pattern_favorite_count(uid: Uuid, conn: &SqliteConnection) -> Result<(), DbError> {
    use crate::schema::patterns::dsl::*;

    let favorites_count = get_favorite_count(uid, conn)?;

    diesel::update(patterns.filter(id.eq(uid.to_string())))
        .set(num_favorites.eq(favorites_count))
        .execute(conn)?;

    Ok(())
}

use crate::utils::sanitize_string;

pub fn generate_homepage_url(name: &str) -> String {
    let homepage_uid = Uuid::new_v4().to_string();
    let name_with_dashs = str::replace(&name, " ", "-");
    format!("{}{}-{}", constants::page::URL, sanitize_string(name_with_dashs), homepage_uid)
}