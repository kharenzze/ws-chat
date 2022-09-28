import { render } from 'preact'
import { App } from './App.jsx'

console.log("Starting ws")

const wsUrl = new URL('/ws/', window.location.origin.replace('http', 'ws'))

const ws = new WebSocket(wsUrl)

ws.addEventListener('open', (_evt) => {
	let count = 1
	const msg = "Msg no " + count
	console.log(`Sent ${count}`)
	ws.send(msg)
	count++
})

ws.addEventListener('message', (evt) => {
	console.log(evt.data)
})

render(<App />, document.body)
