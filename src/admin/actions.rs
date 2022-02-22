use bcrypt::{DEFAULT_COST, hash};
use diesel::prelude::*;
use fake::Fake;
use rand::Rng;
use std::collections::HashSet;
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Populates the database with users.
fn populate_users(conn: &SqliteConnection) -> Result<Option<Vec<String>>, DbError> {
    use fake::faker::name::raw::*;
    use fake::faker::internet::raw::*;
    use fake::locales::*;

    use crate::user;
    use crate::schema::users::dsl::*;

    let num_of_users = 20;
    let mut user_ids = vec![];

    for n in 1..=num_of_users {
        println!("Inserting user: {}/{}", n, num_of_users);
        let user_id = Uuid::new_v4().to_string();
        let pw: String = Password(EN, 12..32).fake();
        let hash_pw = hash(pw, DEFAULT_COST)?;

        let new_user = user::models::User {
            id: user_id.to_owned(),
            first_name: FirstName(EN).fake(),
            last_name: LastName(EN).fake(),
            username: Username(EN).fake(),
            password: hash_pw,
        };
        
        diesel::insert_into(users).values(&new_user).execute(conn)?;
        user_ids.push(user_id);
        println!("Completed user: {}/{}", n, num_of_users);
    }

    Ok(Some(user_ids))
}

/// Populates the database with patterns. A random number of patterns are
/// generated per user.
fn populate_patterns(
    conn: &SqliteConnection,
    user_ids: &Vec<String>
) -> Result<Option<Vec<Uuid>>, DbError> {
    use fake::faker::lorem::raw::*;
    use fake::faker::chrono::raw::*;
    use fake::locales::*;
    use rand::prelude::*;

    use crate::pattern;
    use crate::schema::patterns::dsl::*;

    let mut rng = rand::thread_rng();
    let num_of_patterns = 50;
    let mut pattern_ids = vec![];
    let user_ids_len = user_ids.len();

    for n in 1..=num_of_patterns {
        println!("Inserting pattern: {}/{}", n, num_of_patterns);
        let random_index = rng.gen_range(0..user_ids_len);
        let random_num_of_visits = rng.gen_range(0..500);
        let random_user_id = &user_ids[random_index];
        let pattern_id = Uuid::new_v4();
        let title: String = Sentence(EN, 5..11).fake();

        let new_pattern = pattern::models::Pattern {
            id: pattern_id.to_string().to_owned(),
            user_id: random_user_id.to_string(),
            name: title.to_owned(),
            created_at: DateTime(EN).fake(), 
            homepage_url: pattern::actions::generate_homepage_url(&title),
            times_visited_in_24_hours: random_num_of_visits,
            num_favorites: 0,
            highlight_image_url: Some("https://randomuser.me/api/portraits/thumb/men/94.jpg".to_string()),
        };

        diesel::insert_into(patterns).values(&new_pattern).execute(conn)?;
        pattern_ids.push(pattern_id);
        println!("Completed pattern: {}/{}", n, num_of_patterns);
    }

    Ok(Some(pattern_ids))
}

/// Populates random number of favorites per user per pattern. Relationship
/// between user and favorites are unique (e.g., a user can only like a
/// specific pattern once).
fn populate_favorites(
    conn: &SqliteConnection,
    user_ids: &Vec<String>,
    pattern_ids: &Vec<Uuid>
) -> Result<(), DbError> {
    use crate::favorite;
    use crate::pattern;
    use crate::schema::favorites::dsl::*;

    let mut memoized_favorite = HashSet::new();
    let num_of_favorites = 350;
    let mut rng = rand::thread_rng();

    for n in 1..=num_of_favorites {
        println!("Inserting favorite: {}/{}", n, num_of_favorites);
        let mut random_user_id = user_ids[rng.gen_range(0..user_ids.len())].to_owned();
        let random_pattern_id = pattern_ids[rng.gen_range(0..pattern_ids.len())].to_owned();
        let mut memoized_key = random_user_id.to_owned();

        let mut random_pattern_id = random_pattern_id.to_string();

        memoized_key.push_str(&random_pattern_id.to_string());

        loop {
            if memoized_favorite.contains(&memoized_key) {
                random_user_id = user_ids[rng.gen_range(0..user_ids.len())].to_owned();
                random_pattern_id = pattern_ids[rng.gen_range(0..pattern_ids.len())].to_string().to_owned();
                memoized_key = random_user_id.to_owned();
                memoized_key.push_str(&random_pattern_id);
                continue;
            }

            break;
        }

        let new_favorite = favorite::models::Favorite {
            id: Uuid::new_v4().to_string(),
            pattern_id: random_pattern_id.clone(),
            user_id: random_user_id.to_owned(),
        };
        
        diesel::insert_into(favorites).values(&new_favorite).execute(conn)?;
        
        memoized_favorite.insert(memoized_key);
        println!("Completed favorite: {}/{}", n, num_of_favorites);
    }

    for pid in pattern_ids {
        pattern::actions::update_pattern_favorite_count(pid.clone(), conn)?;
    }

    Ok(())
}

/// Populates database
pub fn populate_database(
    conn: &SqliteConnection
) -> Result<(), DbError> {
    let user_ids = populate_users(&conn)?.unwrap();
    let pattern_ids = populate_patterns(&conn, &user_ids)?.unwrap();
    populate_favorites(&conn, &user_ids, &pattern_ids)
}