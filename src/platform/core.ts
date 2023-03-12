import { getClient, Body } from '@tauri-apps/api/http'

const supportedLanguages = [
  ["auto", "auto"],
  ["english", "EN"],
  ["chinese", "ZH"],
];

const langMap = new Map<string, string>(supportedLanguages.map(lang => [lang[0], lang[1]]));

const postJson = async (url: string, data: string) => {
  const client = await getClient()
  return client.post<Object>(url, Body.text(data), {
    headers: {
      'Content-Type': 'application/json',
      'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36'
    }
  })
}

export { supportedLanguages, langMap, postJson };