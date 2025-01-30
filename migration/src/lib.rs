#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250130_170002_projects;
mod m20250130_190331_assets;
mod m20250130_191426_remove_img_src_from_projects;
mod m20250130_191757_add_asset_ref_to_projects;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250130_170002_projects::Migration),
            Box::new(m20250130_190331_assets::Migration),
            Box::new(m20250130_191426_remove_img_src_from_projects::Migration),
            Box::new(m20250130_191757_add_asset_ref_to_projects::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
