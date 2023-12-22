<div align="center">
  <h2>Soroban Purchase Receipt Contract</h2>
  <p>An example of custom payment contracts.</p>
  <a href="https://dev.to/fernandodavidmusician/soroban-purchase-receipt-contract-bg7">DEV.TO Tutorial</a>
</div>

The following repository contains an example of a Soroban smart contract designed to manage payments in native asset sent to customers as receipts.


# Getting Started

First, make sure you have set up the environment for developing smart contracts with Soroban. A detailed guide on how to install Rust, the target, the Soroban CLI and setting your code editor can be found <a href="https://soroban.stellar.org/docs/getting-started/setup">here</a>.

1. Clone this repository.
    ```sh
    git clone https://github.com/fernandodavidmartinez/soroban-purchase-receipt.git
    ```

2. Set the Testnet network.
    ```sh
    soroban config network add --global testnet --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015"
    ```

3. Create and fund 3 new addresses (`admin`, `recipient` and `payer`).
    ```sh
    soroban config identity generate --global admin

    soroban config identity address admin
    
    soroban config identity show admin
    
    curl "https://friendbot.stellar.org/?addr=$(soroban config identity address admin)"
    ```
    
    _Repeat for each one of the addresses by changing the name. Save both, public and private keys, to later access to funds._

4. Build the contract.
    ```sh
    soroban contract build
    ```

5. Deploy the contract.
    ```sh
    soroban contract deploy --wasm target/wasm32-unknown-unknown/release/hello_soroban.wasm --network testnet --source admin
    ```

6. Retrieve the Stellar Native Asset on Testnet.
    ```sh
    soroban lab token id --asset native --network testnet

    ID: "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"
    ```

7. Initialize the contract, set and store the information.
    ```sh
    soroban contract invoke --id CONTRACT_ID --network testnet --source admin -- init --admin admin --asset CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC --recipient recipient --amount 150000000 --receipt "Magazine monthly subscription = 15 XLM"
    ```

    _In this case, we have set a payment of 10 XLM for an hypothetical monthly subscription to a magazine. Note the way we set `amount`, we use "stroop" units, 1 XLM = 10_000_000 Stroops._

8. Invoke the `pay` function (by `payer`).
    ```sh
    soroban contract invoke --id CONTRACT_ID --network testnet --source payer -- pay --payer payer
    ```

9. Invoke the `withdraw` function (by `admin`).
    ```sh
    soroban contract invoke --id CONTRACT_ID --network testnet --source admin -- withdraw --admin admin
    ```

    _At this point, `recipient` should have received all the funds from the contract._

The functions `balance` and `receipt` can be called at any time, both functions are convenient in cases where `admin` needs to check either the amount of funds deposited or the receipt set, before proceeding to new invokations or updates of the storage for future payment requests.

10. Update information.
    ```sh
    soroban contract invoke --id CONTRACT_ID --network testnet --source admin -- update --admin admin --recipient recipient --amount 100000000 --receipt "Magazine monthly subscription = 10 XLM"
    ```

    _Updating the information stored in the contract, won't update the actual CONTRACT_ID._

**You are ready to start again and request custom payments to your customers!**

# Review in details

- `DataKey`: Each variant of this enum corresponds to a different piece of data that the contract store, such as the `Admin`, `Asset`, `Recipient`, `Amount`, and `Receipt`. 

- Helper functions (`get_asset`, `get_recipient`, `get_amount`, `get_receipt`, `get_balance`, `transfer`): These functions retrieve data from the contract's storage or perform actions like transferring assets. If the data is not found in the storage, these functions will panic and terminate the contract execution.

- `ContractTrait`: Defines the interface for the contract, including methods for initializing the contract, updating it, making a payment, checking the balance, retrieving the receipt, and withdrawing funds.

- `Contract`: Represents the contract itself.

- `ContractTrait` implementation for `Contract`: Implements the methods defined in the `ContractTrait` trait. These methods include checks for authorization and initialization, using the helper functions to interact with the contract's storage and perform actions.

### Functions

- `init`: Initializes the contract, the deployer set an `admin`, the native `asset` contract, a `recipient`, `amount`, and `receipt`. This function requires authorizaton to be invoked.

- `update`: Updates the the previous stored information.

- `pay`: Transfers the native `asset` (XLM) from the `payer` (the one who interact with the contract to pay for a purchase) to the `contract`.

- `balance`: This method is called from the CLI and returns the `balance` recieved by the contract.

- `receipt`: Returns the `receipt` from the contract's storage, useful before updating the contract information.

- `withdraw`: Transfers all the native `asset` from the contract to the `recipient`. Only the `admin` can call this method from the CLI.

## Contributing

Feel free to open issues, fork the project, create and push a new branch, commit changes, etc.

## License

This project is licensed under the `MIT License` provided in this repo.
