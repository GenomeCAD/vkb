//! Tools to request catalog

/* std use */

/* crate use */

/* module declaration */

/* project use */
use crate::error;

pub async fn request_sessions(
    catalog: std::sync::Arc<dyn iceberg_rust::catalog::Catalog>,
    namespace: String,
    tables_names: &[&str],
) -> error::Result<datafusion::execution::context::SessionContext> {
    let ctx = datafusion::execution::context::SessionContext::new();

    for table_name in tables_names {
        let table_id = iceberg_rust::catalog::identifier::Identifier::new(
            &[namespace.to_string()],
            table_name,
        );

        let table = match catalog.clone().load_tabular(&table_id).await? {
            iceberg_rust::catalog::tabular::Tabular::Table(t) => t,
            _ => anyhow::bail!("View or MaterializeView are not support"),
        };

        let datafusion_table =
            std::sync::Arc::new(datafusion_iceberg::DataFusionTable::from(table));

        ctx.register_table(*table_name, datafusion_table)?;
    }

    Ok(ctx)
}

#[rocket::get("/")]
async fn info() -> rocket::serde::json::Value {
    rocket::serde::json::json!({ "status": "ok" })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("vkb_beacon_request", |rocket| async {
        rocket.mount("/", rocket::routes![info])
    })
}
