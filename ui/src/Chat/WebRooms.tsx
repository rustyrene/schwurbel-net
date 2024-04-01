import { useState } from "react";
import { Button, Modal, Form } from "react-bootstrap";
import { IoAdd } from "react-icons/io5";

interface Props {
    room_ids: string[];
    joined_room: string | null;
    on_join: (key: string) => void;
}

function WebRooms({ room_ids, joined_room, on_join}: Props) {
  const [showModal, setShowModal] = useState(false);
  const [createRoomName, setCreateRoomName] = useState("");

  const handle_create = () => {
    console.log(createRoomName);
    setShowModal(false);
    setCreateRoomName("");
  }

  const handle_close = () => {
    setShowModal(false);
    setCreateRoomName("");
  }

  return (
    <>
      <div className="ms-2">
        <div className="list-group">
          {room_ids.length > 0 && room_ids.filter(room_id => room_id != "").map(room_id => 
          <a href="#" className={`list-group-item list-group-item-action ${joined_room === room_id ? "active" : ""}`} key={room_id} onClick={() => on_join(room_id)}>
            <div className="d-flex w-100 justify-content-between">
              <h5 className="mb-1">{room_id}</h5>
              <small><span className="badge text-bg-primary rounded-pill"></span></small>
            </div>
            <p className="mb-1">Room Description</p>
          </a>)}
        </div>
        <div className="d-flex create-room-btn">
          <button className="btn btn-primary" onClick={() => setShowModal(true)}><IoAdd className="fs-3" /> Create Room</button>
        </div>
        {/* Modal */}
        <Modal show={showModal} onHide={handle_close} centered>
          <Modal.Header closeButton>
            <Modal.Title>Create a new </Modal.Title>
           </Modal.Header>
          <Modal.Body>
            <Form.Group>
              <Form.Label>Room Name</Form.Label>
              <Form.Control type="text" value={createRoomName} onChange={(event) => setCreateRoomName(event.target.value)} />
            </Form.Group>
          </Modal.Body>
          <Modal.Footer>
            <Button variant="secondary" onClick={handle_close}>Close</Button>
            <Button variant="primary" onClick={handle_create}>Create</Button>
          </Modal.Footer>
        </Modal>
      </div>
    </>
  )
}

export default WebRooms