import {MutableRefObject, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {message} from "@tauri-apps/plugin-dialog"

export default function ConnectBar(props: {
    nameRef: MutableRefObject<HTMLInputElement | null>,
    setConnected: (val: boolean) => void,
    setOtherUser: (val: string) => void
    setOtherId: (val: string) => void,
}) {
    const [inputVal, setInputVal] = useState("");
    const [connecting, setConnecting] = useState(false)
    
    function doClick() {
        tryConnect(
            inputVal, 
            props.setConnected,
            props.setOtherUser,
            props.setOtherId,
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
    setConnected: (val: boolean) => void, 
    setOtherUser: (val: string) => void,
    setOtherId: (val: string) => void,
    setConnecting: (val: boolean) => void
){
    setConnecting(true)
    
    invoke<string>("try_connect", { dst })
        .then((other) => {
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
            })
        });
}