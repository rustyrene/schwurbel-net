import { FormEvent, useEffect, useRef, useState } from "react";
import WebChat from "./Chat/WebChat"
import WebMessages from "./Chat/WebMessages";
import WebRooms from "./Chat/WebRooms";

function App() {
  const [messages, setMessages] = useState<WebMessage[]>([]);
  const [rooms, setRooms] = useState<string[]>([]);

  console.log(rooms);

  //setting up websocket to server
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const handleCommand = (command: string) => {
      if (command.startsWith("/created")) {
        setRooms(prevRooms => [...prevRooms, command.split(" ")[1]]);
      }
      if (command.startsWith("/list")) {
        const rooms = command.split(" ").splice(1);
        setRooms(prevRooms => [...prevRooms, ...rooms]);
      }
    }

    const onReciveMessage = (event: MessageEvent<string>) => {
      if (event.data.startsWith("/")) {
        handleCommand(event.data);
      } else {
        setMessages(prevMessages => [...prevMessages, {
          msg: event.data,
          isAuthor: false,
        }])
      }
    }

    socketRef.current = new WebSocket("ws://localhost:3000/ws/");
    socketRef.current.addEventListener("message", (event) => onReciveMessage(event))
    socketRef.current.addEventListener("open", () => {
      socketRef.current?.send("/list");
    })
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
      <WebRooms room_ids={rooms} />
      <WebMessages messages={messages} />
      <WebChat onSendMessage={onSendMessage} />
    </>
  )
}

export default App
