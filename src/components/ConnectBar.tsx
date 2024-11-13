import {useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {message} from "@tauri-apps/plugin-dialog"

export default function ConnectBar(props: {setConnected: (boolean) => void, setOtherUser: (string) => void}) {
    const [inputVal, setInputVal] = useState("");
    const [connecting, setConnecting] = useState(false)
    
    return (
        <div className={"connectbar sidebyside"}>
            <p className={"fit-content"}>Other ID:</p>
            <input 
                className={"purple-border"} 
                type={"text"}
                value={inputVal}
                onChange={(evt) => setInputVal(evt.target.value)}
            />
            <button onClick={() => handleClick(
                inputVal, props.setConnected, 
                props.setOtherUser, 
                setConnecting
            )}>{connecting ? "Connecting..." : "Connect"}</button>
        </div>
    );
}

async function handleClick(
    val: string, 
    setConnected: (boolean) => void, 
    setOtherUser: (string) => void,
    setConnecting: (boolean) => void
){
    setConnecting(true)
    
    invoke("try_connect", {seq: val})
        .then((other: string) => {
            setConnecting(false)
            setConnected(true)
            setOtherUser(other)
        })
        .catch((error: string) => {
            setConnecting(false)
            message(`Error doing handshak\n${error}`, { 
                title: "Connection Error",
                type: "warning",
            })
        });
}