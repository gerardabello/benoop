use std::fs;
use std::fs::File;
use std::io::prelude::*;

use crate::config::Config;
use crate::error::Error;
use crate::reporter::Results;

const BASELINE_SAVE_FILE: &str = "./.baseline.bench";

pub fn get_config(file: &str) -> Result<Option<Config>, Error> {
    let contents = match fs::read_to_string(file) {
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => {
            return Err(Error::Io {
                source: e,
                op: "read-config-file",
            })
        }
        Ok(c) => c,
    };

    match serde_yaml::from_str(&contents) {
        Ok(c) => Ok(Some(c)),
        Err(e) => Err(Error::CannotParseConfig(file.to_string(), e)),
    }
}

pub fn save_config(file: &str, config: &Config) -> Result<(), Error> {
    let encoded: Vec<u8> =
        serde_yaml::to_vec(config).expect("config should be serializable to yaml");

    let mut file = File::create(file).map_err(|e| Error::Io {
        source: e,
        op: "save-config",
    })?;

    file.write_all(&encoded).map_err(|e| Error::Io {
        source: e,
        op: "save-config",
    })?;

    Ok(())
}

pub fn save_baseline(results: &Results) -> Result<(), Error> {
    let encoded: Vec<u8> =
        bincode::serialize(results).expect("results should be serializable to bincode");

    let mut file = File::create(BASELINE_SAVE_FILE).map_err(|e| Error::Io {
        source: e,
        op: "save-baseline",
    })?;

    file.write_all(&encoded).map_err(|e| Error::Io {
        source: e,
        op: "save-baseline",
    })?;

    Ok(())
}

pub fn get_baseline() -> Result<Option<Results>, Error> {
    let contents = match fs::read(BASELINE_SAVE_FILE) {
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => {
            return Err(Error::Io {
                source: e,
                op: "read-baseline",
            })
        }
        Ok(c) => c,
    };

    match bincode::deserialize(&contents) {
        Ok(c) => Ok(Some(c)),
        Err(e) => Err(Error::CannotParseBaseline(
            BASELINE_SAVE_FILE.to_string(),
            e,
        )),
    }
}

pub fn remove_baseline() -> Result<(), Error> {
    let mut file = fs::remove_file(BASELINE_SAVE_FILE).map_err(|e| Error::Io {
        source: e,
        op: "remove-baseline",
    })?;

    Ok(())
}
