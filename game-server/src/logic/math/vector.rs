use shorekeeper_protocol::{Vector, VectorData};

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn to_protobuf(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn from_save(data: VectorData) -> Self {
        Self {
            x: data.x,
            y: data.y,
            z: data.z,
        }
    }

    pub fn save_data(&self) -> VectorData {
        VectorData {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn from_data(data: &shorekeeper_data::VectorData) -> Self {
        Self {
            x: data.get_x(),
            y: data.get_y(),
            z: data.get_z(),
        }
    }
}
