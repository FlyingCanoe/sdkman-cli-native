pub mod constants {
    pub const CANDIDATES_DIR: &str = "candidates";
    pub const CANDIDATES_FILE: &str = "candidates";
    pub const CURRENT_DIR: &str = "current";
    pub const DEFAULT_SDKMAN_HOME: &str = ".sdkman";
    pub const SDKMAN_DIR_ENV_VAR: &str = "SDKMAN_DIR";
    pub const CANDIDATES_DIR_VAR: &str = "SDKMAN_CANDIDATES_DIR";
    pub const SDKMAN_CANDIDATES_API_VAR: &str = "SDKMAN_CANDIDATES_API";
    pub const SDKMAN_CANDIDATES_API_AVAILABLE_VAR: &str = "SDKMAN_AVAILABLE";
    pub const PLATEFORM_NAME_VAR: &str = "SDKMAN_PLATFORM";
    pub const ALLOW_UNSECURE_TLS_VAR: &str = "sdkman_insecure_ssl";
    pub const SHOULD_RETRY_VAR: &str = "sdkman_curl_retry";
    pub const MAX_RETRY: &str = "sdkman_curl_retry_max_time";
    pub const TMP_DIR: &str = "tmp";
    pub const VAR_DIR: &str = "var";
}

pub mod helpers {
    use colored::Colorize;
    use directories::UserDirs;
    use reqwest::blocking::ClientBuilder;
    use std::fmt::format;
    use std::path::PathBuf;
    use std::{env, fs, process};

    use crate::constants::{
        ALLOW_UNSECURE_TLS_VAR, CANDIDATES_DIR, CANDIDATES_DIR_VAR, CANDIDATES_FILE,
        DEFAULT_SDKMAN_HOME, PLATEFORM_NAME_VAR, SDKMAN_CANDIDATES_API_AVAILABLE_VAR,
        SDKMAN_CANDIDATES_API_VAR, SDKMAN_DIR_ENV_VAR, VAR_DIR,
    };

    pub fn infer_candidate_version(
        candidate: &str,
        version: Option<String>,
        folder: Option<String>,
    ) -> String {
        let api_available = env::var(SDKMAN_CANDIDATES_API_AVAILABLE_VAR).unwrap() == "true";
        let api_url = env::var(SDKMAN_CANDIDATES_API_VAR).unwrap();
        let plateform = env::var(PLATEFORM_NAME_VAR).unwrap();
        let candidate_dir = env::var(CANDIDATES_DIR_VAR).unwrap();
        let sdkman_dir = infer_sdkman_dir();
        if api_available {
            let version = version.unwrap_or_else(|| {
                get(format!("{api_url}/candidates/default/{candidate}").as_str())
            });
            let version_valid = get(format!(
                "{api_url}/candidates/validate/{candidate}/{version}/{plateform}"
            )
            .as_str());
            let version_valid = version_valid == "valid";
            if version_valid {
                return version;
            } else if folder.is_some() {
                return version;
            } else if fs::exists(format!("{candidate_dir}/{candidate}/{version}")).unwrap() {
                return version;
            }

            eprintln!(
                "\n\
            Stop! {candidate} {version} is not available. Possible causes:\n \
			* {version} is an invalid version\n \
			* {candidate} binaries are incompatible with your platform\n \
			* {candidate} has not been released yet\
			\
			Tip: see all available versions for your platform:\n\
			\
			$ sdk list {candidate}",
                version = version.italic(),
                candidate = candidate.bold()
            )
        } else if fs::metadata(format!("{candidate_dir}/{candidate}"))
            .unwrap()
            .is_dir()
        {
        } else if known_candidates(sdkman_dir).contains(x) {
        }

        todo!()
    }

    pub fn get(url: &str) -> String {
        let allow_unsecure = env::var(ALLOW_UNSECURE_TLS_VAR).unwrap() == "true";

        ClientBuilder::new()
            .danger_accept_invalid_certs(allow_unsecure)
            .build()
            .unwrap()
            .get(url)
            .send()
            .unwrap()
            .text()
            .unwrap()
    }

    pub fn infer_sdkman_dir() -> PathBuf {
        match env::var(SDKMAN_DIR_ENV_VAR) {
            Ok(s) => PathBuf::from(s),
            Err(_) => fallback_sdkman_dir(),
        }
    }

    fn fallback_sdkman_dir() -> PathBuf {
        UserDirs::new()
            .map(|dir| dir.home_dir().join(DEFAULT_SDKMAN_HOME))
            .unwrap()
    }

    pub fn check_file_exists(path: PathBuf) -> PathBuf {
        if path.exists() && path.is_file() {
            path
        } else {
            panic!("not a valid path: {}", path.to_str().unwrap())
        }
    }

    pub fn read_file_content(path: PathBuf) -> Option<String> {
        match fs::read_to_string(path) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
    }

    pub fn known_candidates<'a>(sdkman_dir: PathBuf) -> Vec<&'static str> {
        let absolute_path = sdkman_dir.join(VAR_DIR).join(CANDIDATES_FILE);
        let verified_path = check_file_exists(absolute_path);
        let panic = format!(
            "the candidates file is missing: {}",
            verified_path.to_str().unwrap()
        );
        let content = read_file_content(verified_path).expect(&panic);
        let line_str: &'static str = Box::leak(content.into_boxed_str());
        let mut fields = Vec::new();
        for field in line_str.split(',') {
            fields.push(field.trim());
        }

        fields
    }

    pub fn validate_candidate(all_candidates: Vec<&str>, candidate: &str) -> String {
        if !all_candidates.contains(&candidate) {
            eprintln!("{} is not a valid candidate.", candidate.bold());
            process::exit(1);
        } else {
            candidate.to_string()
        }
    }

    pub fn validate_version_path(base_dir: PathBuf, candidate: &str, version: &str) -> PathBuf {
        let version_path = base_dir.join(CANDIDATES_DIR).join(candidate).join(version);
        if version_path.exists() && version_path.is_dir() {
            version_path
        } else {
            eprintln!(
                "{} {} is not installed on your system",
                candidate.bold(),
                version.bold()
            );
            process::exit(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::io::Write;
    use std::path::PathBuf;

    use serial_test::serial;
    use tempfile::NamedTempFile;

    use crate::constants::SDKMAN_DIR_ENV_VAR;
    use crate::helpers::infer_sdkman_dir;
    use crate::helpers::read_file_content;

    #[test]
    #[serial]
    fn should_infer_sdkman_dir_from_env_var() {
        let sdkman_dir = PathBuf::from("/home/someone/.sdkman");
        env::set_var(SDKMAN_DIR_ENV_VAR, sdkman_dir.to_owned());
        assert_eq!(sdkman_dir, infer_sdkman_dir());
    }

    #[test]
    #[serial]
    fn should_infer_fallback_dir() {
        env::remove_var(SDKMAN_DIR_ENV_VAR);
        let actual_sdkman_dir = dirs::home_dir().unwrap().join(".sdkman");
        assert_eq!(actual_sdkman_dir, infer_sdkman_dir());
    }

    #[test]
    #[serial]
    fn should_read_content_from_file() {
        let expected_version = "5.0.0";
        let mut file = NamedTempFile::new().unwrap();
        file.write(expected_version.as_bytes()).unwrap();
        let path = file.path().to_path_buf();
        let maybe_version = read_file_content(path);
        assert_eq!(maybe_version, Some(expected_version.to_string()));
    }

    #[test]
    #[serial]
    fn should_fail_reading_file_content_from_empty_file() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();
        let maybe_version = read_file_content(path);
        assert_eq!(maybe_version, None);
    }
}
