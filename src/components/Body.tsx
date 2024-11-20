import "./Body.css"
import Chatbox from "./Chatbox.tsx";
import ConnectBar from "./ConnectBar.tsx";
import {MutableRefObject, useState} from "react";

export default function Body(props: {nameRef: MutableRefObject<HTMLInputElement | null>}) {
    const [connected, setConnected] = useState(false);
    const [other, setOther] = useState("(placeholder)")
    
    return (
      <div className={"body"}>
          <p>Connected to {connected ? other : "nobody"}.</p>
          <ConnectBar nameRef={props.nameRef} setConnected={setConnected} setOtherUser={setOther}/>
          {
              connected 
              ? <Chatbox />
              : null
          }
      </div>  
    );
}