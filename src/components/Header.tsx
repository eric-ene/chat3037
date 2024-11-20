import {invoke} from "@tauri-apps/api/core";
import {MutableRefObject, useEffect, useRef, useState} from "react";


export default function Header(props: {nameRef: MutableRefObject<HTMLInputElement | null>}) {
    const [id, setId] = useState("loading...");
    
    useEffect(() => {
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
    
    return (
        <div className={"topbar"}>
            <h1 id={"title"}>Chat3037</h1>
            <div id={"login"}>
                <p>Name:</p>
                <input ref={props.nameRef} type={"text"} defaultValue={"user"} className={"no-border"}/>
            </div>
            <p>Your ID is {id}</p>
        </div>
    );
}
