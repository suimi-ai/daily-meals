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

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["success"].as_bool().unwrap());
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

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["success"].as_bool().unwrap());
    }

    #[actix_web::test]
    async fn test_recipe_get() {
        let config = Arc::new(Config::from_env());
        let app = test::init_service(
            App::new().configure(|cfg| configure(cfg, config))
        ).await;

        // 使用 URL 编码的中文
        let req = test::TestRequest::get()
            .uri("/api/recipe/%E7%BA%A2%E7%83%A7%E8%82%89")  // 红烧肉的 URL 编码
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["success"].as_bool().unwrap());
    }

    #[actix_web::test]
    async fn test_menu_generate() {
        let config = Arc::new(Config::from_env());
        let app = test::init_service(
            App::new().configure(|cfg| configure(cfg, config))
        ).await;

        let req = test::TestRequest::post()
            .uri("/api/menu/generate")
            .set_json(&serde_json::json!({
                "mealType": "午餐",
                "servings": 2
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["success"].as_bool().unwrap());
    }

    #[actix_web::test]
    async fn test_shopping_generate() {
        let config = Arc::new(Config::from_env());
        let app = test::init_service(
            App::new().configure(|cfg| configure(cfg, config))
        ).await;

        let req = test::TestRequest::post()
            .uri("/api/shopping/generate")
            .set_json(&serde_json::json!({
                "dishes": [{"name": "红烧肉"}],
                "servings": 2
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["success"].as_bool().unwrap());
    }

    #[actix_web::test]
    async fn test_recipe_not_found() {
        let config = Arc::new(Config::from_env());
        let app = test::init_service(
            App::new().configure(|cfg| configure(cfg, config))
        ).await;

        // 使用 URL 编码的中文
        let req = test::TestRequest::get()
            .uri("/api/recipe/%E4%B8%8D%E5%AD%98%E5%9C%A8%E7%9A%84%E8%8F%9C")  // 不存在的菜的 URL 编码
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }
}
