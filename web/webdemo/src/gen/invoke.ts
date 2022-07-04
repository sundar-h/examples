/* eslint-disable */
export const protobufPackage = "invoker";

export interface CallRequest {
  /** Request message contains the name to be greeted */
  name: string;
}

export interface CallReply {
  /** Reply contains the greeting message */
  message: string;
}

export interface Invoker<Context extends DataLoaders> {
  Call(ctx: Context, request: CallRequest): Promise<CallReply>;
}

export interface DataLoaderOptions {
  cache?: boolean;
}

export interface DataLoaders {
  rpcDataLoaderOptions?: DataLoaderOptions;
  getDataLoader<T>(identifier: string, constructorFn: () => T): T;
}
