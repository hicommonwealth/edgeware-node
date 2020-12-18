pragma solidity ^0.5.0;

contract OwnerContract {
  address owner;
  constructor() public {
    owner = msg.sender;
  }

  function makeCall() public view isOwner returns(bool) {
    return true;
  }

  modifier isOwner() {
    require(msg.sender == owner, "only owner");
    _;
  }
}
