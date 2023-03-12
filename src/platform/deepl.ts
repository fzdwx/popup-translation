// inspired by https://github.com/akl7777777/bob-plugin-akl-deepl-free-translate/blob/main/node_js_implementation/deepl.js
// MIT License

import { langMap, postJson } from './core'


function initData(source_lang: string, target_lang: string) {
  return {
    jsonrpc: '2.0',
    method: 'LMT_handle_texts',
    id: 1,
    params: {
      splitting: 'newlines',
      timestamp: 1,
      texts: [
        {

        }
      ],
      lang: {
        source_lang_user_selected: source_lang,
        target_lang: target_lang
      }
    }
  };
}

function getICount(translate_text: string) {
  return translate_text.split('i').length - 1;
}

function getRandomNumber() {
  const rand = Math.floor(Math.random() * 99999) + 100000;
  return rand * 1000;
}

function getTimeStamp(iCount: number) {
  const ts = Date.now();
  if (iCount !== 0) {
    iCount = iCount + 1;
    return ts - (ts % iCount) + iCount;
  } else {
    return ts;
  }
}

async function deepl(query: string, from: string, to: string): Promise<string> {
  const sourceLanguage = langMap.get(from);
  const targetLanguage = langMap.get(to);

  const source_lang = sourceLanguage || 'auto';
  const target_lang = targetLanguage || 'zh';
  const translate_text = query || '';
  const url = 'https://www2.deepl.com/jsonrpc';
  let id = getRandomNumber()
  const post_data = initData(source_lang, target_lang);
  const text = {
    text: translate_text,
    requestAlternatives: 3
  };
  post_data.id = id;
  post_data.params.texts = [text];
  post_data.params.timestamp = getTimeStamp(getICount(translate_text));
  let post_str = JSON.stringify(post_data);
  if ((id + 5) % 29 === 0 || (id + 3) % 13 === 0) {
    post_str = post_str.replace('"method":"', '"method" : "');
  } else {
    post_str = post_str.replace('"method":"', '"method": "');
  }

  const response = await postJson(url, post_str);

  //@ts-ignore
  // console.log("deepl", response.data.result.texts[0].text);
  return response.data.result.texts[0].text
}

export {
  deepl
}