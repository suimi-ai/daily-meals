/**
 * 统一响应格式化工具
 */

/**
 * 成功响应
 */
function successResponse(data, message = 'Success') {
  return {
    success: true,
    data,
    message
  };
}

/**
 * 错误响应
 */
function errorResponse(code, message, details = null) {
  const response = {
    success: false,
    error: {
      code,
      message,
      timestamp: new Date().toISOString()
    }
  };

  if (details) {
    response.error.details = details;
  }

  return response;
}

/**
 * 分页响应
 */
function paginatedResponse(data, pagination, message = 'Success') {
  return {
    success: true,
    data,
    pagination: {
      page: pagination.page || 1,
      pageSize: pagination.pageSize || 20,
      total: pagination.total || 0,
      totalPages: pagination.totalPages || 0
    },
    message
  };
}

module.exports = {
  successResponse,
  errorResponse,
  paginatedResponse
};
