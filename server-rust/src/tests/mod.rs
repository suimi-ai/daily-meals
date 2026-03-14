#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use crate::config::Config;
    use crate::routes::configure;
    use std::sync::Arc;

    #[actix_web::test]
    async fn test_health_endpoint() {
        let config = Arc::new(Config::from_env());
        let app = test::init_service(
            App::new().configure(|cfg| configure(cfg, config))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/health")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_menu_recommend() {
        let config = Arc::new(Config::from_env());
        let app = test::init_service(
            App::new().configure(|cfg| configure(cfg, config))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/menu/recommend")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_recipe_search() {
        let config = Arc::new(Config::from_env());
        let app = test::init_service(
            App::new().configure(|cfg| configure(cfg, config))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/recipe/search")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
