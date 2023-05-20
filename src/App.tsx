import {getSelectionText} from "@/command/core";
import {Card, CardHeader} from "@/components/ui/card";
import {listen} from "@tauri-apps/api/event";
import {useEffect, useState} from "react";
import {Button} from "@/components/ui/button";

async function readText(setText: React.Dispatch<React.SetStateAction<String | undefined>>) {
  const text = await getSelectionText();
  setText(text);
}

export default function () {
  const [text, setText] = useState<String>();

  listen('refresh-translation', async () => {
    await readText(setText)
  })

  return (
    <div>
      <div>
        <Button onClick={() => readText(setText)}>
          读取选中文本
        </Button>
      </div>

      <div>
        <ApiCard name="chatgpt"/>
        <img src="chatgpt.png"/>
      </div>
    </div>
  )
}


function ApiCard({name}: { name: string }) {
  return (
    <Card className="w-[350px]">
      <CardHeader>
        {name}
      </CardHeader>
    </Card>
  )
}
