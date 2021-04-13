import Tap = require('tap');
import { makeSpectacle } from '../src';
import * as OpticEngine from '@useoptic/diff-engine-wasm/engine/build';
import { InMemoryOpticContextBuilder } from '../src/in-memory';

Tap.test('spectacle batchCommits query', async (test) => {
  const opticContext = await InMemoryOpticContextBuilder.fromEvents(OpticEngine, [])
  const spectacle = await makeSpectacle(opticContext);

  const results = await spectacle({
    query: `{
        batchCommits {
          createdAt
          batchId
        }
      }`,
    variables: {}
  });
  test.matchSnapshot(results);
});