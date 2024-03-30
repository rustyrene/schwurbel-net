import { useState } from "react";

interface Props {
    room_ids: string[];
    on_join: (key: string) => void;
}

function WebRooms({ room_ids, on_join}: Props) {
  const [activeRoomKey, setActivRoomKey] = useState<string | null>(null);

  const handleJoin = (key: string) => {
    setActivRoomKey(key);
    on_join(key)
  }

  return (
    <>
        <div className="list-group">
          {room_ids.length > 1 && room_ids.filter(room_id => room_id != "").map(room_id => 
          <a href="#" className={`list-group-item list-group-item-action ${activeRoomKey === room_id ? "active" : ""}`} key={room_id} onClick={() => handleJoin(room_id)}>
            <div className="d-flex w-100 justify-content-between">
              <h5 className="mb-1">{room_id}</h5>
              <small><span className="badge text-bg-primary rounded-pill"></span></small>
            </div>
            <p className="mb-1">Some Text</p>
          </a>)}
        </div>
    </>
  )
}

export default WebRooms