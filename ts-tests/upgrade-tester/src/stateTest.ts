import { ApiPromise } from '@polkadot/api';
import { createTestPairs, TestKeyringMap } from '@polkadot/keyring/testingPairs';

// A specific test case
abstract class StateTest {
  protected started = false;
  protected completed = false;
  protected accounts: TestKeyringMap;

  // runDelay: # of blocks after upgrade to run the test
  constructor(
    // the publicly-displayable name of the test (usually set in the `super` call)
    public readonly name: string,
  ) {
    this.accounts = createTestPairs();
  }

  // checks if the test has completed
  public isComplete(): boolean {
    return this.completed;
  }

  public async before(api: ApiPromise): Promise<void> {
    this.started = true;
  }

  public async after(api: ApiPromise): Promise<void> {
    this.completed = true;
  }
}

export default StateTest;
