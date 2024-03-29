import { FormEvent, useEffect, useRef, useState } from "react";
import WebChat from "./Chat/WebChat"
import WebMessages from "./Chat/WebMessages";

function App() {
  const [messages, setMessages] = useState<WebMessage[]>([]);

  //setting up websocket to server
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const onReciveMessage = (event: MessageEvent<string>) => {
      setMessages(prevMessages => [...prevMessages, {
        msg: event.data,
        isAuthor: false,
      }])
    }

    socketRef.current = new WebSocket("ws://localhost:3000/ws/");
    socketRef.current.addEventListener("message", (event) => onReciveMessage(event))
    return () => {
      socketRef.current?.close();
    }
  }, [])

  const onSendMessage = (event: FormEvent, msg:string) => {
    event.preventDefault();
    if (socketRef.current && socketRef.current.readyState == WebSocket.OPEN) {
      socketRef.current.send(msg);
      //add message to record
      setMessages([...messages, {
        msg: msg,
        isAuthor: true,
      }]);
    }
  }

  return (
    <>
      <WebMessages messages={messages} />
      <WebChat onSendMessage={onSendMessage} />
    </>
  )
}

export default App
