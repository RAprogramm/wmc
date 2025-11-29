use crate::{error::CoreError, marker::Marker};

/// GPU instance data for a single marker
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MarkerInstance {
    /// X position in screen space
    pub x: f32,
    /// Y position in screen space
    pub y: f32,
    /// Glow intensity
    pub intensity: f32,
    /// Animation phase
    pub phase: f32,
    /// Red color component
    pub r: f32,
    /// Green color component
    pub g: f32,
    /// Blue color component
    pub b: f32,
    /// Alpha transparency
    pub a: f32,
}

/// Buffer for managing marker instances
pub struct MarkerBuffer {
    data: Vec<MarkerInstance>,
    capacity: usize,
    count: usize,
}

impl MarkerBuffer {
    /// Creates a new marker buffer with the specified capacity
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
            count: 0,
        }
    }

    /// Returns the number of markers in the buffer
    #[must_use]
    pub const fn len(&self) -> usize {
        self.count
    }

    /// Returns true if the buffer contains no markers
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns the maximum capacity of the buffer
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Adds a marker instance to the buffer
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::BufferOverflow`] if the buffer is full
    pub fn push(&mut self, instance: MarkerInstance) -> Result<(), CoreError> {
        if self.count >= self.capacity {
            return Err(CoreError::BufferOverflow {
                requested: self.count + 1,
                capacity: self.capacity,
            });
        }

        if self.data.len() < self.capacity {
            self.data.push(instance);
        } else {
            self.data[self.count] = instance;
        }

        self.count += 1;
        Ok(())
    }

    /// Clears all markers from the buffer
    pub const fn clear(&mut self) {
        self.count = 0;
    }

    /// Returns the buffer data as a byte slice for GPU upload
    #[must_use]
    #[allow(unsafe_code)]
    pub fn as_bytes(&self) -> &[u8] {
        let data_slice = &self.data[0..self.count];
        let ptr = data_slice.as_ptr().cast::<u8>();
        let len = self.count * std::mem::size_of::<MarkerInstance>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    /// Updates a marker instance at the specified index
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::BufferOverflow`] if index is out of bounds
    pub fn update(&mut self, index: usize, instance: MarkerInstance) -> Result<(), CoreError> {
        if index >= self.count {
            return Err(CoreError::BufferOverflow {
                requested: index + 1,
                capacity: self.count,
            });
        }

        self.data[index] = instance;
        Ok(())
    }
}

/// Builder for creating marker instances from markers
pub struct InstanceBuilder {
    default_color: [f32; 4],
}

impl InstanceBuilder {
    /// Creates a new instance builder with the specified default color
    #[must_use]
    pub const fn new(default_color: [f32; 4]) -> Self {
        Self { default_color }
    }

    /// Builds a marker instance from a marker and screen coordinates
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub fn build(&self, marker: &Marker, x: f32, y: f32) -> MarkerInstance {
        let (r, g, b, a) = marker.color.map_or_else(
            || {
                (
                    self.default_color[0],
                    self.default_color[1],
                    self.default_color[2],
                    self.default_color[3],
                )
            },
            |color| (color.r, color.g, color.b, color.a),
        );

        MarkerInstance {
            x,
            y,
            intensity: marker.intensity,
            phase: marker.phase(),
            r,
            g,
            b,
            a,
        }
    }
}
