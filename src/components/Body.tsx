import "./Body.css"
import Chatbox from "./Chatbox.tsx";
import ConnectBar from "./ConnectBar.tsx";
import {useState} from "react";

export default function Body() {
    const [connected, setConnected] = useState(false);
    const [other, setOther] = useState("(placeholder)")
    
    return (
      <div className={"body"}>
          <p>Connected to {connected ? other : "nobody"}.</p>
          <ConnectBar  setConnected={setConnected} setOtherUser={setOther}/>
          {
              connected 
              ? <Chatbox />
              : null
          }
      </div>  
    );
}