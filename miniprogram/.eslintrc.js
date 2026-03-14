module.exports = {
  env: {
    browser: true,
    es2021: true
  },
  extends: 'eslint:recommended',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module'
  },
  globals: {
    wx: 'readonly',
    App: 'readonly',
    Page: 'readonly',
    getApp: 'readonly',
    getCurrentPages: 'readonly'
  },
  rules: {
    'no-unused-vars': 'warn',
    'no-console': 'off'
  }
};
