// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract HeavyOps {
    uint256 public value;

    function doWork(uint256 input) public {
        // Multiple expensive opcodes: SSTORE, ADD, MUL, SHA3
        value = value + input;
        uint256 hash = uint256(keccak256(abi.encodePacked(value, input)));
        value = value ^ hash;
    }
}
