pub fn install_service() -> anyhow::Result<()> {
    #[cfg(unix)]
    {
        #[cfg(target_os = "linux")]
        {
            crate::os::linux::service::install_linux_service()
        }

        #[cfg(target_os = "macos")]
        {
            crate::os::mac::service::install_macos_service()
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            Err(anyhow::anyhow!(
                "Service installation not supported on this Unix platform"
            ))
        }
    }

    #[cfg(not(unix))]
    {
        Err(anyhow::anyhow!(
            "Service installation not supported on this platform"
        ))
    }
}
