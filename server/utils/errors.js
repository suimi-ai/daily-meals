/**
 * 自定义错误类
 */
class AppError extends Error {
  constructor(code, message, statusCode = 500) {
    super(message);
    this.code = code;
    this.statusCode = statusCode;
    this.isOperational = true; // 标识这是可预期的错误

    Error.captureStackTrace(this, this.constructor);
  }
}

/**
 * 预定义错误类型
 */
class ValidationError extends AppError {
  constructor(message) {
    super('VALIDATION_ERROR', message, 400);
  }
}

class NotFoundError extends AppError {
  constructor(message = '资源不存在') {
    super('NOT_FOUND', message, 404);
  }
}

class AIServiceError extends AppError {
  constructor(message = 'AI服务调用失败') {
    super('AI_SERVICE_ERROR', message, 500);
  }
}

class InternalError extends AppError {
  constructor(message = '服务器内部错误') {
    super('INTERNAL_ERROR', message, 500);
  }
}

module.exports = {
  AppError,
  ValidationError,
  NotFoundError,
  AIServiceError,
  InternalError
};
