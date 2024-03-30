interface Props {
    room_ids: string[];
}

function WebRooms({ room_ids }: Props) {
  return (
    <>
        <p>{room_ids.map(room => " " + room)}</p>
    </>
  )
}

export default WebRooms