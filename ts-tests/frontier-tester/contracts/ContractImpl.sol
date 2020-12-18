pragma solidity ^0.5.0;

import './IContract.sol';

contract ContractImpl is IContract {
	function doSomething() external view returns (bool) {
		return true;
	}
}
