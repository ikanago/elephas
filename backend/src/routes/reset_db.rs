use actix_web::{delete, web, HttpResponse, Responder};
use sqlx::PgPool;

struct Table {
    table_name: Option<String>,
}

#[delete("/reset-db")]
#[tracing::instrument(skip(pool))]
pub async fn reset_db(pool: web::Data<PgPool>) -> crate::Result<impl Responder> {
    let tables = sqlx::query_as!(
        Table,
        r#"
        SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = 'public'
            AND table_type = 'BASE TABLE'
            AND table_name NOT LIKE '\_%' ESCAPE '\';"#
    )
    .fetch_all(pool.as_ref())
    .await?;

    for table in tables {
        if let Some(table_name) = table.table_name {
            // To wait for the query to complete, bind the result to `_`.
            let _ = sqlx::query(format!("DELETE FROM {} CASCADE;", table_name).as_str())
                .execute(pool.as_ref())
                .await?;
        }
    }

    Ok(HttpResponse::Ok().finish())
}
