//免费翻译API
import { get } from './core'

const supportedLanguages = [
  ["auto", "AUTO"],
  ["zh2en", "ZH_CN2EN"],
  ["zh2jp", "ZH_CN2JP"],
];

const langMap = new Map<string, string>(supportedLanguages.map(lang => [lang[0], lang[1]]));

async function youdao(query: string, type: string): Promise<string> {
  const sourceLanguage = langMap.get(type);

  const source_lang = sourceLanguage || 'AUTO';
  const translate_text = query || '';
  const url = `http://fanyi.youdao.com/translate?doctype=json&type=${source_lang}&i=${translate_text}`;
  const response = await get(url);
  //@ts-ignore
  return response.data.translateResult.map(s => s[0].tgt).join('');
}

export {
  youdao,
}