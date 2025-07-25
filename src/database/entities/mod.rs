pub mod user;
pub mod camera;
pub mod evidences;
pub mod evidence_photo;
pub mod occurrences;
pub mod oc_user;
pub mod historic;
pub mod status_oc_user;
pub mod status_occurrences;

// reexports (se necessário)
pub use user::Entity as User;
pub use camera::Entity as Camera;
pub use evidences::Entity as Evidences;
pub use evidence_photo::Entity as EvidencePhoto;
pub use occurrences::Entity as Occurrences;
pub use oc_user::Entity as OcUser;
pub use historic::Entity as Historic;
pub use status_oc_user::Entity as StatusOcUser;
pub use status_occurrences::Entity as StatusOccurrences;
