use attohttpc;
use config::builder::DefaultState;
use config::{ConfigBuilder, File};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum InputError {
    #[error("failed to load configuration file")]
    Configuration(#[from] config::ConfigError),
    #[error("failed to parse url")]
    ParseUrl(#[from] url::ParseError),
    #[error("error fetching input data")]
    HttpGet(#[from] attohttpc::Error),
    #[error("failed to cache data at {path}")]
    Caching { source: std::io::Error, path: String },
    #[error("get failed with {status}")]
    GetFailed { status: String },
}

pub struct Input {
    cache_path: PathBuf,
    url: Url,
    session: String,
}

impl Input {
    pub fn open<P: AsRef<Path>>(config: P) -> Result<Self, InputError> {
        let config = ConfigBuilder::<DefaultState>::default()
            .add_source(File::from(config.as_ref()))
            .build()
            .map_err(|e| InputError::Configuration(e))?;
        let cache_path = config.get_string("cache_path").map_err(|e| InputError::Configuration(e))?;
        let url = config.get_string("url").map_err(|e| InputError::Configuration(e))?;
        let session = config.get("session").map_err(|e| InputError::Configuration(e))?;

        Ok(Self {
            cache_path: PathBuf::from(cache_path),
            session,
            url: Url::parse(&url).map_err(|e| InputError::ParseUrl(e))?,
        })
    }

    pub fn get(&self, day: u8) -> Result<String, InputError> {
        let day = day.to_string();
        let input_path = self.cache_path.join(Path::new(&day)).join("input");
        let dir_path = self.cache_path.join(Path::new(&day));
        match fs::read_to_string(&input_path) {
            Ok(input) => Ok(input),
            Err(_) => {
                let mut new_url = self.url.clone();
                new_url.path_segments_mut().expect("Is base URL").extend(&["day", &day, "input"]);
                let input = attohttpc::get(new_url.as_str())
                    .header_append(attohttpc::header::COOKIE, format!("session={}", &self.session))
                    .send()
                    .map_err(|e| InputError::HttpGet(e))?;
                if input.is_success() {
                    let input = input.text().map_err(|e| InputError::HttpGet(e))?;
                    fs::create_dir_all(&dir_path).map_err(|e| InputError::Caching {
                        source: e,
                        path: dir_path.to_string_lossy().to_string(),
                    })?;
                    fs::write(&input_path, &input).map_err(|e| InputError::Caching {
                        source: e,
                        path: input_path.to_string_lossy().to_string(),
                    })?;
                    Ok(input)
                } else {
                    Err(InputError::GetFailed {
                        status: input.status().to_string(),
                    })
                }
            }
        }
    }
}
