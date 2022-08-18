using Atc.V1;
using Grpc.Net.Client;
using static Atc.V1.AtcService;

var channel = GrpcChannel.ForAddress("http://localhost:4747");
var atcClient = new AtcServiceClient(channel);

var response = await atcClient.GetVersionAsync(new GetVersionRequest());

var version = response?.Version;

if (version != null)
{
    var versionString = string.Join(".", new[] {
        version.Major,
        version.Minor,
        version.Patch
    });

    if (!string.IsNullOrEmpty(version.Pre))
    {
        versionString += version.Pre;
    }

    Console.WriteLine($"Auto Traffic Control is running version '{versionString}'");
}
else
{
    throw new InvalidOperationException("Requesting the version returned an empty response.");
}
