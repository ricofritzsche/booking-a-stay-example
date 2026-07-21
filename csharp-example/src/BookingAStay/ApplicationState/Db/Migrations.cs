using DbUp;

namespace BookingAStay.ApplicationState.Db;

public static class Migrations
{
    private const string ResourcePrefix = "BookingAStay.Migrations.";

    public static void Apply(string connectionString)
    {
        var result = DeployChanges.To
            .PostgresqlDatabase(connectionString)
            .WithScriptsEmbeddedInAssembly(
                typeof(Migrations).Assembly,
                resourceName => resourceName.StartsWith(ResourcePrefix, StringComparison.Ordinal))
            .WithTransactionPerScript()
            .LogToConsole()
            .Build()
            .PerformUpgrade();

        if (!result.Successful)
        {
            throw result.Error;
        }
    }
}
