const swaggerUi = require('swagger-ui-express');
const YAML = require('yamljs');
const path = require('path');

/**
 * Swagger 配置
 */
function setupSwagger(app) {
  // 仅在非生产环境启用 Swagger UI
  if (process.env.ENABLE_SWAGGER === 'true' || process.env.NODE_ENV !== 'production') {
    try {
      const swaggerDocument = YAML.load(path.join(__dirname, '../openapi/openapi.yaml'));
      
      // Swagger UI 选项
      const swaggerOptions = {
        customCss: '.swagger-ui .topbar { display: none }',
        customSiteTitle: '一日三餐 API 文档',
        swaggerOptions: {
          persistAuthorization: true,
          displayOperationId: false,
          filter: true
        }
      };

      // 挂载 Swagger UI
      app.use('/api-docs', swaggerUi.serve, swaggerUi.setup(swaggerDocument, swaggerOptions));
      
      console.log('📚 Swagger UI 已启用: http://localhost:3000/api-docs');
    } catch (error) {
      console.error('❌ 加载 Swagger 文档失败:', error.message);
    }
  }
}

module.exports = { setupSwagger };
