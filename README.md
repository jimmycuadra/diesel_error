This repository is a demonstration of the error reported in https://github.com/sgrif/diesel/issues/227.

Clone it to your machine and run `cargo build` (using nightly Rust) to see the error:

```
$ cargo build
   Compiling diesel_error v0.1.0 (file:///diesel_error)
diesel::query_builder::insert_statement::InsertStatement<schema::access_tokens::table, access_token::NewAccessToken>
src/access_token.rs:34:10: 34:20 error: no method named `get_result` found for type `diesel::query_builder::insert_statement::InsertStatement<schema::access_tokens::table, access_token::NewAccessToken>` in the current scope
src/access_token.rs:34         .get_result(connection)
                                ^~~~~~~~~~
src/access_token.rs:34:10: 34:20 note: the method `get_result` exists but the following trait bounds were not satisfied: `diesel::query_builder::insert_statement::InsertQuery<(schema::access_tokens::columns::id, schema::access_tokens::columns::user_id, schema::access_tokens::columns::value, schema::access_tokens::columns::created_at), diesel::query_builder::insert_statement::InsertStatement<schema::access_tokens::table, access_token::NewAccessToken>> : diesel::query_builder::QueryFragment<_>`, `&diesel::query_builder::insert_statement::InsertStatement<schema::access_tokens::table, access_token::NewAccessToken> : diesel::query_builder::AsQuery`, `_ : diesel::query_builder::QueryFragment<_>`, `&diesel::query_builder::insert_statement::InsertStatement<schema::access_tokens::table, access_token::NewAccessToken> : diesel::query_builder::Query`, `&diesel::query_builder::insert_statement::InsertStatement<schema::access_tokens::table, access_token::NewAccessToken> : diesel::query_builder::Query`, `diesel::query_builder::insert_statement::InsertStatement<schema::access_tokens::table, access_token::NewAccessToken> : diesel::query_builder::Query`, `diesel::query_builder::insert_statement::InsertStatement<schema::access_tokens::table, access_token::NewAccessToken> : diesel::query_builder::Query`, `&mut diesel::query_builder::insert_statement::InsertStatement<schema::access_tokens::table, access_token::NewAccessToken> : diesel::query_builder::AsQuery`, `_ : diesel::query_builder::QueryFragment<_>`, `&mut diesel::query_builder::insert_statement::InsertStatement<schema::access_tokens::table, access_token::NewAccessToken> : diesel::query_builder::Query`, `&mut diesel::query_builder::insert_statement::InsertStatement<schema::access_tokens::table, access_token::NewAccessToken> : diesel::query_builder::Query`, `&mut diesel::query_builder::insert_statement::InsertStatement<schema::access_tokens::table, access_token::NewAccessToken> : diesel::query_builder::Query`
error: aborting due to previous error
error: Could not compile `diesel_error`.

To learn more, run the command again with --verbose.
```
