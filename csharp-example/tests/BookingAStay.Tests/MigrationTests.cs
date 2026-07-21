using BookingAStay.ApplicationState.Db;
using Npgsql;

namespace BookingAStay.Tests;

[Collection(PostgreSqlCollection.Name)]
public sealed class MigrationTests
{
    private readonly PostgreSqlFixture _fixture;

    public MigrationTests(PostgreSqlFixture fixture)
    {
        _fixture = fixture;
    }

    [Fact]
    public async Task MigrationsApplyCleanlyAndAreIdempotent()
    {
        await using var dataSource = NpgsqlDataSource.Create(_fixture.ConnectionString);
        await ResetDatabaseAsync(dataSource);

        Migrations.Apply(_fixture.ConnectionString);
        var firstAppliedScripts = await LoadAppliedScriptsAsync(dataSource);
        var reservationsTableExists = await ReservationsTableExistsAsync(dataSource);

        Migrations.Apply(_fixture.ConnectionString);
        var secondAppliedScripts = await LoadAppliedScriptsAsync(dataSource);

        Assert.Equal(3, firstAppliedScripts.Count);
        Assert.Equal(firstAppliedScripts, secondAppliedScripts);
        Assert.True(reservationsTableExists);
    }

    private static async Task ResetDatabaseAsync(NpgsqlDataSource dataSource)
    {
        await using var connection = await dataSource.OpenConnectionAsync();
        await using var command = new NpgsqlCommand(
            """
            DROP SCHEMA public CASCADE;
            CREATE SCHEMA public;
            GRANT ALL ON SCHEMA public TO PUBLIC;
            """,
            connection);
        await command.ExecuteNonQueryAsync();
    }

    private static async Task<List<string>> LoadAppliedScriptsAsync(NpgsqlDataSource dataSource)
    {
        await using var connection = await dataSource.OpenConnectionAsync();
        await using var command = new NpgsqlCommand(
            "SELECT scriptname FROM schemaversions ORDER BY scriptname;",
            connection);
        await using var reader = await command.ExecuteReaderAsync();

        var scripts = new List<string>();
        while (await reader.ReadAsync())
        {
            scripts.Add(reader.GetString(0));
        }

        return scripts;
    }

    private static async Task<bool> ReservationsTableExistsAsync(NpgsqlDataSource dataSource)
    {
        await using var connection = await dataSource.OpenConnectionAsync();
        await using var command = new NpgsqlCommand(
            "SELECT to_regclass('public.reservations') IS NOT NULL;",
            connection);
        return (bool)(await command.ExecuteScalarAsync() ?? false);
    }
}
