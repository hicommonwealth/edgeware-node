pragma solidity ^0.5.0;

contract StoreNum {
  uint8[] store;

  function addToStore(uint8 number) public {
    store.push(number);
  }

  function getStore() public view returns(uint8[] memory) {
    return store;
  }
}
