import { useReducer } from 'preact/hooks'

const defState = {
	messages: [],
	input: ""
}

const Actions = {
	ChangeInput: 'change-input'
}

const handlers = {
	[Actions.ChangeInput]: (state, action) => {
		state.input = action.text
		return state
	}
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

export const Chat = () => {
	const [state, dispatch] = useReducer(reducer, defState);
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
	return (
		<div className="chat">
			<div className="row">
				<label>Room</label>
				<input />
			</div>
			<div className="chat-box">
				I'm a message
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
