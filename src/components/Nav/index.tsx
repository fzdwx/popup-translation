import { LogicalSize, appWindow } from '@tauri-apps/api/window'
import LangSwitch from './LangSwitch'
import { Mode } from '@/types'
import { defineComponent } from 'vue'

export default defineComponent({
  setup(props, { emit, slots }) {
    const { resetWindow, mode } = useConfigState()
    const { toogle, showSetting } = useToogleSetting()

    const onToogleSetting = () => {
      toogle()
      if (showSetting()) {
        appWindow.setSize(new LogicalSize(1000, 400))
      } else {
        resetWindow()
      }
    }

    return {
      onToogleSetting,
      mode,
      emit,
      slots
    }
  },
  render() {
    return (
      <div class="flex flex-row">
        <div class="basis-1/4">
          <div v-of={this.mode == Mode.Aggregate}>
            {
              //@ts-ignore
              this.slots['agg']()
            }
          </div>
        </div>
        <div class="basis-1/2">
          <div class="flex justify-center">{<LangSwitch />}</div>
        </div>
        <div class=" basis-1/4">
          <div class="flex justify-end">
            <button type="button" onClick={() => this.onToogleSetting()}>
              设置
            </button>
          </div>
        </div>
      </div>
    )
  }
})
