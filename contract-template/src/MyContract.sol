// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "../lib/coprocessor-base-contract/src/BaseContract.sol";

contract MyContract is BaseContract {
    constructor(address _coprocessorAddress, bytes32 _machineHash) BaseContract(_coprocessorAddress, _machineHash) {}

    function runExecution(bytes calldata input) external {
        callCoprocessor(input);
    }

    function handleNotice(bytes calldata notice) internal override {
        // Add logic for handling callback from co-processor containing notices.
    }

    // Add your other app logic here
}
