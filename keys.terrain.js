// can use `process.env.SECRET_MNEMONIC` or `process.env.SECRET_PRIV_KEY`
// to populate secret in CI environment instead of hardcoding
require('dotenv').config()

module.exports = {
  "bombay-12": {
    mnemonic: process.env.SECRET_MNEMONIC,
  }
};
