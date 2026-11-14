use csv::Writer;
use serde::Serialize;
use std::error::Error;
// We define our struct and derive Serialize so it can be converted to text.
#[derive(Serialize)]
struct SystemMetric {
    server_name: String,
    cpu_usage_percent: f32,
    active_connections: u32,
}
fn export_metrics() -> Result<(), Box<dyn Error>> {
    // We create a Writer. This automatically creates the file and a BufWriter.
    let mut csv_writer = Writer::from_path("metrics_export.csv")?;
    // We create a few records to simulate data gathering.
    let metric_one = SystemMetric {
        server_name: String::from("web_node_01"),
        cpu_usage_percent: 45.5,
        active_connections: 1050,
    };
    let metric_two = SystemMetric {
        server_name: String::from("database_node_01"),
        cpu_usage_percent: 88.2,
        active_connections: 420,
    };
    // We serialize the records.
    // The Writer automatically generates the header row based on the struct field names.
    // Then, it formats the data, adding commas and quotes automatically.
    csv_writer.serialize(metric_one)?;
    csv_writer.serialize(metric_two)?;
    // Just like a BufWriter, we must flush the internal buffers to ensure
    // all data is physically written to the hard drive before the function ends.
    csv_writer.flush()?;
    println!("Metrics successfully exported to CSV.");
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
