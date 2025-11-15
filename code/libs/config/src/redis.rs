use derive_more::Display;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;

#[derive(Clone, Debug, Default)]
pub struct RedisConfig {
    designations: HashMap<RedisDesignation, RedisAccess>,
}

impl RedisConfig {
    pub fn get_access(
        &self,
        designation: &RedisDesignation,
    ) -> Result<RedisAccess, RedisConfigError> {
        self.designations
            .get(designation)
            .cloned()
            .ok_or(RedisConfigError::MissingDesignation(*designation))
    }
}

#[derive(Deserialize, Clone, Copy, Debug, Display, Hash, Eq, PartialEq)]
pub enum RedisDesignation {
    RenderWorkerChache,
    PreloadArticleWorkerChache,
}

#[derive(Clone, Debug)]
pub struct RedisAccess {
    pub url: Arc<str>,
    pub base_key: Arc<str>,
}

impl<'de> Deserialize<'de> for RedisConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serde_wrapper = serde_wrapper::RedisConfigSerde::deserialize(deserializer)?;
        RedisConfig::try_from(serde_wrapper).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Error)]
pub enum RedisConfigError {
    #[error("Duplicate designation: {0}")]
    DuplicateDesignation(RedisDesignation),
    #[error("Missing designation: {0}")]
    MissingDesignation(RedisDesignation),
}

mod serde_wrapper {
    use super::*;

    #[derive(Deserialize)]
    pub struct RedisConfigSerde {
        instances: Arc<[RedisInstance]>,
    }

    #[derive(Deserialize, Clone, Debug)]
    struct RedisInstance {
        url: Arc<str>,
        designations: Arc<[RedisInstanceDesignation]>,
    }

    #[derive(Deserialize, Clone, Debug)]
    struct RedisInstanceDesignation {
        base_key: Arc<str>,
        designation: RedisDesignation,
    }

    impl TryFrom<RedisConfigSerde> for RedisConfig {
        type Error = RedisConfigError;

        fn try_from(value: RedisConfigSerde) -> Result<Self, Self::Error> {
            let mut out = Self::default();
            for inst in value.instances.iter() {
                for inst_des in inst.designations.iter() {
                    if out
                        .designations
                        .insert(
                            inst_des.designation,
                            RedisAccess {
                                url: inst.url.clone(),
                                base_key: inst_des.base_key.clone(),
                            },
                        )
                        .is_some()
                    {
                        return Err(RedisConfigError::DuplicateDesignation(inst_des.designation));
                    }
                }
            }
            Ok(out)
        }
    }
}
