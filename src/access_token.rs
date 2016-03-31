use diesel::*;
use diesel::pg::PgConnection;
use diesel::pg::data_types::PgTimestamp;
use num::BigInt;

use schema::access_tokens;

#[derive(Debug, Queryable)]
pub struct AccessToken {
    pub id: BigInt,
    pub user_id: String,
    pub value: String,
    pub created_at: PgTimestamp,
}

#[derive(Debug)]
#[insertable_into(access_tokens)]
pub struct NewAccessToken {
    pub user_id: String,
    pub value: String,
}

pub fn create_access_token<'a>(
    connection: &'a PgConnection,
    user_id: &'a str,
) -> QueryResult<AccessToken> {
    let new_access_token = NewAccessToken {
        user_id: user_id.to_owned(),
        value: "fake access token".to_owned(),
    };

    insert(new_access_token)
        .into(access_tokens::table)
        .get_result(connection)
}

