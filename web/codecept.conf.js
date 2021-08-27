require("ts-node/register");

// eslint-disable-next-line @typescript-eslint/no-var-requires
const { setHeadlessWhen } = require("@codeceptjs/configure");
// eslint-disable-next-line @typescript-eslint/no-var-requires
const { config: dotenv } = require("dotenv");

dotenv({ path: ".env.local" });
dotenv();

setHeadlessWhen(process.env.HEADLESS);

exports.config = {
  tests: "./tests/**_test.ts",
  output: "./output",
  helpers: {
    Playwright: {
      url: process.env.COBASE_TEST_WEB_APP_URL,
      show: true,
      browser: "chromium",
      emulate: { locale: "en-US" },
    },
    Custom: {
      require: "./tests/custom_helper.ts",
    },
  },
  bootstrap: null,
  mocha: {},
  name: "tests",
  plugins: {
    pauseOnFail: {},
    retryFailedStep: {
      enabled: true,
    },
    tryTo: {
      enabled: true,
    },
    screenshotOnFail: {
      enabled: true,
    },
  },
};
