console.log("Starting ws")

const wsUrl = new URL('/ws/', window.location.origin.replace('http', 'ws'))

const ws = new WebSocket(wsUrl)

let count = 1

setInterval(() => {
  const msg = "Msg no " + count
  console.log(`Sent ${count}`)
  ws.send(msg)
  count++
}, 1000)

ws.addEventListener('message', (evt) => {
  console.log(evt.data)
})
