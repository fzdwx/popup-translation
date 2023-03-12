import { getClient, Body } from '@tauri-apps/api/http'

const client = await getClient()

const supportedLanguages = [
  ["auto", "auto"],
  ["de", "DE"],
  ["en", "EN"],
  ["es", "ES"],
  ["fr", "FR"],
  ["it", "IT"],
  ["ja", "JA"],
  ["ko", "KO"],
  ["nl", "NL"],
  ["pl", "PL"],
  ["pt", "PT"],
  ["ru", "RU"],
  ["zh", "ZH"],
  ["zh", "ZH"],
  ["bg", "BG"],
  ["cs", "CS"],
  ["da", "DA"],
  ["el", "EL"],
  ["et", "ET"],
  ["fi", "FI"],
  ["hu", "HU"],
  ["lt", "LT"],
  ["lv", "LV"],
  ["ro", "RO"],
  ["sk", "SK"],
  ["sl", "SL"],
  ["sv", "SV"]
];

const langMap = new Map<string, string>(supportedLanguages.map(lang => [lang[0], lang[1]]));
const langMapReverse = new Map<string, string>(supportedLanguages.map(([standardLang, lang]) => [lang, standardLang]));

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

async function postData(url: string, data: string) {
  return await client.post<Object>(url, Body.text(data), {
    headers: {
      'Content-Type': 'application/json'
    }
  })
}



async function deepl(query: string, from: string, to: string): Promise<string> {
  const targetLanguage = langMap.get(to);
  const sourceLanguage = langMap.get(from);

  const source_lang = sourceLanguage || 'AUTO';
  const target_lang = targetLanguage || 'ZH';
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

  const response = await postData(url, post_str);

  //@ts-ignore
  // console.log("deepl", response.data.result.texts[0].text);
  return response.data.result.texts[0].text
}

export {
  deepl
}