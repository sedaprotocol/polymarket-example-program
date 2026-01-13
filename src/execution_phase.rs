use anyhow::Result;
use seda_sdk_rs::{elog, http_fetch, log, Process};
use serde::{Deserialize, Serialize};

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[derive(Serialize, Deserialize)]
struct PolyMarketEvent {
    closed: bool,
    markets: Vec<PolyMarketMarket>,
}

#[derive(Serialize, Deserialize)]
struct PolyMarketMarket {
    #[serde(rename = "outcomePrices")]
    outcome_prices: String, // PolyMarket returns this as a JSON string, not an array
    #[serde(rename = "groupItemTitle")]
    group_item_title: String,
    closed: bool,
}

#[derive(Serialize, Deserialize)]
struct Market {
    yes_price: String,
    closed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    markets: Vec<Market>,
}

#[derive(Serialize, Deserialize)]
struct Input {
    event_slug: String,
}

// ============================================================================
// EXECUTION PHASE - FETCHES LIVE DATA FROM POLYMARKET ONLY
// ============================================================================

/**
 * Executes the data request phase within the SEDA network.
 * This phase fetches event data from PolyMarket for a specific event.
 * Takes a single input: event_slug
 * Loops through all markets in the event and extracts the first outcome price from each market.
 * Returns a JSON response with a vector of first outcome prices and whether the event is closed.
 */
pub fn execution_phase() -> Result<()> {
    // Retrieve the input parameters for the data request (DR).
    // Expected to be a single event_slug
    let event_slug = serde_json::from_slice::<Input>(&Process::get_inputs())?.event_slug;
    log!(
        "Fetching event data from PolyMarket for event: {}",
        event_slug
    );

    // Fetch PolyMarket event data
    let polymarket_event_response = http_fetch(
        format!("https://gamma-api.polymarket.com/events/slug/{}", event_slug),
        None,
    );

    // Check if the event request was successful
    if !polymarket_event_response.is_ok() {
        elog!(
            "PolyMarket HTTP Response was rejected: {} - {}",
            polymarket_event_response.status,
            String::from_utf8(polymarket_event_response.bytes)?
        );
        Process::error("Error while fetching PolyMarket event information".as_bytes());
    }

    // Parse event information
    let poly_market_event_data =
        serde_json::from_slice::<PolyMarketEvent>(&polymarket_event_response.bytes)?;

    // Validate that the event has at least one market
    if poly_market_event_data.markets.is_empty() {
        elog!("Event has no markets available");
        Process::error("Error: Event has no markets".as_bytes());
    }

    let markets: Vec<Market> = poly_market_event_data
        .markets
        .into_iter()
        .map(|market| {
            // Parse the outcome_prices JSON string into a vector
            let outcome_prices_array: Vec<String> = serde_json::from_str(&market.outcome_prices)?;

            if outcome_prices_array.is_empty() {
                elog!("Market {} has no outcome prices", market.group_item_title);
                return Err(anyhow::anyhow!(
                    "Market {} has no outcome prices",
                    market.group_item_title
                ));
            }

            // Get the first outcome price and parse it as f64
            let yes_price = outcome_prices_array[0].parse::<f64>().map_err(|e| {
                elog!(
                    "Failed to parse first outcome price for market {}: {}",
                    market.group_item_title,
                    e
                );
                anyhow::anyhow!("Invalid price format")
            })?;

            Ok(Market {
                yes_price: yes_price.to_string(),
                closed: market.closed,
            })
        })
        .collect::<Result<Vec<Market>>>()?;

    log!(
        "Collected {} first outcome prices from all markets",
        markets.len()
    );

    // Return the PolyMarket first outcome prices and event status as JSON
    Process::success(&serde_json::to_vec(&Response { markets })?);
}
