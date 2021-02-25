use crate::value::Value;

pub struct Space {
    width: u16,
    height: u16,
    fields: Box<[Value]>,
}

// new
impl Space {
    pub fn new_empty(width: u16, height: u16) -> Space {
        Space {
            width,
            height,
            fields: vec![Value::default(); (width * height) as usize].into_boxed_slice(),
        }
    }

    // pub fn new_from_str() -> Space {}
}

// methods
impl Space {
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn coord_wrapping(&self, (x, y): (Value, Value)) -> (Value, Value) {
        let (width, height) = (self.width as i32, self.height as i32);
        let (mut x, mut y) = (x.into_i32(), y.into_i32());

        if x < 0 {
            x = width - 1;
        } else if x >= width {
            x = 0;
        }

        if y < 0 {
            y = height - 1;
        } else if y >= height {
            y = 0;
        }

        (Value::from_i32(x), Value::from_i32(y))
    }

    pub fn field_index_wrapping(&self, (x, y): (Value, Value)) -> usize {
        let (x, y) = self.coord_wrapping((x, y));
        let (x, y) = (x.into_i32() as u16, y.into_i32() as u16);
        (self.width * y + x) as usize
    }

    pub fn field_at_mut(&mut self, (x, y): (Value, Value)) -> &mut Value {
        &mut self.fields[self.field_index_wrapping((x, y))]
    }

    pub fn field_at(&self, (x, y): (Value, Value)) -> &Value {
        &self.fields[self.field_index_wrapping((x, y))]
    }

    pub fn coord_advanced_in_direction(
        &self,
        origin: (Value, Value),
        delta: Value,
        direction: Value,
    ) -> (Value, Value) {
        // x coord (width: +right, -left)
        // y coord (height: +down, -up)
        // direction (0 is up, 1 is right, 2 is down, 3 is left)

        let (mut x, mut y) = (origin.0.into_i32(), origin.1.into_i32());
        let direction = direction.into_i32().abs() % 4;
        let delta = delta.into_i32();

        match direction {
            0 => y -= delta,
            1 => x += delta,
            2 => y += delta,
            3 => x -= delta,
            _ => {}
        }

        (Value::from_i32(x), Value::from_i32(y))
    }
}
