use crate::{crd::handle_crd, devfile_schema::handle_devfile_schema};

pub mod crd;
pub mod devfile_schema;

fn main() {
    handle_devfile_schema();
    handle_crd();
}
