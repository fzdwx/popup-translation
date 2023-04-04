import { invoke } from '@tauri-apps/api/tauri'
import { Config } from '@/types'

/**
 * 获取光标选择的文本/粘贴板内容
 * @returns selection text
 */
async function getSelectionText(): Promise<String> {
  return invoke<String>('get_selection_text')
}

/**
 * 写入配置
 * @param config string
 * @returns
 */
async function writeConfig(config: any): Promise<void> {
  return invoke('write_config', { data: config })
}

/**
 *  读取配置
 * @returns Config
 */
async function readConfig(): Promise<Config> {
  return invoke('read_config')
}

export { getSelectionText, writeConfig, readConfig }
