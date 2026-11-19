
$outputPath = "samples/large-access.log"

$writer = [System.IO.StreamWriter]::new(
    $outputPath,
    $false,
    [System.Text.Encoding]::UTF8
)

$endpoints = @(
    "/api/users",
    "/api/products",
    "/api/login",
    "/api/orders",
    "/api/dashboard"
)

$statusCodes = @(200, 201, 200, 404, 500)

try {
    for ($i = 1; $i -le 100000; $i++) {

        $ip = "192.168.1.$(($i % 254) + 1)"
        $endpoint = $endpoints[$i % $endpoints.Length]
        $status = $statusCodes[$i % $statusCodes.Length]
        $latency = 50 + (($i * 17) % 950)
        $size = 500 + (($i * 13) % 5000)

        $timestamp = "18/Nov/2026:08:10:00"

        $line = "$ip - - [$timestamp] `"GET $endpoint HTTP/1.1`" $status $size $latency"

        $writer.WriteLine($line)
    }
}
finally {
    $writer.Dispose()
}

Write-Host "Successfully generated 100,000 log entries."
