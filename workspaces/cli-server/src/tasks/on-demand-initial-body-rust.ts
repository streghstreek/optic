import {
  InitialBodiesWorkerRust,
  InitialBodiesWorkerConfig,
  LearnedBodies,
} from '@useoptic/cli-shared/build/diffs/initial-bodies-worker-rust';

export { LearnedBodies };

export class OnDemandInitialBodyRust {
  private worker: InitialBodiesWorkerRust;

  constructor(config: InitialBodiesWorkerConfig) {
    this.worker = new InitialBodiesWorkerRust(config);
  }

  async run(): Promise<LearnedBodies> {
    return this.worker.run();
  }
}
