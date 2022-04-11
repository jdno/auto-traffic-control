---
title: Auto Traffic Control
---

# `AtcService`

The `AtcService` provides information about **Auto Traffic Control** itself.

## Get Version

The current version of the game can be queried with the `GetVersion` request,
which returns the [`Version`](/docs/api/types#version) message type.

```protobuf
rpc GetVersion(GetVersionRequest) returns (GetVersionResponse);
```

### Request

```protobuf
message GetVersionRequest {}
```

### Response

```protobuf
message GetVersionResponse {
  Version version = 1;
}
```
