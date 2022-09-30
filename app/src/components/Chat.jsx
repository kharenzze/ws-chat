import { useReducer, useMemo, useEffect } from 'preact/hooks'

const defState = {
	messages: [
		`I'm a message`,
		`I'm another message`,
	],
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
			const msg = "Msg from " + position
			ws.send(msg)
		})
		ws.addEventListener('message', (evt) => {
			dispatch({
				type: Actions.ReceiveMessage,
				message: evt.data
			})
		})
	}, [ws])
	return {
		ws
	}
}

export const Chat = ({ position }) => {
	console.log(position)
	const [state, dispatch] = useReducer(reducer, defState);
	useWebsocket(dispatch, position)
	const sendMessage = () => {
		const message = state.input
		if (!message) {
			console.log('Empty message')
			return
		}
		//todo send
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
	const messageElements = state.messages.map((m) => {
		return <p>{m}</p>
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
