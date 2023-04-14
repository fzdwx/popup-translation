import { rs, defineComponent } from '@/utils'
import { SetupContext } from 'vue'
import { UseDark } from '@vueuse/components'
import { Mode } from '@/types'

const dark = () => {
  return (
    <UseDark>
      {{
        default: ({ isDark, toggleDark }: any) => (
          <div>
            <button
              onClick={() => {
                toggleDark()
              }}
            >
              Is Dark: {rs(isDark, 'dark', 'light')}
            </button>
          </div>
        )
      }}
    </UseDark>
  )
}

const modeSwitch = () => {
  const { mode } = useConfigState()
  return (
    <div>
      <span class=" pr-2 ">模式切换</span>
      <input
        type="radio"
        name="1"
        value={mode}
        checked={mode.value == Mode.Aggregate}
      />
      <label for="1">聚合模式</label>
      <input
        type="radio"
        name="2"
        value={mode}
        checked={mode.value == Mode.Split}
      />
      <label for="2">拆分模式</label>
    </div>
  )
}

export default defineComponent((props: {}, { slots }: SetupContext) => {
  const { showSetting } = useToogleSetting()

  if (!showSetting()) return <div></div>
  return (
    <div>
      <div>{dark()}</div>
      <div>{modeSwitch()}</div>
    </div>
  )
})
