# 一日三餐 🍳

[![GitHub stars](https://img.shields.io/github/stars/suimi-ai/daily-meals?style=social)](https://github.com/suimi-ai/daily-meals)
[![GitHub license](https://img.shields.io/github/license/suimi-ai/daily-meals)](https://github.com/suimi-ai/daily-meals)

> AI驱动的智能餐饮规划助手 - 让每一餐都充满期待

## 📱 项目简介

**一日三餐**是一款基于AI的智能餐饮规划小程序，帮助用户：
- 🍽️ 根据偏好生成个性化菜单
- 🛒 智能生成购物清单
- 👨‍🍳 提供详细烹饪指导

## ✨ 核心功能

### 1. 智能菜单生成 📋
- 支持**早餐/午餐/晚餐**三种用餐类型
- 根据人数、口味偏好、饮食限制智能推荐
- 展示菜品营养信息（卡路里、蛋白质等）

### 2. 智能购物清单 🛒
- 自动提取所需食材
- 智能调整分量（根据用餐人数）
- 标记已有食材，避免重复购买
- 按分类展示（肉类/蔬菜/调料等）
- 一键复制清单

### 3. 烹饪指导 👨‍🍳
- 详细的步骤说明
- 时间预估
- 难度评级
- 烹饪小贴士
- 步骤导航（上一步/下一步）

## 🛠️ 技术栈

### 后端
- **Node.js** - 服务端运行环境
- **Express** - Web框架
- **AI API** - 支持GLM/OpenAI/Claude

### 前端
- **微信小程序** - 移动端应用
- **自定义UI** - 原生组件开发

## 🚀 快速开始

### 后端启动

```bash
# 进入服务端目录
cd server

# 安装依赖
npm install

# 配置环境变量
cp .env.example .env
# 编辑.env文件，填入AI API密钥

# 启动服务
npm run dev
```

### 小程序启动

1. 下载并安装[微信开发者工具](https://developers.weixin.qq.com/miniprogram/dev/devtools/download.html)
2. 导入项目：选择 `miniprogram` 目录
3. 修改API地址：编辑 `miniprogram/utils/api.js` 中的 `apiBaseUrl`
4. 编译运行

## 📖 文档

- [项目架构](./ARCHITECTURE.md) - 详细的技术架构说明
- [API文档](./docs/API.md) - 接口文档（待完善）
- [开发指南](./docs/DEVELOPMENT.md) - 开发规范和指南（待完善）

## 📸 功能截图

（待补充）

## 🗺️ 开发路线

### v1.0（当前）
- [x] 基础框架搭建
- [x] 菜单生成功能
- [x] 购物清单功能
- [x] 菜谱查询功能

### v1.1（计划中）
- [ ] 用户偏好设置
- [ ] 食材库存管理
- [ ] 分享功能

### v2.0（未来）
- [ ] 用户系统
- [ ] 社区功能
- [ ] 多平台支持

## 🤝 贡献指南

欢迎提交Issue和Pull Request！

1. Fork本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 提交Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 📞 联系方式

- GitHub: [@suimi-ai](https://github.com/suimi-ai)
- 项目地址: [https://github.com/suimi-ai/daily-meals](https://github.com/suimi-ai/daily-meals)

## 🙏 致谢

感谢所有AI技术提供商，让智能餐饮规划成为可能！

---

⭐ 如果这个项目对你有帮助，请给一个Star支持一下！
