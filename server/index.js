const express = require('express');
const cors = require('cors');
require('dotenv').config();

const { errorHandler, notFoundHandler } = require('./middleware/errorHandler');
const { setupSwagger } = require('./config/swagger');

const app = express();
const PORT = process.env.PORT || 3000;

// 中间件
app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// 请求日志
app.use((req, res, next) => {
  console.log(`${new Date().toISOString()} ${req.method} ${req.path}`);
  next();
});

// Swagger UI
setupSwagger(app);

// 路由
app.use('/api/menu', require('./routes/menu'));
app.use('/api/shopping', require('./routes/shopping'));
app.use('/api/recipe', require('./routes/recipe'));

// 健康检查
app.get('/health', (req, res) => {
  res.json({ 
    success: true,
    data: {
      status: 'ok', 
      message: '一日三餐服务运行正常',
      version: '1.0.0',
      timestamp: new Date().toISOString()
    }
  });
});

// API 信息
app.get('/', (req, res) => {
  res.json({
    success: true,
    data: {
      name: '一日三餐 API',
      version: '1.0.0',
      description: 'AI驱动的智能餐饮规划助手',
      documentation: '/api-docs',
      endpoints: {
        menu: '/api/menu',
        shopping: '/api/shopping',
        recipe: '/api/recipe',
        health: '/health'
      }
    }
  });
});

// 404 处理
app.use(notFoundHandler);

// 错误处理
app.use(errorHandler);

// 启动服务器
app.listen(PORT, () => {
  console.log(`🚀 服务器运行在 http://localhost:${PORT}`);
  console.log(`📚 API 文档: http://localhost:${PORT}/api-docs`);
  console.log(`🔧 环境: ${process.env.NODE_ENV || 'development'}`);
});

module.exports = app;
