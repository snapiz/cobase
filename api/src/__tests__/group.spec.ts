import { AxiosInstance } from "axios";
import faker from "faker";
import KcAdminClient from "keycloak-admin";
import { authKcAdminClient, subscribe } from "./base";

const john = new KcAdminClient();
let johnApi: AxiosInstance = null;

beforeAll(async () => {
  johnApi = await authKcAdminClient(john, "john", "timada123!");
});

const createGroup = async (input) => {
  return await johnApi.post("groups/create", input);
};

test("create group success", async () => {
  const getEvent = subscribe<{ name: string }>(john, "group");
  const input = { name: faker.name.title() };
  const { data } = await createGroup(input);

  expect(data).toEqual({ success: true });

  const event = await getEvent((e) => e.eventType === "created");

  expect(event.data.name).toEqual(input.name);
});
