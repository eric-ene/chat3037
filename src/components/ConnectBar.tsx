import {MutableRefObject, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {message} from "@tauri-apps/plugin-dialog"
import {listen} from "@tauri-apps/api/event";
import {ConnectedPayload} from "../Classes.ts";

export default function ConnectBar(props: {
    nameRef: MutableRefObject<HTMLInputElement | null>,
    setConnected: (boolean) => void,
    setOtherUser: (string) => void
    setOtherId: (string) => void,
}) {
    const [inputVal, setInputVal] = useState("");
    const [connecting, setConnecting] = useState(false)
    
    function doClick() {
        tryConnect(
            inputVal, 
            props.setConnected,
            props.setOtherUser,
            setConnecting
        ).then();
    }
    
    return (
        <div className={"connectbar"}>
            <p className={"fit-content"}>Other ID:</p>
            <input 
                className={"purple-border"} 
                type={"text"}
                value={inputVal}
                onKeyDown={(evt) => {
                    if (evt.key == "Enter") doClick()
                }}
                onChange={(evt) => setInputVal(evt.target.value)}
            />
            <button onClick={doClick}>{connecting ? "Connecting..." : "Connect"}</button>
        </div>
    );
}

async function tryConnect(
    dst: string, 
    setConnected: (boolean) => void, 
    setOtherUser: (string) => void,
    setConnecting: (boolean) => void
){
    setConnecting(true)
    
    invoke("try_connect", { dst })
        .then((other: string) => {
            setConnecting(false)
            setConnected(true)
            setOtherUser(other)
            setOtherId(other)
        })
        .catch((error: string) => {
            setConnecting(false)
            setConnected(false)
            setOtherUser("")
            message(`Error connecting to target.\n${error}`, { 
                title: "Connection Error",
                type: "warning",
            })
        });
}