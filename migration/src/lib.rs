pub use sea_orm_migration::prelude::*;

// Importação dos arquivos de migration
mod m20250727_191031_create_user;
mod m20250727_191622_create_camera;
mod m20250727_191634_create_evidences;
mod m20250727_191715_create_oc_user;            // ✅ oc_user vem antes
mod m20250727_191643_create_evidence_photo;
mod m20250727_191709_create_historic;
mod m20250727_191730_create_status_occurrences;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250727_191031_create_user::Migration),
            Box::new(m20250727_191622_create_camera::Migration),
            Box::new(m20250727_191634_create_evidences::Migration),
            Box::new(m20250727_191715_create_oc_user::Migration),            // ✅ oc_user antes
            Box::new(m20250727_191643_create_evidence_photo::Migration),     // usa oc_user
            Box::new(m20250727_191709_create_historic::Migration),
            Box::new(m20250727_191730_create_status_occurrences::Migration), // provavelmente usa oc_user também
        ]
    }
}
