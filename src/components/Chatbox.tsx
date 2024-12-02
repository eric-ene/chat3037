import './Chatbox.css'
import {useRef, useState, KeyboardEvent, ChangeEvent} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Message} from "../Classes.ts";

export default function Chatbox(props: { other: string, otherId: string }) {
    const inputRef = useRef<HTMLInputElement | null>(null);
    const [messages, setMessages] = useState<Message[]>([])
    const [currentMessage, setCurrentMessage] = useState("");
    const [id, setId] = useState(0);
    
    function generateMessage(dst: string, content: string) {
        let retval = new Message();
        
        retval.dst = dst;
        retval.sender = 'You';
        retval.content = content;

        setId(id => id + 1);
        retval.id = id;
        
        return retval;
    }
    
    function appendMessage(message: Message) {
        setMessages(oldArray => [...oldArray, message]);
    }
    
    function handleChange(evt: ChangeEvent<HTMLInputElement>) {
        setCurrentMessage(evt.target.value);
    }
    
    function handleKeyDown(evt: KeyboardEvent<HTMLInputElement>) {
        if (evt.code === 'Enter' && !isWhitespace(currentMessage)) {
            let msg = generateMessage(props.other, currentMessage);
            
            setCurrentMessage('');

            appendMessage(msg);
            
            console.log(props.otherId)
            
            return;
        }
    }
    
    return (
        <div className={"chatbox"} onClick={() => inputRef.current?.focus() }>
            <div className={"chat-window"}>
                {
                    messages.map((message) => {
                       return <div className={"message sidebyside"}>
                           <p key={message.id} className={"msg-name"}>{message.sender}:</p>
                           <p className={"msg-text"}>{message.content}</p>
                       </div>    
                    })
                }
            </div>
            <p className={"send"}>&gt;</p>
            <input ref={inputRef} className={"send"} onChange={handleChange} onKeyDown={handleKeyDown} value={currentMessage}/>
        </div>
    );
}

function isWhitespace(str: string) {
    return str.replace(/\s/g, '').length == 0;
}