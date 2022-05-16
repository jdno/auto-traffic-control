import {
  getCredentials,
  AtcServiceClient,
  GetVersionRequest,
  GetVersionResponse,
  ServiceError,
} from "auto-traffic-control";

const atcService = new AtcServiceClient("localhost:4747", getCredentials());

atcService.getVersion(
  new GetVersionRequest(),
  (err: ServiceError | null, response: GetVersionResponse) => {
    if (err != null) {
      throw err;
    }

    const version = response.getVersion();

    if (version != null) {
      let versionString = [
        version.getMajor(),
        version.getMinor(),
        version.getPatch(),
      ].join(".");

      if (version.getPre() !== "") {
        versionString = versionString.concat(version.getPre());
      }

      console.log(`Auto Traffic Control is running version '${versionString}'`);
    } else {
      throw new Error("Requesting the version returned an empty response.");
    }
  }
);
