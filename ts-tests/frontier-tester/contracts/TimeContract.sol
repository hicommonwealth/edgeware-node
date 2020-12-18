pragma solidity ^0.5.0;

contract TimeContract {
  uint timeCreated;
  uint lastChecked;

  constructor() public {
    timeCreated = now;
  }

  modifier didNotEnd() {
    require(now <= (timeCreated + 1 days));
    _;
  }

  function timeBeforeEnd() public didNotEnd returns (uint) {
    lastChecked = now;
    return lastChecked;
  }

  function viewTimeCreated() public view returns (uint) {
    return timeCreated;
  }

  function viewNow() public view returns (uint) {
    return now;
  }

  function viewBlockTimestamp() public view returns (uint) {
    return block.timestamp;
  }
}
