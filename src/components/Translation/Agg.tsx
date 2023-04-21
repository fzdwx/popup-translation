import { SetupContext, defineComponent } from 'vue'
import { google } from '@/api/google'

type LangGetter = () => string

interface State {
  source: {
    text: string
    lang: 'auto' | 'chinese' | 'english'
  }
  google: {
    text: string
  }
  targetLang: LangGetter
}

const googleBox = (state: State) => {
  return <div>{state.google.text}</div>
}

const textbox = (str: string) => {
  return <div>{str}</div>
}

const refresh = async (state: State) => {
  getSelectionText()
}

export default defineComponent({
  setup(props, { emit }) {
    const { getTargetLang } = useConfigState()
    const state = reactive<State>({
      source: {
        text: '',
        lang: 'auto'
      },
      google: {
        text: ''
      },
      targetLang: getTargetLang
    })

    return { state }
  },
  render() {
    google('hello', 'auto', this.state.targetLang()).then((targetText) => {
      this.state.google.text = targetText
    })
    return <div>{googleBox(this.state)}</div>
  }
})
