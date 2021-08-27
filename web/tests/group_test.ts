import faker from "faker";

Feature("group");

Scenario("test something", ({ I }) => {
  const title = faker.name.title();
  const formatted = title.substr(0, 15);

  I.amOnPageWitoutWait("/");

  I.see("TIMADA-DEV");
  I.fillField("username", "john");
  I.fillField("password", "timada123!");
  I.click("Sign In");

  I.click("Contacts");
  I.see("Group name");
  I.fillField("name", formatted);
  I.click("button >> text=Create");
  I.waitForElement(`span >> "${formatted}"`);
});

export {};
