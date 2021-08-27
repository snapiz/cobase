import { config as dotenv } from "dotenv";
import https from "https";
import KcAdminClient from "keycloak-admin";
import WebSocket from "ws";
import axios, { AxiosRequestConfig, AxiosInstance } from "axios";

dotenv({ path: ".env.local" });
dotenv();

interface Message<D = unknown> {
  topic: string;
  eventType: string;
  data: D;
}

export async function authKcAdminClient(
  kcAdminClient: KcAdminClient,
  username: string,
  password: string
): Promise<AxiosInstance> {
  kcAdminClient.setConfig({
    baseUrl: process.env.COBASE_TEST_SSO_URL,
    realmName: process.env.COBASE_TEST_SSO_REALM,
    requestConfig: {
      httpsAgent: new https.Agent({
        rejectUnauthorized: false,
      }),
    },
  });

  await kcAdminClient.auth({
    username,
    password,
    grantType: "password",
    clientId: process.env.COBASE_TEST_SSO_CLIENT_ID,
  });

  const client = axios.create({
    baseURL: process.env.COBASE_TEST_API_URL,
    httpsAgent: new https.Agent({
      rejectUnauthorized: false,
    }),
  });

  client.interceptors.request.use((config: AxiosRequestConfig) => {
    config.headers.Authorization = `Bearer ${kcAdminClient.accessToken}`;

    return config;
  });

  return client;
}

export function subscribe<D = unknown>(
  kcAdminClient: KcAdminClient,
  topic: string
): (
  predicate: (e: Message<D>) => boolean,
  timeoutMs?: number
) => Promise<Message<D>> {
  const messages: Message<D>[] = [];

  const ws = new WebSocket(process.env.COBASE_TEST_WS_URL, null, {
    rejectUnauthorized: false,
  });

  ws.onopen = () => {
    const data = JSON.stringify({
      name: "subscribe",
      data: { topic, token: kcAdminClient.accessToken },
    });

    ws.send(data);
  };

  ws.onmessage = (event) => {
    messages.push(JSON.parse(event.data));
  };

  return (
    predicate: (e: Message<D>) => boolean,
    timeoutMs = 5000
  ): Promise<Message<D>> => {
    return new Promise((resolve, reject) => {
      let message = null;
      let timeout = null;

      const interval = setInterval(() => {
        message = messages.find(predicate);

        if (message) {
          ws.close();
          clearInterval(interval);
          clearTimeout(timeout);
          resolve(message);
        }
      }, 200);

      timeout = setTimeout(() => {
        clearInterval(interval);

        if (message) {
          return;
        }

        ws.close();
        reject(`timeout ${timeoutMs}ms`);
      }, timeoutMs);
    });
  };
}
