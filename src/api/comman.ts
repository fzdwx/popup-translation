import { getClient, Body } from '@tauri-apps/api/http';

const AGENT = "User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36";

const postJson = async (url: string, data: string) => {
  const client = await getClient();
  return client.post<Object>(url, Body.text(data), {
    headers: {
      'Content-Type': 'application/json',
      'User-Agent': AGENT,
    },
  });
};

const get = async (url: string) => {
  const client = await getClient();
  return client.get<Object>(url, {
    headers: {
      'User-Agent': AGENT,
    },
  });
};

export { postJson, get };
