#![feature(custom_derive, plugin)]
#![plugin(diesel_codegen)]

#[macro_use] extern crate diesel;
extern crate num;

mod access_token;
mod schema;

fn main() {
}
