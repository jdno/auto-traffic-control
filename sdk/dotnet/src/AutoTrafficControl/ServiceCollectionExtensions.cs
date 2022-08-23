using Microsoft.Extensions.DependencyInjection;
using static Atc.V1.AirplaneService;
using static Atc.V1.AtcService;
using static Atc.V1.EventService;
using static Atc.V1.GameService;
using static Atc.V1.MapService;

namespace AutoTrafficControl;

public static class ServiceCollectionExtensions
{
    /// <summary>
    /// Utility method to automatically register all clients in the dependency chain.
    /// Instead of newing up the service clients, you can request them wherever needed.
    /// </summary>
    /// <param name="services">The DI chain to add game clients to</param>
    /// <param name="serverUri">The default server host URI to use</param>
    /// <returns>The enriched DI chain</returns>
    public static IServiceCollection AddAutoTrafficControlClients(this IServiceCollection services, Uri serverUri)
    {
        services.AddGrpcClient<AirplaneServiceClient>(o =>
        {
            o.Address = serverUri;
        });
        services.AddGrpcClient<AtcServiceClient>(o =>
        {
            o.Address = serverUri;
        });
        services.AddGrpcClient<EventServiceClient>(o =>
        {
            o.Address = serverUri;
        });
        services.AddGrpcClient<GameServiceClient>(o =>
        {
            o.Address = serverUri;
        });
        services.AddGrpcClient<MapServiceClient>(o =>
        {
            o.Address = serverUri;
        });

        return services;
    }
}
