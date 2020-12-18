pragma solidity ^0.5.0;

import './ValueContract.sol';

contract Create2Factory {
  address addr;

  constructor() public { }

  function deploy(uint256 salt) public {
    address addrLocal;
    bytes memory bytecode = type(ValueContract).creationCode;
    assembly {
      addrLocal := create2(0, add(bytecode, 0x20), mload(bytecode), salt)
      if iszero(extcodesize(addrLocal)) {
        revert(0, 0)
      }
    }
    addr = addrLocal;
  }

  function viewAddr() public view returns (address) {
    return addr;
  }
}
