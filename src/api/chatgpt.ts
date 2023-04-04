import { postJson } from './comman';

const url = 'https://chatgpt-api.shn.hk/v1';

async function freegpt(query: string, to: string) {
  let systemPrompt = 'You are a translation engine that can only translate text and cannot interpret it, and it is not allowed to output more than required content.';
  let assistantPrompt = `translate to ${to}`;

  const resp = await postJson(
    url,
    JSON.stringify({
      model: 'gpt-3.5-turbo',
      messages: [
        {
          role: 'system',
          content: systemPrompt,
        },
        {
          role: 'user',
          content: assistantPrompt,
        },
        { role: 'user', content: `${query}` },
      ],
    })
  );

  if (resp.status == 429) {
    console.log(resp);
    throw Error('To Many request');
  }

  const rs: string[] = [];
  //@ts-ignore
  resp.data.choices.forEach((item) => {
    rs.push(item.message.content);
  });

  return rs.join('\n');
}

export { freegpt };
