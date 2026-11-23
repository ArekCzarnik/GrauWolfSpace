//! Service binary of the generated agent: `/run`, `/run/stream`, `/health`, A2A and MCP
//! (feature `service`; settings from the environment, see `grauwolf::service`).

use std::sync::Arc;

fn main() -> std::process::ExitCode {
    grauwolf::service::main(
        reviewer::SPEC_JSON,
        Arc::new(|base_dir, options| {
            Box::pin(async move {
                reviewer::provision(&base_dir, options)
                    .await
                    .map_err(|error| error.to_string())
            })
        }),
        Arc::new(|agent, input, options| {
            Box::pin(async move {
                let input = serde_json::from_value(input).map_err(|error| error.to_string())?;
                let output = reviewer::run_provisioned(&agent, input, options)
                    .await
                    .map_err(|error| error.to_string())?;
                serde_json::to_value(output).map_err(|error| error.to_string())
            })
        }),
    )
}
