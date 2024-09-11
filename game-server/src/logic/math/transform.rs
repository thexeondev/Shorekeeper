use shorekeeper_protocol::{Rotator, TransformData};

use super::Vector3f;

#[derive(Default, Clone, Debug)]
pub struct Transform {
    pub position: Vector3f,
    pub rotation: Vector3f,
}

impl Transform {
    pub fn get_position_protobuf(&self) -> shorekeeper_protocol::Vector {
        self.position.to_protobuf()
    }

    pub fn get_rotation_protobuf(&self) -> Rotator {
        Rotator {
            pitch: self.rotation.x,
            yaw: self.rotation.y,
            roll: self.rotation.z,
        }
    }

    pub fn set_position_from_protobuf(&mut self, pos: &shorekeeper_protocol::Vector) {
        self.position.x = pos.x;
        self.position.y = pos.y;
        self.position.z = pos.z;
    }

    pub fn set_rotation_from_protobuf(&mut self, rotator: &Rotator) {
        self.rotation.x = rotator.pitch;
        self.rotation.y = rotator.yaw;
        self.rotation.z = rotator.roll;
    }

    pub fn load_from_save(data: TransformData) -> Self {
        Self {
            position: Vector3f::from_save(data.position.unwrap_or_default()),
            rotation: Vector3f::from_save(data.rotation.unwrap_or_default()),
        }
    }

    pub fn build_save_data(&self) -> TransformData {
        TransformData {
            position: Some(self.position.save_data()),
            rotation: Some(self.rotation.save_data()),
        }
    }
}
