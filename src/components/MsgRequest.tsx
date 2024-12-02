import './MsgRequest.css'
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

const timeout = 5;
const rate = 10;

export default function MsgRequest(props: { key: boolean, nameFrom: string, onEmpty: () => void, onAccept: (bool) => void }) { 
    let [value, setValue] = useState(timeout);
    let [max, setMax] = useState(timeout);
    
    useEffect(() => {
        const interval = setInterval(() => {
            setValue((prevNumber) => {
                if (prevNumber < 0.001) {
                    clearInterval(interval);
                    props.onEmpty();
                    return 0;
                }
                
                return prevNumber - (max / (timeout * (1000 / rate)));
            })
        }, rate);
        
        return () => clearInterval(interval);
    }, []);
    
    return (
        <div id={"msg-request"}>
            <p>Request from {props.nameFrom}</p>
            <button id={"accept"} onClick={() => props.onAccept(true)}>Accept</button>
            <button id={"reject"} onClick={() => props.onAccept(false)}>Reject</button>
            <progress value={value} max={max}></progress>
        </div>
    );
}