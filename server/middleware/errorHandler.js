const { AppError } = require('../utils/errors');
const { errorResponse } = require('../utils/response');

/**
 * 全局错误处理中间件
 */
function errorHandler(err, req, res, next) {
  // 记录错误日志
  console.error('Error occurred:', {
    code: err.code || 'UNKNOWN',
    message: err.message,
    stack: err.stack,
    path: req.path,
    method: req.method,
    body: req.body,
    timestamp: new Date().toISOString()
  });

  // 处理自定义错误
  if (err instanceof AppError) {
    return res.status(err.statusCode).json(
      errorResponse(err.code, err.message)
    );
  }

  // 处理 JSON 解析错误
  if (err instanceof SyntaxError && err.status === 400 && 'body' in err) {
    return res.status(400).json(
      errorResponse('INVALID_JSON', 'JSON 格式错误')
    );
  }

  // 处理未知错误
  return res.status(500).json(
    errorResponse('INTERNAL_ERROR', '服务器内部错误，请稍后重试')
  );
}

/**
 * 404 处理中间件
 */
function notFoundHandler(req, res, next) {
  const error = new Error(`路径 ${req.originalUrl} 不存在`);
  error.statusCode = 404;
  error.code = 'NOT_FOUND';
  next(error);
}

module.exports = {
  errorHandler,
  notFoundHandler
};
