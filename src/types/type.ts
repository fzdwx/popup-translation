enum Platform {
	ChatGTP,
	YouDao,
	Bing,
	Google,
	Deepl,
}

enum Model {
	ModelOne,
	ModelTwo,
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

interface Config {
	keys: {
		chatGpt: string,
		youdao: string,
		google: string,
	}
}

export { Platform, Model };
export type {
	KeyInfo,
	TranslationItem,
	TranslationInfo,
	AggregateTranslationInfo,
	Config
};
