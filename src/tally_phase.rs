use anyhow::Result;
use seda_sdk_rs::{elog, get_reveals, Process};
use crate::execution_phase::Response;

/**
 * Executes the tally phase within the SEDA network.
 * For SEDA fast mode, this phase expects exactly one reveal from the execution phase,
 * validates the response format, and returns it directly.
 * Note: SEDA fast mode uses a replication factor of 1.
 */
pub fn tally_phase() -> Result<()> {
    // Tally inputs can be retrieved from Process.getInputs(), though it is unused in this example.
    // let tally_inputs = Process::get_inputs();

    // Retrieve reveals from the tally phase.
    let reveals = get_reveals()?;

    // SEDA fast mode expects exactly 1 reveal
    if reveals.len() != 1 {
        elog!("SEDA fast mode expects exactly 1 reveal, got {}", reveals.len());
        Process::error("Expected exactly 1 reveal in SEDA fast mode".as_bytes());
    }

    let reveal = &reveals[0];
    
    // Parse and validate the single reveal
    let response = serde_json::from_slice::<Response>(&reveal.body.reveal).map_err(|e| {
        elog!("Failed to parse reveal as Response: {}", e);
        anyhow::anyhow!("Invalid response format")
    })?;

    // Return the single response directly (no aggregation needed)
    Process::success(&serde_json::to_vec(&response)?)
}

