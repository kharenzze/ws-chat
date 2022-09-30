import { Chat } from "./components/Chat"

export const App = () => {
	return (
		<div className="app">
			<div>
				<Chat username="left" />
			</div>
			<div>
				<Chat username="right" />
			</div>
		</div>
	)
}
