extern crate gl;
use gl::types::*;

pub type Id = GLuint;

pub enum Target {
  Value1D,
  Array1D,
  Value2D,
  Array2D,
  Value2DMultisample,
  Array2DMultisample,
  Value3D,
  CubeMap,
  CubeMapArray,
  Rectangle,
}

pub enum Property {
  WrapS,
  WrapT,
  WrapR,
  MinifyFilter,
  MagnifyFilter,
}

pub enum Wrap {
  Repeat,
  Mirrored,
  ClampToEdge,
  ClampToBorder,
}

pub enum Filter {
  Nearest,
  Linear,
  NearestMipmapNearest,
  NearestMipmapLinear,
  LinearMipmapNearest,
  LinearMipmapLinear,
}

pub enum InternalFormat {
  DepthComponent,
  DepthStencil,
  Red,
  RedGreen,
  RGB,
  RGBA,
}

pub enum DataFormat {
  Red,
  RedGreen,
  RGB,
  BGR,
  RGBA,
  BGRA,
  RedInteger,
  RedGreenInteger,
  RGBInteger,
  BGRInteger,
  RGBAInteger,
  BGRAInteger,
  StencilIndex,
  DepthComponent,
  DepthStencil,
}

pub enum DataType {
  U8,
  I8,
  U16,
  I16,
  U32,
  I32,
  F16,
  F32,
  U8_332,
  U8_233R,
  U16_565,
  U16_565R,
  U16_4444,
  U16_4444R,
  U16_5551,
  U16_1555R,
  U32_8888,
  U32_8888R,
  U32_1010102,
  U32_2101010R,
}

pub fn create() -> Id {
  let mut id: Id = 0;
  unsafe { gl::GenTextures(1, &mut id) };
  id
}

pub fn delete(ids: &[Id]) -> () {
  unsafe { gl::DeleteTextures(ids.len() as i32, ids.as_ptr()) };
}

pub fn bind(target: Target, id: Id) -> () {
  unsafe { gl::BindTexture(target.into(), id) };
}

pub fn apply_2d_image<Data>(
  target: Target,
  level: u32,
  internal_format: InternalFormat,
  width: u32,
  height: u32,
  data_format: DataFormat,
  data_type: DataType,
  data: &[Data])
  -> () {
  unsafe {
    gl::TexImage2D(
      target.into(),
      level as i32,
      internal_format.into(),
      width as GLsizei,
      height as GLsizei,
      0,
      data_format.into(),
      data_type.into(),
      data.as_ptr() as *const std::os::raw::c_void
    )
  };
}

pub fn set_texture_property<Value>(target: Target, property: Property, value: Value)
  -> ()
  where Value: Into<i32> {
  unsafe {
    gl::TexParameteri(target.into(), property.into(), value.into());
  }
}

pub fn generate_mipmap(target: Target) -> () {
  unsafe { gl::GenerateMipmap(target.into()) };
}

impl From<Target> for GLenum {
  fn from(value: Target) -> Self {
    match value {
      Target::Value1D => gl::TEXTURE_1D,
      Target::Array1D => gl::TEXTURE_1D_ARRAY,
      Target::Value2D => gl::TEXTURE_2D,
      Target::Array2D => gl::TEXTURE_2D_ARRAY,
      Target::Value2DMultisample => gl::TEXTURE_2D_MULTISAMPLE,
      Target::Array2DMultisample => gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
      Target::Value3D => gl::TEXTURE_3D,
      Target::CubeMap => gl::TEXTURE_CUBE_MAP,
      Target::CubeMapArray => gl::TEXTURE_CUBE_MAP_ARRAY,
      Target::Rectangle => gl::TEXTURE_RECTANGLE,
    }
  }
}

impl From<Property> for GLenum {
  fn from(value: Property) -> Self {
    match value {
      Property::WrapS => gl::TEXTURE_WRAP_S,
      Property::WrapT => gl::TEXTURE_WRAP_T,
      Property::WrapR => gl::TEXTURE_WRAP_R,
      Property::MinifyFilter => gl::TEXTURE_MIN_FILTER,
      Property::MagnifyFilter => gl::TEXTURE_MAG_FILTER,
    }
  }
}

impl From<InternalFormat> for i32 {
  fn from(value: InternalFormat) -> Self {
    (match value {
      InternalFormat::DepthComponent => gl::DEPTH_COMPONENT,
      InternalFormat::DepthStencil => gl::DEPTH_STENCIL,
      InternalFormat::Red => gl::RED,
      InternalFormat::RedGreen => gl::RG,
      InternalFormat::RGB => gl::RGB,
      InternalFormat::RGBA => gl::RGBA,
    }) as i32
  }
}

impl From<DataFormat> for GLenum {
  fn from(value: DataFormat) -> Self {
    match value {
      DataFormat::Red => gl::RED,
      DataFormat::RedGreen => gl::RG,
      DataFormat::RGB => gl::RGB,
      DataFormat::BGR => gl::BGR,
      DataFormat::RGBA => gl::RGBA,
      DataFormat::BGRA => gl::BGRA,
      DataFormat::RedInteger => gl::RED_INTEGER,
      DataFormat::RedGreenInteger => gl::RG_INTEGER,
      DataFormat::RGBInteger => gl::RGB_INTEGER,
      DataFormat::BGRInteger => gl::BGR_INTEGER,
      DataFormat::RGBAInteger => gl::RGBA_INTEGER,
      DataFormat::BGRAInteger => gl::BGRA_INTEGER,
      DataFormat::StencilIndex => gl::STENCIL_INDEX,
      DataFormat::DepthComponent => gl::DEPTH_COMPONENT,
      DataFormat::DepthStencil => gl::DEPTH_STENCIL,
    }
  }
}

impl From<DataType> for GLenum {
  fn from(value: DataType) -> Self {
    match value {
      DataType::U8 => gl::UNSIGNED_BYTE,
      DataType::I8 => gl::BYTE,
      DataType::U16 => gl::UNSIGNED_SHORT,
      DataType::I16 => gl::SHORT,
      DataType::U32 => gl::UNSIGNED_INT,
      DataType::I32 => gl::INT,
      DataType::F16 => gl::HALF_FLOAT,
      DataType::F32 => gl::FLOAT,
      DataType::U8_332 => gl::UNSIGNED_BYTE_3_3_2,
      DataType::U8_233R => gl::UNSIGNED_BYTE_2_3_3_REV,
      DataType::U16_565 => gl::UNSIGNED_SHORT_5_6_5,
      DataType::U16_565R => gl::UNSIGNED_SHORT_5_6_5_REV,
      DataType::U16_4444 => gl::UNSIGNED_SHORT_4_4_4_4,
      DataType::U16_4444R => gl::UNSIGNED_SHORT_4_4_4_4_REV,
      DataType::U16_5551 => gl::UNSIGNED_SHORT_5_5_5_1,
      DataType::U16_1555R => gl::UNSIGNED_SHORT_1_5_5_5_REV,
      DataType::U32_8888 => gl::UNSIGNED_INT_8_8_8_8,
      DataType::U32_8888R => gl::UNSIGNED_INT_8_8_8_8_REV,
      DataType::U32_1010102 => gl::UNSIGNED_INT_10_10_10_2,
      DataType::U32_2101010R => gl::UNSIGNED_INT_2_10_10_10_REV,
    }
  }
}

impl From<Wrap> for i32 {
  fn from(value: Wrap) -> Self {
    (match value {
      Wrap::Repeat => gl::REPEAT,
      Wrap::Mirrored => gl::MIRRORED_REPEAT,
      Wrap::ClampToEdge => gl::CLAMP_TO_EDGE,
      Wrap::ClampToBorder => gl::CLAMP_TO_BORDER,
    }) as i32
  }
}

impl From<Filter> for i32 {
  fn from(value: Filter) -> Self {
    (match value {
      Filter::Nearest => gl::NEAREST,
      Filter::Linear => gl::LINEAR,
      Filter::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
      Filter::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
      Filter::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
      Filter::LinearMipmapLinear => gl::LINEAR_MIPMAP_NEAREST,
    }) as i32
  }
}