// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "../lib/coprocessor-base-contract/src/BaseContract.sol";

contract MyContract is BaseContract {
    constructor(address _coprocessorAddress, bytes32 _machineHash) BaseContract(_coprocessorAddress, _machineHash) {}

    function runExecution(bytes calldata input) external {
        callCoprocessor(input);
    }

    // Add your logic here
}
