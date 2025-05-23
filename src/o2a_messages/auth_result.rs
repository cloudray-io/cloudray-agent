use crate::config::set_agent_token;
use crate::generated::pb::o2a::AuthResult;
use crate::types::AgentToken;
use tracing::info;

pub async fn process_auth_result(message: AuthResult) -> anyhow::Result<()> {
    let token = message.agent_token;
    set_agent_token(AgentToken(token)).await;
    info!("Machine is registered on: {}", message.machine_url);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::get_agent_token;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_process_auth_result_success() {
        let test_token = "test_token".to_string();
        let auth_result = AuthResult {
            agent_token: test_token.clone(),
            machine_url: "".to_string(),
        };

        let result = process_auth_result(auth_result).await;

        assert!(result.is_ok());

        let stored_token = get_agent_token().await;
        assert!(stored_token.is_some());
        assert_eq!(stored_token.as_ref().unwrap().0, test_token);
    }

    #[tokio::test]
    #[serial]
    async fn test_process_auth_result_overwrites_existing() {
        let initial_token = "initial_token".to_string();
        set_agent_token(AgentToken(initial_token)).await;

        let new_token = "new_token".to_string();
        let auth_result = AuthResult {
            agent_token: new_token.clone(),
            machine_url: "".to_string(),
        };

        let result = process_auth_result(auth_result).await;

        assert!(result.is_ok());

        let stored_token = get_agent_token().await;
        assert!(stored_token.is_some());
        assert_eq!(stored_token.as_ref().unwrap().0, new_token);
    }
}
