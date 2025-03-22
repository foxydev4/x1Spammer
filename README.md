# Solana Transaction Spammer

This Rust application is designed to generate a high volume of Solana transactions, primarily for stress testing and performance analysis of Solana clusters. It leverages the Solana SDK and asynchronous operations via `tokio` to achieve efficient transaction submission.

## Features

* **High-Throughput Transaction Generation:** Sends numerous concurrent transactions to a specified Solana address.
* **Configurable Parameters:** Allows customization of the RPC URL, recipient address, private key, and number of concurrent tasks through environment variables.
* **`.env` Configuration:** Utilizes a `.env` file for easy and secure management of sensitive configuration details.
* **Asynchronous Operations:** Employs `tokio` for efficient concurrent transaction submission.
* **Real-time Statistics:** Provides feedback on transaction submission rate and elapsed time.
* **Pre-Spam Test Transaction:** Executes a single test transaction to verify connectivity and configuration before initiating the main spamming loop.

## Getting Started

### Prerequisites

* Rust and Cargo: Ensure you have Rust and Cargo installed on your system. You can download them from [rustup.rs](https://rustup.rs/).
* Solana CLI (Optional): While not strictly necessary for running the application, having the Solana CLI installed can be useful for interacting with the Solana network.

### Installation

1.  **Clone the Repository:**
    ```bash
    git clone <repository_url>
    cd solana-spammer
    ```
2.  **Configure Environment Variables:**
    * Create a `.env` file in the root directory of the project.
    * Add the following environment variables to the `.env` file, replacing the placeholder values with your actual data:

        ```
        RPC_URL="<your_solana_rpc_url>"
        RECIPIENT_ADDRESS="<recipient_solana_address>"
        PRIVATE_KEY_BYTES="<comma_separated_private_key_bytes>"
        NUM_TASKS="128" # Optional: Adjust the number of concurrent tasks
        ```

        **Important:** Handle your private key with extreme caution. Avoid committing it to version control.
3.  **Build the Project:**
    ```bash
    cargo build --release # use release for better performance
    ```
4.  **Run the Application:**
    ```bash
    cargo run --release
    ```

    The application will prompt you to confirm if you want to start spamming. Type `y` and press Enter to begin.

### Usage Notes

* **Security:** Exercise extreme caution when handling your private key. Never commit it to version control or share it.
* **Network Responsibility:** Use this tool responsibly and ethically. Only use it on networks where you have explicit permission.
* **Sufficient SOL:** Ensure that the sender account has sufficient SOL to cover transaction fees.
* **Performance Tuning:** The `NUM_TASKS` parameter significantly impacts performance. Adjust it based on your system's capabilities and the network's capacity.

## Contributing

Contributions are welcome! If you find a bug or have an idea for an enhancement, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more information.

## Disclaimer

This tool is provided as-is, without any warranty. Use it at your own risk. The authors are not responsible for any damages or losses incurred as a result of using this software.