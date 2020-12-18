pragma solidity ^0.5.0;

contract EventContract {
  event e(address c);

  function emitEvent() public {
    emit e(address(this));
  }
}
