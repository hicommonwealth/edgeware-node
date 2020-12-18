pragma solidity ^0.5.0;

contract SubContract {
  constructor() public payable { }
  function getAddress() external view returns (address ownAddress) {
    return address(this);
  }

  function getValue() external view returns (uint) {
    return address(this).balance;
  }
}

contract CreateContract {
  address public deployed;

  constructor() public { }

  function spawn() external returns (SubContract subAddress) {
    SubContract result = new SubContract();
    deployed = address(result);
    return result;
  }

  function spawnWithValue() external payable returns (SubContract subAddress) {
    SubContract result = (new SubContract).value(msg.value)();
    deployed = address(result);
    return result;
  }
}
