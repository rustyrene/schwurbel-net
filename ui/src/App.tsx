import { FormEvent, useEffect, useRef, useState } from "react";
import WebChat from "./Chat/WebChat"
import WebMessages from "./Chat/WebMessages";
import WebRooms from "./Chat/WebRooms";

function App() {
  const [messages, setMessages] = useState<WebMessage[]>([]);
  const [rooms, setRooms] = useState<string[]>([]);
  const [joinedRoom, setJoinedRoom] = useState<string|null>(null);

  //setting up websocket to server
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const handleCommand = (command: string) => {
      if (command.startsWith("/created")) {
        setRooms(prevRooms => [...prevRooms, command.split(" ")[1]]);
      }
      if (command.startsWith("/creator")) {
        setRooms(prevRooms => [...prevRooms, command.split(" ")[1]]);
        setJoinedRoom(command.split(" ")[1]);
      }
      if (command.startsWith("/list")) {
        const rooms = command.split(" ").splice(1);
        setRooms([...rooms.filter(room => rooms.indexOf(room) !== -1)]);
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
      console.log("LIST IT");
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

  const on_join = (key: string) => {
    if (socketRef.current && socketRef.current.OPEN) {
      socketRef.current.send(`/join ${key}`);
      setJoinedRoom(key);
    }
  }

  return (
    <>
      <WebRooms room_ids={rooms} joined_room={joinedRoom} on_join={on_join} />
      <WebMessages messages={messages} />
      <WebChat onSendMessage={onSendMessage} />
    </>
  )
}

export default App
