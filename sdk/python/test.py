import atc
import grpc


def main():
    with grpc.insecure_channel("localhost:4747") as channel:
        svc = atc.AtcServiceStub(channel)
        resp: atc.GetVersionResponse = svc.GetVersion(atc.GetVersionRequest())
        print(
            f"Server version {resp.version.major}.{resp.version.minor}.{resp.version.patch}.{resp.version.pre}"
        )


if __name__ == "__main__":
    main()
