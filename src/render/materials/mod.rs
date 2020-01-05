use super::math;
use super::math::Ray;
use super::math::Color;
use super::math::Intersection;

pub mod diffuse;
pub use diffuse::Diffuse;
pub mod normals;
pub use normals::Normals;
pub mod metalic;
pub use metalic::Metalic;
pub mod refractive;
pub use refractive::Refractive;

#[typetag::serde]
pub trait Material: Sync
{
    fn scatter(&self, ray: &Ray, intersection: &Intersection, scatter_index: u16) -> (Option<Ray>, Color);
}
