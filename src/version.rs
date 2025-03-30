use crate::generated::pb::a2o::Version;

pub fn agent_version() -> semver::Version {
    let version_str = env!("CARGO_PKG_VERSION");
    semver::Version::parse(version_str).unwrap_or(semver::Version::new(0, 0, 0))
}

pub fn agent_version_as_pb() -> Version {
    let version = agent_version();
    Version {
        major: version.major as u32,
        minor: version.minor as u32,
        patch: version.patch as u32,
    }
}
