import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
	const [something, setSomething] = useState("");
	const [scrape, setScrape] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

	async function scrapeReq() {
		setScrape(await invoke("scrapeDataCommand", { scrape }));
	}

	console.log(scrape);
	const scrapedItems = [scrape];

  return (
    <main className="container">
      <h1>clawed. </h1>
			<div>
				<button onClick={(e) => {
					setSomething(something)
					setScrape(scrape)
					scrapeReq();
				}}> click me</button>
				<div id="scrapeList">
					{scrapedItems.map(data => (
						<div id="scrapeItem">
							<h1>hi:</h1>
							{data}
						</div>
					))}
				</div>
				<p>
					{scrape}
				</p>
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
