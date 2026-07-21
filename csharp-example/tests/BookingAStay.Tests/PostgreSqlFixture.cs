using BookingAStay.ApplicationState.Db;
using Testcontainers.PostgreSql;

namespace BookingAStay.Tests;

[CollectionDefinition(Name)]
public sealed class PostgreSqlCollection : ICollectionFixture<PostgreSqlFixture>
{
    public const string Name = "PostgreSQL";
}

public sealed class PostgreSqlFixture : IAsyncLifetime
{
    private readonly PostgreSqlContainer _container = new PostgreSqlBuilder("postgres:16-alpine")
        .WithDatabase("booking_a_stay_tests")
        .WithUsername("postgres")
        .WithPassword("postgres")
        .Build();

    public string ConnectionString => _container.GetConnectionString();

    public async Task InitializeAsync()
    {
        await _container.StartAsync();
        Migrations.Apply(ConnectionString);
    }

    public Task DisposeAsync()
    {
        return _container.DisposeAsync().AsTask();
    }
}
