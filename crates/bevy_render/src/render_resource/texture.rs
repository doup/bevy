use bevy_utils::Uuid;
use std::ops::Deref;

use crate::render_resource::resource_macros::*;

/// A [`Texture`] identifier.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TextureId(Uuid);

render_resource_wrapper!(ErasedTexture, wgpu::Texture);

/// A GPU-accessible texture.
///
/// May be converted from and dereferences to a wgpu [`Texture`](wgpu::Texture).
/// Can be created via [`RenderDevice::create_texture`](crate::renderer::RenderDevice::create_texture).
#[derive(Clone, Debug)]
pub struct Texture {
    id: TextureId,
    value: ErasedTexture,
}

impl Texture {
    /// Returns the [`TextureId`].
    #[inline]
    pub fn id(&self) -> TextureId {
        self.id
    }

    /// Creates a view of this texture.
    pub fn create_view(&self, desc: &wgpu::TextureViewDescriptor) -> TextureView {
        TextureView::from(self.value.create_view(desc))
    }
}

impl From<wgpu::Texture> for Texture {
    fn from(value: wgpu::Texture) -> Self {
        Texture {
            id: TextureId(Uuid::new_v4()),
            value: ErasedTexture::new(value),
        }
    }
}

impl Deref for Texture {
    type Target = wgpu::Texture;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// A [`TextureView`] identifier.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TextureViewId(Uuid);

render_resource_wrapper!(ErasedTextureView, wgpu::TextureView);
render_resource_wrapper!(ErasedSurfaceTexture, wgpu::SurfaceTexture);

/// This type combines wgpu's [`TextureView`](wgpu::TextureView) and
/// [`SurfaceTexture`](wgpu::SurfaceTexture) into the same interface.
#[derive(Clone, Debug)]
pub enum TextureViewValue {
    /// The value is an actual wgpu [`TextureView`](wgpu::TextureView).
    TextureView(ErasedTextureView),

    /// The value is a wgpu [`SurfaceTexture`](wgpu::SurfaceTexture), but dereferences to
    /// a [`TextureView`](wgpu::TextureView).
    SurfaceTexture {
        // NOTE: The order of these fields is important because the view must be dropped before the
        // frame is dropped
        view: ErasedTextureView,
        texture: ErasedSurfaceTexture,
    },
}

/// Describes a [`Texture`] with its associated metadata required by a pipeline or [`BindGroup`](super::BindGroup).
///
/// May be converted from a [`TextureView`](wgpu::TextureView) or [`SurfaceTexture`](wgpu::SurfaceTexture)
/// or dereferences to a wgpu [`TextureView`](wgpu::TextureView).
#[derive(Clone, Debug)]
pub struct TextureView {
    id: TextureViewId,
    value: TextureViewValue,
}

impl TextureView {
    /// Returns the [`TextureViewId`].
    #[inline]
    pub fn id(&self) -> TextureViewId {
        self.id
    }

    /// Returns the [`SurfaceTexture`](wgpu::SurfaceTexture) of the texture view if it is of that type.
    #[inline]
    pub fn take_surface_texture(self) -> Option<wgpu::SurfaceTexture> {
        match self.value {
            TextureViewValue::TextureView(_) => None,
            TextureViewValue::SurfaceTexture { texture, .. } => texture.try_unwrap(),
        }
    }
}

impl From<wgpu::TextureView> for TextureView {
    fn from(value: wgpu::TextureView) -> Self {
        TextureView {
            id: TextureViewId(Uuid::new_v4()),
            value: TextureViewValue::TextureView(ErasedTextureView::new(value)),
        }
    }
}

impl From<wgpu::SurfaceTexture> for TextureView {
    fn from(value: wgpu::SurfaceTexture) -> Self {
        let view = ErasedTextureView::new(value.texture.create_view(&Default::default()));
        let texture = ErasedSurfaceTexture::new(value);

        TextureView {
            id: TextureViewId(Uuid::new_v4()),
            value: TextureViewValue::SurfaceTexture { texture, view },
        }
    }
}

impl Deref for TextureView {
    type Target = wgpu::TextureView;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match &self.value {
            TextureViewValue::TextureView(value) => value,
            TextureViewValue::SurfaceTexture { view, .. } => view,
        }
    }
}

/// A [`Sampler`] identifier.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct SamplerId(Uuid);

render_resource_wrapper!(ErasedSampler, wgpu::Sampler);

/// A Sampler defines how a pipeline will sample from a [`TextureView`].
/// They define image filters (including anisotropy) and address (wrapping) modes, among other things.
///
/// May be converted from and dereferences to a wgpu [`Sampler`](wgpu::Sampler).
/// Can be created via [`RenderDevice::create_sampler`](crate::renderer::RenderDevice::create_sampler).
#[derive(Clone, Debug)]
pub struct Sampler {
    id: SamplerId,
    value: ErasedSampler,
}

impl Sampler {
    /// Returns the [`SamplerId`].
    #[inline]
    pub fn id(&self) -> SamplerId {
        self.id
    }
}

impl From<wgpu::Sampler> for Sampler {
    fn from(value: wgpu::Sampler) -> Self {
        Sampler {
            id: SamplerId(Uuid::new_v4()),
            value: ErasedSampler::new(value),
        }
    }
}

impl Deref for Sampler {
    type Target = wgpu::Sampler;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
