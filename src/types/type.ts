enum Platform {
    YouDao,
    Bing,
    Google,
}

interface KeyItem {
    platform: string,
    key: string,
};

interface KeyInfo {
    google: KeyItem,
    youdao: KeyItem,
}

interface TranslationItem {
    text: string;
    loading: boolean;
}

interface TranslationInfo {
    source: TranslationItem;
    deepl: TranslationItem;
    chatgpt: TranslationItem;
    google: TranslationItem;
}

export { Platform }
export type { KeyInfo, TranslationItem, TranslationInfo }