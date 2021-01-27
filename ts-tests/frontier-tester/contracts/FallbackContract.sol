pragma solidity ^0.5.0;

contract FallbackContract {
  constructor() public { }

	function () external payable {
		msg.sender.transfer(msg.value);
	}
}