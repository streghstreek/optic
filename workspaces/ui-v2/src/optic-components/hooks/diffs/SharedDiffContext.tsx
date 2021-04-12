import React, { useContext, useState } from 'react';
import {
  IPendingEndpoint,
  IUndocumentedUrl,
  newSharedDiffMachine,
  SharedDiffStateContext,
} from './SharedDiffState';
// @ts-ignore
import * as shortId from 'shortid';
import { useMachine } from '@xstate/react';
import { PathComponentAuthoring } from '../../diffs/UndocumentedUrl';
import { useAllRequestsAndResponses } from './useAllRequestsAndResponses';
import { IEndpoint, useEndpoints } from '../useEndpointsHook';
import { IRequestBody, IResponseBody } from '../useEndpointBodyHook';
import { IgnoreRule } from '../../../lib/ignore-rule';
import { CurrentSpecContext } from '../../../lib/Interfaces';
import { IUnrecognizedUrl } from '@useoptic/spectacle';
import { newRandomIdGenerator } from '../../../lib/domain-id-generator';
import { ParsedDiff } from '../../../lib/parse-diff';

export const SharedDiffReactContext = React.createContext({});

type ISharedDiffContext = {
  context: SharedDiffStateContext;
  documentEndpoint: (pattern: string, method: string) => string;
  addPathIgnoreRule: (rule: string) => void;
  addDiffHashIgnore: (diffHash: string) => void;
  persistWIPPattern: (
    path: string,
    method: string,
    components: PathComponentAuthoring[],
  ) => void;
  getPendingEndpointById: (id: string) => IPendingEndpoint | undefined;
  wipPatterns: { [key: string]: PathComponentAuthoring[] };
  stageEndpoint: (id: string) => void;
  discardEndpoint: (id: string) => void;
  approveCommandsForDiff: (diffHash: string, commands: any[]) => void;
  pendingEndpoints: IPendingEndpoint[];
  isDiffHandled: (diffHash: string) => boolean;
  currentSpecContext: CurrentSpecContext;
};

type SharedDiffStoreProps = {
  endpoints: IEndpoint[];
  requests: IRequestBody[];
  responses: IResponseBody[];
  diffs: any;
  urls: IUnrecognizedUrl[];
  children?: any;
};

export const SharedDiffStore = (props: SharedDiffStoreProps) => {
  const currentSpecContext: CurrentSpecContext = {
    currentSpecEndpoints: props.endpoints,
    currentSpecRequests: props.requests,
    currentSpecResponses: props.responses,
    domainIds: newRandomIdGenerator(),
  };
  //@dev here is where the diff output needs to go
  const [state, send]: any = useMachine(() =>
    newSharedDiffMachine(
      currentSpecContext,
      props.diffs.map((i: any) => new ParsedDiff(i[0], i[1])),
      props.urls.map((i) => ({ ...i })),
    ),
  );
  const context: SharedDiffStateContext = state.context;
  const [wipPatterns, setWIPPatterns] = useState<{
    [key: string]: PathComponentAuthoring[];
  }>({});

  const value: ISharedDiffContext = {
    context,
    documentEndpoint: (pattern: string, method: string) => {
      const uuid = shortId.generate();
      send({ type: 'DOCUMENT_ENDPOINT', pattern, method, pendingId: uuid });
      return uuid;
    },
    stageEndpoint: (id: string) =>
      send({ type: 'PENDING_ENDPOINT_STAGED', id }),
    discardEndpoint: (id: string) =>
      send({ type: 'PENDING_ENDPOINT_DISCARDED', id }),
    addPathIgnoreRule: (rule: string) => {
      send({ type: 'ADD_PATH_IGNORE_RULE', rule });
    },
    getPendingEndpointById: (id: string) => {
      return context.pendingEndpoints.find((i) => i.id === id);
    },
    pendingEndpoints: context.pendingEndpoints,
    isDiffHandled: (diffHash: string) => {
      return (
        context.choices.approvedSuggestions.hasOwnProperty(diffHash) ||
        context.browserDiffHashIgnoreRules.includes(diffHash)
      );
    },
    approveCommandsForDiff: (diffHash: string, commands: any[]) => {
      send({ type: 'COMMANDS_APPROVED_FOR_DIFF', diffHash, commands });
    },
    addDiffHashIgnore: (diffHash: string) =>
      send({ type: 'ADD_DIFF_HASH_IGNORE', diffHash }),
    persistWIPPattern: (
      path: string,
      method: string,
      components: PathComponentAuthoring[],
    ) =>
      setWIPPatterns((obj) => ({
        ...obj,
        [path + method]: components,
      })),
    wipPatterns,
    currentSpecContext,
  };

  return (
    <SharedDiffReactContext.Provider value={value}>
      {props.children}
    </SharedDiffReactContext.Provider>
  );
};

export function useSharedDiffContext() {
  return useContext(SharedDiffReactContext) as ISharedDiffContext;
}
