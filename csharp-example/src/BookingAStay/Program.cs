using System.Text.Json;
using BookingAStay.ApplicationState.Db;
using BookingAStay.Api.Http;
using BookingAStay.Providers;
using Npgsql;

var builder = WebApplication.CreateBuilder(args);

if (builder.Environment.IsProduction())
{
    builder.Logging.ClearProviders();
    builder.Logging.AddJsonConsole(options => options.IncludeScopes = true);
}

var connectionString = builder.Configuration.GetConnectionString("ApplicationState")
    ?? throw new InvalidOperationException("Connection string 'ApplicationState' is required.");

builder.Services.AddSingleton(_ => new NpgsqlDataSourceBuilder(connectionString).Build());
builder.Services.AddSingleton<ProviderBundle>();
builder.Services.ConfigureHttpJsonOptions(options =>
{
    options.SerializerOptions.PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower;
});

var app = builder.Build();

app.UseRequestId();
app.UseRouting();
app.MapApiRoutes();

var skipMigrationsForTest = app.Environment.IsEnvironment("Testing")
    && string.Equals(
        app.Configuration["Testing:SkipMigrations"],
        "true",
        StringComparison.OrdinalIgnoreCase);

if (!skipMigrationsForTest)
{
    Migrations.Apply(connectionString);
}

app.Run();

public partial class Program;
