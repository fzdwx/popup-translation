import { BaseDirectory, readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { KeyInfo } from "../types/type";

const filePath = "appKey.json";
const readKeys = async (): Promise<KeyInfo> => {
	const fileContent = await readTextFile(`${filePath}`, {
		dir: BaseDirectory.AppConfig,
	});
	return JSON.parse(fileContent);
};

const saveKey2Json = async (keys: KeyInfo) => {
	await writeTextFile(`${filePath}`, JSON.stringify(keys), {
		dir: BaseDirectory.AppConfig,
	});
};

export { saveKey2Json, readKeys };
