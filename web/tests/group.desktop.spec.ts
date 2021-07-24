import { Page } from "playwright";
import faker from "faker";

export default async (page: Page): Promise<void> => {
  try {
    const title = faker.name.title();
    const formatted = title.substr(0, 15);

    await page.fill("input#username", "john");
    await page.fill("input#password", "timada123!");
    await page.click('"Sign In"');

    await page.click('"Contacts"');
    await page.waitForSelector('"Group name"');
    await page.fill("input[name=name]", formatted);
    await page.click("button >> text=Create");
    await page.waitForSelector(`span >> "${formatted}"`);
  } catch (error) {
    await page.screenshot({ path: `tests/__screenshots__/group_desktop.png` });
    throw error.message;
  }
};
