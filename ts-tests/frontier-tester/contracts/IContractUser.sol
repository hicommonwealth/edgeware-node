pragma solidity ^0.5.0;

import './IContract.sol';

contract IContractUser {
	IContract created;

	constructor() public { }

	function linkContract(address a) public returns (bool) {
		created = IContract(a);
	}

	function doTheThing() public view returns (bool) {
		return created.doSomething();
	}
}
