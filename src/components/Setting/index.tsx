import { rs, defineComponent } from '@/utils'
import { SetupContext } from 'vue'
import { UseDark } from '@vueuse/components'

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

export default defineComponent((props: {}, { slots }: SetupContext) => {
  const { showSetting } = useToogleSetting()

  if (!showSetting()) return <div></div>
  return (
    <div>
      <div>{dark()}</div>
    </div>
  )
})
