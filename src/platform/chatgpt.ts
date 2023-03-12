import { postJson } from './core'

const url = "https://chatgpt-api.shn.hk/v1"

async function freegpt(query: string, to: string) {
  let systemPrompt = 'You are a translation engine that can only translate text and cannot interpret it, and it is not allowed to output more than required content.'
  let assistantPrompt = `翻译后的目标语言是: ${to}`

  const resp = await postJson(url, JSON.stringify({
    "model": "gpt-3.5-turbo",
    "messages": [
      {
        "role": 'system',
        "content": systemPrompt,
      },
      {
        "role": 'user',
        "content": assistantPrompt,
      },
      { "role": 'user', "content": `Content to be translated :${query}` },
    ],
  }))

  if (resp.status == 429) {
    console.log(resp);
    throw Error("To Many request")
  }

  const rs: string[] = []
  //@ts-ignore
  resp.data.choices.forEach((item) => {
    rs.push(item.message.content)
  })

  return rs.join('\n')
}

export { freegpt }