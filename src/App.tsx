import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
	const [scrape, setScrape] = useState([]);

	async function scrapeReq() {
		setScrape(await invoke("scrape_data_command", { scrape }));
	}

	console.log(scrape);

  return (
    <main className="container">
      <h1>clawed. </h1>
			<div>
				<button onClick={(e) => {
					setScrape(scrape)
					scrapeReq();
				}}> click me</button>
				<div id="scrapeList">
					{scrape.map(data => (
						<div id="scrapeItem">
							{data}
						</div>
					))}
				</div>
			</div>

    </main>
  );
}

export default App;
