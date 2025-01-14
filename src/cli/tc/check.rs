use anyhow::Result;

use super::helpers::{display_schema, log_endpoint_set};
use crate::cli::fmt::Fmt;
use crate::core::blueprint::Blueprint;
use crate::core::config::reader::ConfigReader;
use crate::core::runtime::TargetRuntime;
use crate::core::Errata;

pub(super) struct CheckParams {
    pub(super) file_paths: Vec<String>,
    pub(super) n_plus_one_queries: bool,
    pub(super) schema: bool,
    pub(super) runtime: TargetRuntime,
}

pub(super) async fn check_command(params: CheckParams, config_reader: &ConfigReader) -> Result<()> {
    let CheckParams { file_paths, n_plus_one_queries, schema, runtime } = params;

    let config_module = (config_reader.read_all(&file_paths)).await?;
    log_endpoint_set(&config_module.extensions().endpoint_set);
    let blueprint = Blueprint::try_from(&config_module).map_err(Errata::from);

    match blueprint {
        Ok(blueprint) => {
            tracing::info!("Config {} ... ok", file_paths.join(", "));
            Fmt::log_n_plus_one(n_plus_one_queries, config_module.config());
            // Check the endpoints' schema
            let _ = config_module
                .extensions()
                .endpoint_set
                .clone()
                .into_checked(&blueprint, runtime)
                .await?;
            if schema {
                display_schema(&blueprint);
            }

            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
