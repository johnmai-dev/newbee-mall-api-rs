use crate::bootstrap::database::PooledConn;
use crate::models::pagination::{Paginate, Paginator};
use crate::models::schema::tb_newbee_mall_carousel::{carousel_rank, dsl, is_deleted};
use crate::models::NOT_DELETE;
use chrono::NaiveDateTime;
use diesel::dsl::IntoBoxed;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Carousel {
    pub carousel_id: i32,
    pub carousel_url: String,
    pub redirect_url: String,
    pub carousel_rank: i32,
    pub is_deleted: i8,
    pub create_time: NaiveDateTime,
    pub create_user: i32,
    pub update_time: NaiveDateTime,
    pub update_user: i32,
}

impl Carousel {
    fn filter() -> IntoBoxed<'static, dsl::tb_newbee_mall_carousel, Mysql> {
        let query = dsl::tb_newbee_mall_carousel.into_boxed();
        query
            .filter(is_deleted.eq(NOT_DELETE))
            .order(carousel_rank.desc())
    }

    pub fn find(conn: &mut PooledConn, carousel_id: i32) -> QueryResult<Self> {
        dsl::tb_newbee_mall_carousel
            .find(carousel_id)
            .filter(is_deleted.eq(NOT_DELETE))
            .first(conn)
    }

    pub fn get_by_limit(conn: &mut PooledConn, limit: i64) -> QueryResult<Vec<Carousel>> {
        Self::filter().limit(limit).load::<Self>(conn)
    }

    pub fn list(
        conn: &mut PooledConn,
        page_number: Option<i64>,
        page_size: Option<i64>,
    ) -> QueryResult<Paginator<Self>> {
        Paginate::new(Self::filter, page_number)
            .per_page(page_size)
            .load_with_paginator(conn)
    }
}
