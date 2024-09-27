use std::ffi::OsString;

use crate::cargo::{run_cargo, CargoArgs};

pub fn test(args: CargoArgs) -> anyhow::Result<()> {
    run_cargo(args, OsString::from("test"), &[])
}
