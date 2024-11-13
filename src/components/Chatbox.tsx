import './Chatbox.css'
import {useRef} from "react";
import {Simulate} from "react-dom/test-utils";

export default function Chatbox() {
    const inputRef = useRef<HTMLInputElement | null>(null);
    
    return (
        <div className={"chatbox"} onClick={() => inputRef.current?.focus() }>
            <div className={"chat-window"}>
                <p>message 1</p>
            </div>
            <p className={"send"}>&gt;</p>
            <input ref={inputRef} className={"send"}/>
        </div>
    );
}