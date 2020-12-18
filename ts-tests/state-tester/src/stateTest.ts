import { ApiPromise, Keyring } from '@polkadot/api';

// A specific test case
abstract class StateTest {
  // runDelay: # of blocks after upgrade to run the test
  constructor(
    // the publicly-displayable name of the test (usually set in the `super` call)
    public readonly name: string,

    // we use the accounts if we need to e.g. send a tx
    protected readonly accountSeeds: string[],
    protected readonly ss58Prefix: number,
  ) { }

  private _seedToAddress(s: string): string {
    // convert seeds to addresses for use in test cases
    return new Keyring({ ss58Format: this.ss58Prefix, type: 'sr25519' }).addFromUri(s).address;
  }

  // fetches account corresponding to a seed index
  protected account(idx: number) {
    return this._seedToAddress(this.accountSeeds[idx]);
  }

  // checks if the test has completed
  public isComplete(block: number): boolean {
    const finalTestBlock = Math.max(...Object.keys(this.actions).map((n) => +n));
    return block > finalTestBlock;
  }

  public readonly actions: { [block: number]: {
    name: string,
    fn: (api: ApiPromise) => Promise<void>,
  } };
}

export default StateTest;
