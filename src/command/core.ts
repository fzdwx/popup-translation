import { invoke } from "@tauri-apps/api/tauri";
import { appWindow, WebviewWindow } from '@tauri-apps/api/window'

/**
 * 获取光标选择的文本/粘贴板内容
 * @returns selection text
 */
async function getSelectionText(): Promise<String> {
  return invoke<String>("get_selection_text")
}

export { getSelectionText }