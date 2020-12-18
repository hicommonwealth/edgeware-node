pragma solidity ^0.5.0;

contract Hashing {
  function callKeccak256(bytes memory input) public pure returns(bytes32 result) {
    return keccak256(input);
  }

  function callRipemd160(bytes memory input) public pure returns(bytes20 result) {
    return ripemd160(input);
  }

  function callSha256(bytes memory input) public pure returns(bytes32 result) {
    return sha256(input);
  }
}
