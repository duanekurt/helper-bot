use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    environment::Environment,
    task::Tasks,
    errors::Error,
    Result,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Settings {
    token: String,
}

#[allow(unused_imports)]
use crate::{controllers, tasks, workers::downloader::DownloadWorker};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn before_run(ctx: &AppContext) -> Result<()> {
        let raw = ctx
            .config
            .settings
            .as_ref()
            .and_then(|s| s.get("discord"))
            .cloned()
            .ok_or_else(|| Error::string("settings.discord missing"))?;

        let conf: Settings = serde_json::from_value(raw)?;

        tokio::spawn(async {
            if let Err(error) = crate::discord::start_bot(conf.token).await {
                tracing::error!("Discord bot stopped: {error:#}");
            }
        });

        Ok(())
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::home::routes())
    }
    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
    }
}
