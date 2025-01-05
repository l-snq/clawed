import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
	const [pingMsg, setPingMsg ] = useState("");
	const [something, setSomething] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

	async function ping() {
		setPingMsg(await invoke("ping", { something }));
	}

  return (
    <main className="container">
      <h1>clawed. </h1>
			<div>
				<button onClick={(e) => {
					ping();
					setSomething("hello")
				}}> click me</button>
				<p>{something}</p>
			</div>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
