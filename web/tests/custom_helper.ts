import { Helper } from "codeceptjs";

class Custom extends Helper {
  async amOnPageWitoutWait(url: string): Promise<void> {
    const { page, options } = this.helpers.Playwright;

    if (!/^\w+\:\/\//.test(url)) {
      url = options.url + url;
    }

    await page.goto(url);
  }
}

export = Custom;
