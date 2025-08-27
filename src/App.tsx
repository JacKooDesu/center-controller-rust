import { useState } from "react";
// import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [jpegData, setJpegData] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  async function startUdp() {
    await invoke("start_udp");
  }

  async function stopUdp() {
    await invoke("stop_udp");
  }

  async function addJpgDecodedListener() {
    await invoke("add_jpg_decoded_listener");
    await listen("fm://jpeg_decoded", (data) => {
      console.log("Received JPEG packet:", data);
      let arr = new Uint8Array(data.payload as []);

      if (jpegData.length > 0)
        URL.revokeObjectURL(jpegData); // Clean up the old URL if it exists
      const url = URL.createObjectURL(new Blob([arr], { type: "image/jpeg" }));
      setJpegData(url);
    });
  }

  async function addClientChangedListener() {
    await invoke("add_client_changed_listener");
    await listen("fm://client_changed", (data) => {
      console.log("Received Client Changed:", data);
    });
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <button onClick={startUdp}>Start UDP</button>
        <button onClick={stopUdp}>Stop UDP</button>
        <button onClick={addJpgDecodedListener}>Add Jpeg Listener</button>
        <button onClick={addClientChangedListener}>Add Client Changed Listener</button>
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
      <p>{jpegData}</p>
      <img src={jpegData} alt="Decoded JPEG" width="300px" height="auto" />
    </main>
  );
}

export default App;
