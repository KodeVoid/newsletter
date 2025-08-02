use crate::handlers::health_check::health_check;
use actix_web::{
    guard::{Guard, GuardContext, Post},
    web, Route,
};

struct HealthGuard;

impl Guard for HealthGuard {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        ctx.head()
            .headers()
            .get("X-Route-Type")
            .and_then(|h| h.to_str().ok())
            .map_or(false, |token| token == "health")
    }
}

pub fn init(configuration: &mut web::ServiceConfig) {
    configuration.route(
        "/health", // <--  this must start with a slash
        Route::new()
            .guard(Post())
            .guard(HealthGuard)
            .to(health_check),
    );
}
