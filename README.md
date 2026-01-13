<p align="center">
  <a href="https://seda.xyz/">
    <img width="90%" alt="seda-protocol" src="https://raw.githubusercontent.com/sedaprotocol/.github/refs/heads/main/images/banner.png">
  </a>
</p>

<h1 align="center">
  SEDA Starter Kit
</h1>

This starter kit helps you create Data Requests (also known as Oracle Programs) on the SEDA network using Rust. It showcases a basic project setup and serves as a foundation for building more complex projects.

## Requirements

- **Bun**: Install [Bun](https://bun.sh/) for package management and building.
- **Rust**: Install [Rust](https://rustup.rs/) for development and building.
- **WASM**: Install the [`wasm32-wasip1`](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1.html) target with `rustup target add wasm32-wasip1` for WASM compilation.

- Alternatively, use the [devcontainer](https://containers.dev/) for a pre-configured environment.

## Getting Started

A Data Request execution involves two phases executed in a WASM VM:

1. **Execution Phase**: The phase where non-deterministic operations occur. It can access public data via `http_fetch` or `proxy_http_fetch` calls. Multiple executor nodes run this phase and submit their reports to the SEDA network.

2. **Tally Phase**: Aggregates reports from the execution phase using custom logic to determine a final result.

> [!NOTE]
> This starter kit uses the same Oracle Program for both phases, but you can specify different binaries and add branching logic if needed.

### Building

To build the Oracle Program, run the following (builds using the release profile by default):

```sh
bun run build
```

### Local Testing

To test the Oracle Program, this project uses `@seda-protocol/vm` and `@seda-protocol/dev-tools`. These tools help run the Oracle Program in a local WASM VM and test different scenarios.

This project uses Bun's built-in test runner, but other JavaScript/TypeScript testing frameworks should also work.

> [!WARNING]
> The `@seda-protocol/vm` package might not work properly in Node.js. Try setting the environment variable `NODE_OPTIONS=--experimental-vm-modules` before running the test command.

```sh
bun run test
```

## Implement your Oracle Program

Use these key components to create and define your Oracle Program. The starter kit provides a base for building Oracle Programs on the SEDA network:

- **`src/main.rs`**: The entry point that coordinates both the execution and tally phases of your Data Request.

- **`src/execution_phase.rs`**: Manages the fetching and processing of price data from APIs. This phase involves non-deterministic operations as it can access public data via `http_fetch` and `proxy_http_fetch` calls. Multiple Executor Nodes run this phase, each producing a report that is sent to the SEDA network.

- **`src/tally_phase.rs`**: Aggregates results from multiple Executor reports and calculates the final output using consensus data. This phase is deterministic, combining results from Executor Nodes to reach a consensus.

### Utilities and Functions

The following are some of the key utilities and functions from the `seda-sdk` library used in the example provided in this starter kit. These tools help you build and define your Oracle Program. While these are a few important ones, the SDK offers additional utilities to explore:

- **`Process`**: Manages inputs and outputs, allowing interaction with the WASM VM.
- **`http_fetch`**: Fetches data from public APIs.
- **`Bytes`**: Assists in working with byte arrays, useful for encoding and decoding data.

These components and utilities serve as a foundation for developing your Oracle Program logic. For a complete list of utilities and advanced usage, refer to the official documentation.

## Interacting with SEDA Networks

You can upload Oracle Programs and interact with the SEDA network using the CLI tools provided by `@seda-protocol/dev-tools`.

### Uploading an Oracle Program

To upload an Oracle Program binary, run:

```sh
bun run deploy
```

> [!IMPORTANT]  
> This command requires `RPC_SEDA_ENDPOINT` and `MNEMONIC` environment variables.

Alternatively, you can directly use the CLI to upload an Oracle Program and list existing binaries.

List existing Oracle Programs (requires `RPC_SEDA_ENDPOINT` environment variable):

```sh
# With .env file
bunx seda-sdk oracle-program list
# With flag
bunx seda-sdk oracle-program list --rpc https://rpc.devnet.seda.xyz
```

Upload an Oracle Program (requires `RPC_SEDA_ENDPOINT` and `MNEMONIC` environment variables):

```sh
bunx seda-sdk oracle-program upload PATH_TO_BUILD
```

### Submitting a Data Request

`@seda-protocol/dev-tools` exposes functions that make it easy to create scripts that submit Data Requests to the SEDA network and await the result. The `scripts` directory shows an example.

Submitting a Data Request to the SEDA network, run:

```sh
bun run post-dr
```

This will post a transaction and wait till there is an result.

> [!IMPORTANT]  
> Make sure you have the all environment variables set in `.env` file.


Example of an `.env` file:

```sh
# RPC for the SEDA network you want to interact with
SEDA_RPC_ENDPOINT=https://rpc.devnet.seda.xyz

# Your SEDA chain mnemonic, fill this in to upload binaries or interact with data requests directly
SEDA_MNEMONIC=

# Used for posting data request on the seda chain and configuring the consumer contract
# You can get this by running `bun run deploy`
ORACLE_PROGRAM_ID=
```

### SEDA Fast API

For quick testing and development, you can use the SEDA Fast API to execute your Oracle Program without going through the full consensus process. This is ideal for debugging and rapid iteration.

#### Making a Request

Use the following curl command to execute your Oracle Program via SEDA Fast:

```bash
curl -L -X POST 'https://fast-api.testnet.seda.xyz/execute?encoding=json' \
  -H 'Authorization: Bearer <bearer>' \
  -H 'Content-Type: application/json' \
  --data-raw '{
    "execProgramId": "YOUR_PROGRAM_ID_HERE",
    "execInputs": {
      "event_slug": "2025-december-1st-2nd-3rd-hottest-on-record"
    }
  }'
```

Replace `YOUR_PROGRAM_ID_HERE` with your deployed Oracle Program ID and `<bearer>` with your API bearer token.

#### Request Parameters

- **`execProgramId`**: The ID of your deployed Oracle Program (get this from `bun run deploy`)
- **`execInputs`**: Input data for your Oracle Program. For this PolyMarket example, provide:
  - `event_slug`: The PolyMarket event identifier (e.g., "2025-december-1st-2nd-3rd-hottest-on-record")

#### Query Parameters

- **`encoding=json`**: Returns the result in JSON format for easy reading

#### Response Format

The API returns a JSON response with the execution results:

```json
{
  "_tag": "ExecuteResponse",
  "data": {
    "dataResult": {
      "exitCode": 0,
      "result": "7b226d61726b657473223a5b...", // Hex-encoded result
    },
    "result": {
      "markets": [
        {
          "yes_price": "0.0005",
          "closed": false
        },
        {
          "yes_price": "0.996",
          "closed": false
        },
        {
          "yes_price": "0.0005",
          "closed": false
        },
        {
          "yes_price": "0.0045",
          "closed": false
        }
      ]
    }
  }
}
```

- **`exitCode: 0`**: Indicates successful execution
- **`result` (hex)**: Raw hex-encoded output from your Oracle Program
- **`result` (parsed)**: Human-readable JSON when using `encoding=json`

#### Building and Deploying

Before testing with SEDA Fast, build and deploy your Oracle Program:

```bash
# Build the Oracle Program
bun run build

# Deploy to get your program ID
bun run deploy
```

#### Example: Testing This PolyMarket Oracle

This Oracle Program fetches prediction market data from PolyMarket for climate and weather events:

```bash
curl -L -X POST 'https://fast-api.testnet.seda.xyz/execute?encoding=json' \
  -H 'Authorization: Bearer <bearer>' \
  -H 'Content-Type: application/json' \
  --data-raw '{
    "execProgramId": "ae8ccc47060d2763490e963d22850517dd5e749818e13da00e4846f8a1c7a2a5",
    "execInputs": {
      "event_slug": "2025-december-1st-2nd-3rd-hottest-on-record"
    }
  }'
```

This will return market probability data for temperature record scenarios, with prices represented as decimal strings ("0.996" = 99.6% probability).

#### Understanding Market Index Mapping

The oracle returns an array of markets that directly corresponds to the markets within a PolyMarket event:

**PolyMarket Event Structure:**
```
Event: "2025-december-1st-2nd-3rd-hottest-on-record"
├── Market 0: "December 1st hottest" → yes_price: "0.0005" (0.05%)
├── Market 1: "December 2nd hottest" → yes_price: "0.996" (99.6%) 
├── Market 2: "December 3rd hottest" → yes_price: "0.0005" (0.05%)
├── Market 3: "None of the above" → yes_price: "0.0045" (0.45%)
```

**Oracle Result Array:**
```json
{
  "markets": [
    {"yes_price": "0.0005", "closed": false}, // Index 0 = "December 1st hottest" market
    {"yes_price": "0.996", "closed": false},  // Index 1 = "December 2nd hottest" market (99.6% likely!)
    {"yes_price": "0.0005", "closed": false}, // Index 2 = "December 3rd hottest" market
    {"yes_price": "0.0045", "closed": false}  // Index 3 = "None of the above" market
  ]
}
```

**Key Points:**
- **Index preservation**: The array index in the oracle result directly maps to the market index in the PolyMarket event
- **Consistent ordering**: Market 0 in PolyMarket = Index 0 in results, Market 1 = Index 1, etc.
- **First outcome focus**: Each market returns only the "Yes" outcome price (the first outcome in PolyMarket's outcome array)
- **Decimal string format**: Prices are returned as decimal strings representing probabilities (0.0 to 1.0 range)

This mapping ensures that consumers of the oracle data can reliably know which market each result corresponds to by using the array index.

> [!NOTE]
> SEDA Fast is designed for testing and development. For production use cases, deploy your Oracle Program to the full SEDA network using the standard Data Request process.

## License

Contents of this repository are open source under [MIT License](LICENSE).