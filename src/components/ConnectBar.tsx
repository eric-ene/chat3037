import {MutableRefObject, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {message} from "@tauri-apps/plugin-dialog"

export default function ConnectBar(props: {
    nameRef: MutableRefObject<HTMLInputElement | null>,
    setConnected: (boolean) => void,
    setOtherUser: (string) => void
}) {
    const [inputVal, setInputVal] = useState("");
    const [connecting, setConnecting] = useState(false)
    
    function doClick() {
        tryConnect(
            inputVal, 
            props.nameRef.current?.value || "user",
            props.setConnected,
            props.setOtherUser,
            setConnecting
        ).then();
        
        console.log(`name: ${props.nameRef.current?.value}`)
    }
    
    return (
        <div className={"connectbar sidebyside"}>
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
    val: string, 
    user: string,
    setConnected: (boolean) => void, 
    setOtherUser: (string) => void,
    setConnecting: (boolean) => void
){
    setConnecting(true)
    
    invoke("try_connect", {seq: val, name: user})
        .then((other: string) => {
            setConnecting(false)
            setConnected(true)
            setOtherUser(other)
        })
        .catch((error: string) => {
            setConnecting(false)
            message(`Error connecting to target.\n${error}`, { 
                title: "Connection Error",
                type: "warning",
            })
        });
}