# **Most: Aleph Zero Bridge Audit Competition on Hats.finance** 


## Introduction to Hats.finance


Hats.finance builds autonomous security infrastructure for integration with major DeFi protocols to secure users' assets. 
It aims to be the decentralized choice for Web3 security, offering proactive security mechanisms like decentralized audit competitions and bug bounties. 
The protocol facilitates audit competitions to quickly secure smart contracts by having auditors compete, thereby reducing auditing costs and accelerating submissions. 
This aligns with their mission of fostering a robust, secure, and scalable Web3 ecosystem through decentralized security solutions​.

## About Hats Audit Competition


Hats Audit Competitions offer a unique and decentralized approach to enhancing the security of web3 projects. Leveraging the large collective expertise of hundreds of skilled auditors, these competitions foster a proactive bug hunting environment to fortify projects before their launch. Unlike traditional security assessments, Hats Audit Competitions operate on a time-based and results-driven model, ensuring that only successful auditors are rewarded for their contributions. This pay-for-results ethos not only allocates budgets more efficiently by paying exclusively for identified vulnerabilities but also retains funds if no issues are discovered. With a streamlined evaluation process, Hats prioritizes quality over quantity by rewarding the first submitter of a vulnerability, thus eliminating duplicate efforts and attracting top talent in web3 auditing. The process embodies Hats Finance's commitment to reducing fees, maintaining project control, and promoting high-quality security assessments, setting a new standard for decentralized security in the web3 space​​.

## Most: Aleph Zero Bridge Overview

Most is a token bridge between Aleph Zero and Ethereum.

## Competition Details


- Type: A public audit competition hosted by Most: Aleph Zero Bridge
- Duration: 14 days
- Maximum Reward: $40,000
- Submissions: 74
- Total Payout: $23,752 distributed among 12 participants.

## Scope of Audit

## Project overview
Contract part of the MOST guardian bridge designed to bridge ERC20 tokens from Ethereum to Aleph Zero network.
It includes contracts written both in Solidity and ink! language.

## Audit competition scope

```
|-- most
     |-- azero
          |-- contracts
               |-- most
               |-- token
               |-- advisory
               |-- migrations
               |-- ownable2step
               |-- gas-price-oracle
               |-- psp22-traits
               |-- shared
       
     |-- eth
           |-- contracts
                |-- Most.sol
                |-- Migrations.sol
```

## Medium severity issues


- **Issue with WETH Conversion to ETH Causes Transfer Failures in Contracts**

  In the `eth::most::receiveRequest` function, there's an issue with the check `if (_destTokenAddress == wethAddress)`. Using `wethAddress` to differentiate between tokens and ETH may lead to unexpected behavior, particularly for contracts that only accept WETH and not native ETH. For instance, if Contract B interacts with the bridge contract by sending WETH, it may later try to retrieve WETH and instead receive ETH, causing the transfer to fail. This mismatch can result in funds being temporarily locked until recovered by the owner. To resolve this, it is suggested to use a custom address to differentiate between ETH and other tokens, ensuring consistency in handling WETH transactions.


  **Link**: [Issue #34](https://github.com/hats-finance/Most--Aleph-Zero-Bridge-0xab7c1d45ae21e7133574746b2985c58e0ae2e61d/issues/34)


- **Changing Committee Signature Threshold Causes Previous Requests to Be Unprocessable**

  The `receive_request` function in `azero/contracts/most/lib.rs` incorrectly uses the current committee's signature threshold value rather than the committee ID supplied through the request. This results in requests from a previous committee with a lower threshold being unprocessable if the committee changes and the threshold is raised. For instance, if an initial committee of four members with a threshold of four is replaced by a new committee of five members with a threshold of five, any pending requests from the initial committee would remain unresolved. A suggested fix involves modifying the function to use the committee ID supplied in the request instead of the current committee ID. Additionally, the existing Solidity version implements this correctly.


  **Link**: [Issue #63](https://github.com/hats-finance/Most--Aleph-Zero-Bridge-0xab7c1d45ae21e7133574746b2985c58e0ae2e61d/issues/63)

## Low severity issues


- **`committee_sizes` Not Updated in set_committee() Leading to State Discrepancies**

  The `set_committee()` function does not update the `committee_sizes` variable, which is essential for accounting purposes. This oversight causes outdated committee size references in various functions, potentially leading to errors, particularly when calculating rewards. The recommended fix is to insert the committee size update into the `set_committee()` function.


  **Link**: [Issue #27](https://github.com/hats-finance/Most--Aleph-Zero-Bridge-0xab7c1d45ae21e7133574746b2985c58e0ae2e61d/issues/27)

## Minor severity issues


- **Issue with Migration Contract: Upgrade Function Fails to Set Last Completed Migration**

  The `Migrations::upgrade` function fails to work as expected due to the `restricted` modifier, which prevents the proper setting of the `last_completed_migration` value in the new migration contract. The proof of concept demonstrates this issue, showing that the `last_completed_migration` remains unset in the new contract. Suggested resolution includes removing the `upgrade` functionality to save deployment gas.


  **Link**: [Issue #3](https://github.com/hats-finance/Most--Aleph-Zero-Bridge-0xab7c1d45ae21e7133574746b2985c58e0ae2e61d/issues/3)


- **Uninitialized Contract Vulnerability in Most.sol Using OpenZeppelin's Upgradeable Model**

  The `Most.sol` contract, utilizing OpenZeppelin's upgradeable model, is vulnerable to an attacker initializing it before the original deployer. To prevent this, it is recommended to add the `_disableInitializers()` function in the constructor to lock the contract upon deployment.


  **Link**: [Issue #4](https://github.com/hats-finance/Most--Aleph-Zero-Bridge-0xab7c1d45ae21e7133574746b2985c58e0ae2e61d/issues/4)


- **Implement Zero Address Checks to Prevent Governance Issues and Fund Loss**

  Zero address checks are recommended for contract initialization to avoid accidents that could require redeployment. Additionally, safeguards should be implemented for user requests to prevent loss of funds when sending to a null destination address. These measures ensure better governance and user experience by avoiding unexpected states and fund loss.


  **Link**: [Issue #10](https://github.com/hats-finance/Most--Aleph-Zero-Bridge-0xab7c1d45ae21e7133574746b2985c58e0ae2e61d/issues/10)


- **Restricted Modifier Allows Non-Owners to Execute Functions Without Reverting**

  In `Migrations.sol`, the `restricted()` modifier intended to limit function access to the contract owner lacks strict checks and does not revert when called by non-owners. Although the functions are not executed by the unintended callers, they appear to run successfully, causing confusion. A suggested fix is to implement a revert message to save gas and clearly indicate unauthorized access.


  **Link**: [Issue #18](https://github.com/hats-finance/Most--Aleph-Zero-Bridge-0xab7c1d45ae21e7133574746b2985c58e0ae2e61d/issues/18)


- **Incorrect Pocket Money Balance Decrement on Failed Transfer in receive_request Function**

  In the `receive_request` function, when a transfer fails, the `pocket_money_balance` is still decremented, leading to inconsistent state management. If the transfer fails, the balance should remain unchanged to maintain correct state integrity. Despite no proof of concept being provided, the validity of the issue is acknowledged after mitigation.


  **Link**: [Issue #21](https://github.com/hats-finance/Most--Aleph-Zero-Bridge-0xab7c1d45ae21e7133574746b2985c58e0ae2e61d/issues/21)


- **Relay Gas Variables Should be Modifiable Instead of Immutable**

  Certain gas-related values (`relay_gas_usage`, `min_gas_price`, `max_gas_price`, `default_gas_price`) are currently immutable and need modification access for future updates. Adjusting these values, especially as the committee size grows or gas_price_oracle changes, can help maintain accurate base fee calculations and mitigate potential issues. Implementing a function to update these gas variables is recommended.


  **Link**: [Issue #30](https://github.com/hats-finance/Most--Aleph-Zero-Bridge-0xab7c1d45ae21e7133574746b2985c58e0ae2e61d/issues/30)


- **Implement Total Outstanding Rewards to Protect Azero Transfers in Contract**

  The contract currently lacks a mechanism to track total outstanding rewards, making it difficult to safely recover AZERO without affecting unclaimed rewards. The `recover_azero` function might inadvertently deduct unclaimed rewards. It's recommended to implement a `total_outstanding_rewards` metric to prevent such issues and ensure accurate internal accounting.


  **Link**: [Issue #33](https://github.com/hats-finance/Most--Aleph-Zero-Bridge-0xab7c1d45ae21e7133574746b2985c58e0ae2e61d/issues/33)


- **Missing Initialization of `requestNonce` in Upgradeable Contract's Initialize Function**

  In `Most.sol`, the `initialize()` function for the upgradeable `Most` contract is missing an initialization for the `requestNonce` state variable. Unlike `committeeId` and `wethAddress`, `requestNonce` is not explicitly set, which could cause issues in upgradeable instances. It is recommended to initialize `requestNonce` within `initialize()`.


  **Link**: [Issue #38](https://github.com/hats-finance/Most--Aleph-Zero-Bridge-0xab7c1d45ae21e7133574746b2985c58e0ae2e61d/issues/38)


- **Incorrect Event Emitted During Ownership Acceptance in Ink-Based Contracts**

  In `ink`-based contracts, the `accept_ownership()` function emits the wrong event (`TransferOwnershipInitiated`) when ownership is accepted by the pending owner. This misleads users and creates errors when checking events for ownership acceptance. The correct event should be `TransferOwnershipAccepted`, which clarifies that the pending owner has accepted ownership.


  **Link**: [Issue #39](https://github.com/hats-finance/Most--Aleph-Zero-Bridge-0xab7c1d45ae21e7133574746b2985c58e0ae2e61d/issues/39)



## Conclusion

The audit report on Most: Aleph Zero Bridge conducted through a Hats.finance competition demonstrates a proactive and decentralized approach to enhancing smart contract security. With a focus on efficient fund allocation and high-quality vulnerability identification, the competition attracted numerous skilled auditors. Key findings from the audit include:

1. **Medium Severity Issues**: Key issues like WETH conversion leading to transfer failures and incorrect threshold values for committee signatures were identified.
2. **Low Severity Issues**: These included state discrepancies due to unupdated `committee_sizes` and functional failures in the migration contract's upgrade process.
3. **Minor Severity Issues**: Problems mentioned include uninitialized contract vulnerabilities, lack of zero address checks, improper pocket money balance decrements, immutable gas variables, protection mechanisms for rewards, and incorrect event emissions in ink-based contracts.

The audit resulted in a payout of $23,752 distributed among 12 participants, demonstrating the effectiveness of decentralized audit competitions in securing DeFi protocols like Most: Aleph Zero Bridge.

## Disclaimer


This report does not assert that the audited contracts are completely secure. Continuous review and comprehensive testing are advised before deploying critical smart contracts.


The Most: Aleph Zero Bridge audit competition illustrates the collaborative effort in identifying and rectifying potential vulnerabilities, enhancing the overall security and functionality of the platform.


Hats.finance does not provide any guarantee or warranty regarding the security of this project. Smart contract software should be used at the sole risk and responsibility of users.

