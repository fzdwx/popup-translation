enum Platform {
	ChatGTP,
	YouDao,
	Bing,
	Google,
	Deepl,
}

enum Mode {
	Aggergate = "aggergate",  // mode 1
	Split = "split",      // mode 2
}

interface Shortcurs {
	toogle: string;
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
		chatGpt: string,
		youdao: string,
		google: string,
	}
	mode?: Mode
	shortcuts?: Shortcurs
}

export { Platform, Mode };
export type {
	KeyInfo,
	TranslationItem,
	TranslationInfo,
	AggregateTranslationInfo,
	Config,
	Shortcurs,
};
