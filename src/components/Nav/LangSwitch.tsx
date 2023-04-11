import { SetupContext } from 'vue'

export default defineComponent((props: {}, { slots }: SetupContext) => {
  const { getTargetLang, setTargetLang } = useConfigState()
  const state = ref<string>(getTargetLang())

  const changeLang = (e: Event) => {
    const lang = (e.target as HTMLSelectElement).value
    setTargetLang(lang)
  }

  return (
    <div>
      <select value={state.value} onChange={changeLang}>
        <option value="chinese">中文</option>
        <option value="english">English</option>
      </select>
    </div>
  )
})
