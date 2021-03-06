import { credentials, ChannelCredentials, ServiceError } from "@grpc/grpc-js";

export * from "./atc/v1/airplane_pb";
export * from "./atc/v1/airplane_grpc_pb";
export * from "./atc/v1/atc_pb";
export * from "./atc/v1/atc_grpc_pb";
export * from "./atc/v1/event_pb";
export * from "./atc/v1/event_grpc_pb";
export * from "./atc/v1/game_pb";
export * from "./atc/v1/game_grpc_pb";
export * from "./atc/v1/map_pb";
export * from "./atc/v1/map_grpc_pb";
export * from "./atc/v1/tag_pb";

export { ChannelCredentials, ServiceError };

export function getCredentials(): ChannelCredentials {
  return credentials.createInsecure();
}
