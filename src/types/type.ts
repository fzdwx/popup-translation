enum Platform {
  ChatGTP,
  YouDao,
  Bing,
  Google,
  Deepl,
}

enum Mode {
  Aggregate = 'aggregate', // mode 1
  Split = 'split', // mode 2
}

interface Shortcuts {
  toggle: string;
}

interface KeyItem {
  platform: string;
  key: string;
}

interface KeyInfo {
  chatGpt: KeyItem;
  google: KeyItem;
  youdao: KeyItem;
}

interface TranslationItem {
  text: string;
  loading: boolean;
  result?: string;
  targetLang: string;
}

interface AggregateTranslationInfo {
  source: TranslationItem;
  deepl: TranslationItem;
  chatgpt: TranslationItem;
  google: TranslationItem;
}

interface TranslationInfo {
  source: TranslationItem;
}

// app config
interface Config {
  keys: {
    chatGpt: string;
    youdao: string;
    google: string;
  };
  mode?: Mode;
  shortcuts?: Shortcuts;
}

export { Platform, Mode };
export type { KeyInfo, TranslationItem, TranslationInfo, AggregateTranslationInfo, Config, Shortcuts };
