# Smart Contract Implementation details

The Contract object was made generic so it could have more than one implementation, for now only 2 implementation are made:

- NodeContract
- NameContract

## Flow

When either contract is created by the user, following operations happen:

- Contract object inserted in storage
- Contract is inserted in a "Billing Loop"
- Contract Billing Information is stored seperatly
- Contract Last billed information is stored seperatly


## Billing Loop

The Smart Contract module has an onfinalize method which is in charge of billing contracts. This pallet can be configured with a Trait `BillingFrequency` that controls how frequently a contract is billed. For example, if a contract needs to be billed every 1 hour, you set this value to 600. When a contract is created it will be inserted in a storage map that keeps track of at what block this specific contracts needs to be billed. The key is always initalised with (block_number_now + BillingFrequency), this happens on contract create. 

When the onfinalize method fires, a lookup is done on this storage map with the key being the current block and the value a list of contract ID's to be billed at that block. When a contract id is present in the values, the code starts to lookup the amount unbilled inside the contract's billing information. It bills the user according to the amount due and the price of TFT at that time and reinserts the contract to be billed at (block_number_now + BillingFrequency). This way the contract is billed at a consistent rate of `BillingFrequency`.


