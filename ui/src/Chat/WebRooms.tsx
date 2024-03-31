interface Props {
    room_ids: string[];
    joined_room: string | null;
    on_join: (key: string) => void;
}

function WebRooms({ room_ids, joined_room, on_join}: Props) {

  const handleJoin = (key: string) => {
    on_join(key)
  }

  return (
    <>
        <div className="list-group">
          {room_ids.length > 0 && room_ids.filter(room_id => room_id != "").map(room_id => 
          <a href="#" className={`list-group-item list-group-item-action ${joined_room === room_id ? "active" : ""}`} key={room_id} onClick={() => handleJoin(room_id)}>
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