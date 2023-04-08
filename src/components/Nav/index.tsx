import { LogicalSize, appWindow } from '@tauri-apps/api/window'

export default defineComponent(() => {
  const { resetWindow } = useConfigState()
  const { toogle, showSetting } = useToogleSetting()

  const onToogleSetting = () => {
    toogle()
    if (showSetting()) {
      appWindow.setSize(new LogicalSize(1000, 400))
    } else {
      resetWindow()
    }
  }

  return (
    <div class="flex flex-row">
      <div class="basis-1/4"></div>
      <div class="basis-1/4"></div>
      <div class=" basis-1/2">
        <div class="flex justify-end">
          <button type="button" onClick={() => onToogleSetting()}>
            设置
          </button>
        </div>
      </div>
    </div>
  )
})
