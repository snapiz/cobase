import { config as dotenv } from "dotenv";
import { program, Command } from "commander";

import {
  webkit,
  chromium,
  firefox,
  devices,
  BrowserContext,
  BrowserContextOptions,
} from "playwright";

import runGroupDesktop from "./group.desktop.spec";

interface ICommand extends Command {
  mobile?: string;
  tablet?: string;
  desktop?: string;
}

dotenv({ path: ".env.local" });
dotenv();

program
  .version("0.0.1")
  .option("-p, --mobile", "run tests on mobile")
  .option("-t, --tablet", "run tests on tablet");

const getContextOptions = (
  mobile?: string,
  tablet?: string
): BrowserContextOptions => {
  const opts = {
    locale: "en-US",
    timezoneId: "Europe/Paris",
    ignoreHTTPSErrors: true,
  };

  if (cli.mobile) {
    return Object.assign(opts, devices[mobile || "Pixel 2"]);
  }

  if (cli.tablet) {
    return Object.assign(opts, devices[tablet || "Kindle Fire HDX"]);
  }

  return opts;
};

program
  .command("chromium")
  .description("Run tests with chromium")
  .action(async () => {
    try {
      const browser = await chromium.launch({
        headless: !!process.env.COBASE_TEST_PLAYWRIGHT_HEADLESS,
      });
      const context = await browser.newContext(getContextOptions());

      await runTests(context);

      await browser.close();
    } catch (error) {
      console.error(error);
      process.exit(1);
    }
  });

program
  .command("firefox")
  .description("Run tests with firefox")
  .action(async () => {
    try {
      const browser = await firefox.launch({
        headless: !!process.env.COBASE_TEST_PLAYWRIGHT_HEADLESS,
      });
      const context = await browser.newContext(getContextOptions());

      await runTests(context);

      await browser.close();
    } catch (error) {
      console.error(error);
      process.exit(1);
    }
  });

program
  .command("webkit")
  .description("Run tests with webkit")
  .action(async () => {
    try {
      const browser = await webkit.launch({
        headless: !!process.env.COBASE_TEST_PLAYWRIGHT_HEADLESS,
      });
      const context = await browser.newContext(
        getContextOptions("iPhone 11 Pro", "iPad Pro 11")
      );

      await runTests(context);

      await browser.close();
    } catch (error) {
      console.error(error);
      process.exit(1);
    }
  });

const cli: ICommand = program.parse(process.argv);

const runTests = async (context: BrowserContext) => {
  const page = await context.newPage();
  page.setDefaultTimeout(5000);
  await page.goto(process.env.COBASE_TEST_WEB_APP_URL || "");

  if (cli.mobile) {
    //await group.mobile(page);
  } else if (cli.tablet) {
    //await group.tablet(page);
  } else {
    await runGroupDesktop(page);
  }
};
