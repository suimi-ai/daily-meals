const express = require('express');
const cors = require('cors');
require('dotenv').config();

const app = express();
const PORT = process.env.PORT || 3000;

// 中间件
app.use(cors());
app.use(express.json());

// 路由
app.use('/api/menu', require('./routes/menu'));
app.use('/api/shopping', require('./routes/shopping'));
app.use('/api/recipe', require('./routes/recipe'));

// 健康检查
app.get('/health', (req, res) => {
  res.json({ status: 'ok', message: '一日三餐服务运行正常' });
});

app.listen(PORT, () => {
  console.log(`🚀 服务器运行在 http://localhost:${PORT}`);
});
