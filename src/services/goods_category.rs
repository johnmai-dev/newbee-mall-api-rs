use crate::bootstrap::database::PooledConn;
use crate::bootstrap::result;
use crate::models::goods_category::{
    GoodsCategory, GoodsCategoryFilter, SecondGoodsCategory, ThirdGoodsCategory,
};
use crate::models::pagination::Paginator;

pub fn collect(
    conn: &mut PooledConn,
) -> result::Result<
    Vec<(
        GoodsCategory,
        Vec<(SecondGoodsCategory, Vec<ThirdGoodsCategory>)>,
    )>,
> {
    let categories = GoodsCategory::collect(conn)?;

    Ok(categories)
}

pub fn list(
    conn: &mut PooledConn,
    filter: GoodsCategoryFilter,
) -> result::Result<Paginator<GoodsCategory>> {
    Ok(GoodsCategory::list(conn, filter)?)
}
