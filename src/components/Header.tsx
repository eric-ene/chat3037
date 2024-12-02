import {invoke} from "@tauri-apps/api/core";
import {MutableRefObject, useEffect, useRef, useState} from "react";
import {message} from "@tauri-apps/plugin-dialog";
import MsgRequest from "./MsgRequest.tsx";
import {emit, listen} from "@tauri-apps/api/event";
import {ConnectedPayload, HandshakePayload, LoadState} from "../Classes.ts";



export default function Header(props: {nameRef: MutableRefObject<HTMLInputElement | null>}) {
    const [id, setId] = useState("loading...");
    const [loading, setLoading] = useState(LoadState.NotLoading);
    const buttonRef = useRef<HTMLButtonElement | null>(null);

    const [requestName, setRequestName] = useState("PLACEHOLDER");
    const [requestId, setRequestId] = useState("NO ID");
    const [requestActive, setRequestActive] = useState(false);
    const [requestKey, setRequestKey] = useState(true);
    
    const handshakeListener = listen<HandshakePayload>("handshake", (evt) => {
        if (evt.payload.status === "Request") {
            startTimer(evt.payload.sender, evt.payload.id);
        }
    });
    
    useEffect(() => {
        // TODO: This is stupid. I'll emit an event instead if I have time.
        async function applyId() {
            invoke<string>("get_identifier")
                .then((val) => {
                    setId(val);
                })
                .catch();
        }
        
        const interval = setInterval(() => applyId().catch(console.error), 1000);
        const timer = setTimeout(() => clearInterval(interval), 15 * 1000);
        
        return () => {
            clearInterval(interval);
            clearTimeout(timer);
        }
    }, []);
    
    function onTimerDone() {
        setRequestActive(false);
    }
    
    function resetTimer() {
        setRequestKey(val => !val);
    }
    
    function startTimer(name: string, id: string) {
        console.log(`${name}, ${id}`)
        setRequestName(name);
        setRequestId(id);
        setRequestActive(true);
        resetTimer();
    }
    
    async function onAccept(accept: boolean) {
        onTimerDone();
        invoke('handle_request', { dst: requestId, accept: accept })
            .then(() => {
                emit('connected', new ConnectedPayload(requestName, requestId))
            })
            .catch(console.log)
    }
    
    return (
        <div className={"topbar"}>
            <h1 id={"title"}>Chat3037</h1>
            <div id={"login"}>
                <input 
                    ref={props.nameRef} 
                    type={"text"} 
                    defaultValue={"user"} 
                    className={"no-border"}
                    onKeyDown={(evt) => {
                        if (evt.key == "Enter") onClick(props.nameRef, buttonRef, setLoading)
                    }}
                />
                <button 
                    onClick={() => onClick(props.nameRef, buttonRef, setLoading)}
                    ref={buttonRef}
                >{
                    loading == LoadState.NotLoading
                    ? "Log in"
                        : loading == LoadState.Loading
                        ? "Logging in..."
                        : "Logged in ✅"
                }</button>
            </div>
            <p>Your ID is {id}</p>
            {
                requestActive && <MsgRequest key={requestKey} nameFrom={requestName} onEmpty={onTimerDone} onAccept={onAccept}/>
            }
        </div>
    );
}

function startCountdown(maxSeconds: number, value: number) {
    
}

function onClick(
    nameRef: MutableRefObject<HTMLInputElement | null>, 
    buttonRef: MutableRefObject<HTMLButtonElement | null>,
    setLoading: (state: LoadState) => void,
) {
    setLoading(LoadState.Loading);
    
    invoke<string>('request_name', { name: nameRef.current?.value })
        .then(() => {
            setLoading(LoadState.Done)
            buttonRef.current?.style.setProperty("pointer-events", "none");
        })
        .catch((error) => {
            setLoading(LoadState.NotLoading)
            message(`${error}`, {
                title: "Couldn't claim name!",
                type: "warning",
            })
        });
}