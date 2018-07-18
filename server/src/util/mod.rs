use std::time::{Instant, Duration};
use std::sync::{Arc, RwLock};

use futures::Future;
use futures::future;
use futures::Stream;

use tokio;
use tokio_timer::Interval;

use opcua_types::service_types::ServerState as ServerStateType;

use state::ServerState;

/// This is a convenience for a polling action. This struct starts a repeating timer that calls
/// an action repeatedly.
pub struct PollingAction {}

impl PollingAction {
    pub fn spawn<F>(server_state: Arc<RwLock<ServerState>>, interval_ms: u32, action: F) -> PollingAction
        where F: 'static + Fn() + Send
    {
        let server_state_take_while = server_state.clone();
        let f = Interval::new(Instant::now(), Duration::from_millis(interval_ms as u64))
            .take_while(move |_| {
                let server_state = trace_read_lock_unwrap!(server_state);
                // If the server aborts or is in a failed state, this polling timer will stop
                let abort = match server_state.state {
                    ServerStateType::Failed |
                    ServerStateType::NoConfiguration |
                    ServerStateType::Shutdown => {
                        true
                    }
                    _ => {
                        server_state.is_abort()
                    }
                };
                if abort {
                    debug!("Polling action is stopping due to server state / abort");
                }
                future::ok(!abort)
            })
            .for_each(move |_| {
                // Polling timer will only call the action if the server is in a running state
                let process_action = {
                    let server_state = trace_read_lock_unwrap!(server_state_take_while);
                    server_state.is_running()
                };
                if process_action {
                    action();
                }
                Ok(())
            })
            .map_err(|_| ());
        let _ = tokio::spawn(f);
        PollingAction {}
    }
}
