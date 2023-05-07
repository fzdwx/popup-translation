import {useState} from 'react'

export default function () {
  const [state, setState] = useState(1)
  return (
    <div>
      {state}
      <div>
        <button onClick={() => setState(state + 1)}>+</button>
      </div>
    </div>
  )
}
