import "./Body.css"
import Chatbox from "./Chatbox.tsx";
import ConnectBar from "./ConnectBar.tsx";
import {MutableRefObject, useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";
import {ConnectedPayload} from "../Classes.ts";

export default function Body(props: {nameRef: MutableRefObject<HTMLInputElement | null>}) {
    const [connected, setConnected] = useState(false);
    const [other, setOther] = useState("(placeholder)")
    const [otherId, setOtherId] = useState("[PLACEHOLDER]")

    useEffect(() => {
        //listen to a event
        const unlisten = listen<ConnectedPayload>('connected', (evt) => {
            setConnected(true);
            setOther(evt.payload.name)
            setOtherId(evt.payload.id)
        });

        return () => {
            unlisten.then(f => f());
        }
    }, [] );

    
    return (
      <div className={"body"}>
          <p>Connected to {connected ? other : "nobody"}.</p>
          <ConnectBar nameRef={props.nameRef} setConnected={setConnected} setOtherUser={setOther} setOtherId={setOtherId}/>
          {
              connected 
              ? <Chatbox other={other} otherId={otherId}/>
              : null
          }
      </div>  
    );
}