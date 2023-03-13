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
    google:KeyItem,
    youdao: KeyItem,
}

export { Platform }
export type { KeyInfo }