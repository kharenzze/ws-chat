import { useReducer, useMemo, useEffect } from 'preact/hooks'
import { Message } from './Message'

const defState = {
	messages: [],
	input: ""
}

const Actions = {
	ChangeInput: 'change-input',
	ReceiveMessage: 'receive-message'
}

const handlers = {
	[Actions.ChangeInput]: (state, action) => {
		state.input = action.text
		return state
	},
	[Actions.ReceiveMessage]: (state, action) => {
		state.messages.push(action.message)
		return state
	},
}

const reducer = (state, action) => {
	const handler = handlers[action.type]
	if (handler) {
		state = {
			...handler(state, action)
		}
	}
	return state
}

const wsUrl = new URL('/ws/', window.location.origin.replace('http', 'ws'))
const useWebsocket = (dispatch, position) => {
	const ws = useMemo(() => new WebSocket(wsUrl), [])
	useEffect(() => {
		ws.addEventListener('open', (_evt) => {
			const msg = {
				text: "I'm  " + position,
			}
			ws.send(JSON.stringify(msg))
		})
		ws.addEventListener('message', (evt) => {
			dispatch({
				type: Actions.ReceiveMessage,
				message: JSON.parse(evt.data)
			})
		})
	}, [ws])
	return {
		ws
	}
}

export const Chat = ({ position }) => {
	const [state, dispatch] = useReducer(reducer, defState);
	const { ws } = useWebsocket(dispatch, position)
	const sendMessage = () => {
		const message = state.input
		if (!message) {
			console.log('Empty message')
			return
		}
		const data = {
			text: state.input,
		}
		ws.send(JSON.stringify(data))
		dispatch({
			type: Actions.ChangeInput,
			text: ''
		})
	}
	const onInputPress = (evt) => {
		if (evt.charCode === 13) {
			sendMessage()
		}
	}
	const onChangeInput = (evt) => {
		const text = evt.target.value
		dispatch({
			type: Actions.ChangeInput,
			text
		})
	}
	const messageElements = state.messages.map((m, i) => {
		return <Message message={m} key={i} />
	})
	return (
		<div className="chat">
			<div className="row">
				<label>Room</label>
				<input />
			</div>
			<div className="chat-box">
				{messageElements}
			</div>
			<div className="row">
				<input onKeyPress={onInputPress} onInput={onChangeInput} value={state.input} />
				<button onClick={sendMessage}>
					Send
				</button>
			</div>
		</div>
	)
}
