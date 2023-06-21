import { useState } from "react";
//import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  // malin l'etat en chaine vide, pas d'erreur comme sa
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (

    <>
      <h3>Je vais m'amuser sur ça bientôt</h3>
    </>
  );
}

export default App;
