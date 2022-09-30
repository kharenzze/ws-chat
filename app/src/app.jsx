import { Chat } from "./components/Chat"

export const App = () => {
	return (
		<div className="app">
			<div>
				<Chat position="left" />
			</div>
			<div>
				<Chat position="right" />
			</div>
		</div>
	)
}
