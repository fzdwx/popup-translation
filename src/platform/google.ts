import { get } from './core'

const supportedLanguages = [
  ["auto", "auto"],
  ["english", "en-US"],
  ["chinese", "zh-CN"],
];

const langMap = new Map<string, string>(supportedLanguages.map(lang => [lang[0], lang[1]]));

async function google(text: string, from: string, to: string) {
  const sl = langMap.get(from) || "auto";
  const tl = langMap.get(to) || "zh-CN";
  const url = new URL('https://translate.googleapis.com/translate_a/single');

  url.search = (new URLSearchParams({
    client: 'gtx',
    sl: sl,
    tl: tl,
    dt: 't',
    dj: '1',
    q: text,
  })).toString();
  const response = await get(url.toString());

  //@ts-ignore
  const val = response.data.sentences.map((s) => s.trans).join('');
  return val
}

export { google };