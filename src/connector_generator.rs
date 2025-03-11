use crate::config::Config;
use crate::scanner::{EndpointInfo, EndpointType, Protocol, ScanResults};
use handlebars::Handlebars;
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub fn generate_connectors(
    scan_results: &ScanResults,
    output_dir: &str,
    config: &Config,
) -> Result<usize, Box<dyn Error>> {
    // Create output directory if it doesn't exist
    let output_path = Path::new(output_dir);
    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    }

    // Initialize template engine
    let mut handlebars = Handlebars::new();

    // Register templates
    register_templates(&mut handlebars)?;

    // Generate a connector for each endpoint
    let mut generated_count = 0;
    for endpoint in scan_results {
        let template_name = select_template_for_endpoint(endpoint);
        let connector_data = prepare_connector_data(endpoint, config);

        // Generate connector filename
        let connector_id = Uuid::new_v4().to_string();
        let filename = format!("mcp_connector_{}.json", connector_id);
        let file_path = output_path.join(filename);

        // Render template
        let rendered = handlebars.render(template_name, &connector_data)?;

        // Write connector to file
        fs::write(file_path, rendered)?;
        generated_count += 1;

        log::debug!(
            "Generated connector for {} ({}:{})",
            endpoint.address,
            endpoint.port,
            connector_id
        );
    }

    // Generate manifest file
    generate_manifest(scan_results, output_path)?;

    Ok(generated_count)
}

fn register_templates(handlebars: &mut Handlebars) -> Result<(), Box<dyn Error>> {
    // Embed templates in the binary for easier distribution
    // In a real-world scenario, you might want to load these from files

    // API endpoint template
    handlebars.register_template_string(
        "api_connector",
        r#"{
  "connector_id": "{{connector_id}}",
  "name": "{{name}}",
  "type": "api",
  "version": "1.0",
  "endpoint": {
    "protocol": "{{protocol}}",
    "host": "{{host}}",
    "port": {{port}},
    "base_path": "{{base_path}}",
    "authentication": {
      "type": "{{auth_type}}",
      "params": {}
    }
  },
  "polling_interval": {{polling_interval}},
  "transformations": [
    {
      "type": "jq",
      "expression": "{{default_transformation}}"
    }
  ],
  "metadata": {{metadata}}
}"#,
    )?;

    // Log source template
    handlebars.register_template_string(
        "log_connector",
        r#"{
  "connector_id": "{{connector_id}}",
  "name": "{{name}}",
  "type": "log",
  "version": "1.0",
  "source": {
    "protocol": "{{protocol}}",
    "host": "{{host}}",
    "port": {{port}},
    "format": "{{format}}"
  },
  "filters": [
    {
      "field": "severity",
      "operator": ">=",
      "value": "{{min_severity}}"
    }
  ],
  "parsing": {
    "pattern": "{{log_pattern}}",
    "time_format": "{{time_format}}"
  },
  "metadata": {{metadata}}
}"#,
    )?;

    // Database source template
    handlebars.register_template_string(
        "database_connector",
        r#"{
  "connector_id": "{{connector_id}}",
  "name": "{{name}}",
  "type": "database",
  "version": "1.0",
  "connection": {
    "type": "{{db_type}}",
    "host": "{{host}}",
    "port": {{port}},
    "authentication": {
      "type": "{{auth_type}}",
      "params": {}
    }
  },
  "queries": [
    {
      "name": "default_health_check",
      "sql": "{{health_query}}",
      "interval": {{polling_interval}}
    }
  ],
  "metadata": {{metadata}}
}"#,
    )?;

    // Message queue template
    handlebars.register_template_string(
        "message_queue_connector",
        r#"{
  "connector_id": "{{connector_id}}",
  "name": "{{name}}",
  "type": "message_queue",
  "version": "1.0",
  "connection": {
    "protocol": "{{protocol}}",
    "host": "{{host}}",
    "port": {{port}},
    "authentication": {
      "type": "{{auth_type}}",
      "params": {}
    }
  },
  "topics": [
    "{{default_topic}}"
  ],
  "processing": {
    "type": "{{processing_type}}",
    "config": {}
  },
  "metadata": {{metadata}}
}"#,
    )?;

    // Generic connector template
    handlebars.register_template_string(
        "generic_connector",
        r#"{
  "connector_id": "{{connector_id}}",
  "name": "{{name}}",
  "type": "generic",
  "version": "1.0",
  "connection": {
    "protocol": "{{protocol}}",
    "host": "{{host}}",
    "port": {{port}}
  },
  "health_check": {
    "type": "tcp_connection",
    "interval": {{polling_interval}}
  },
  "metadata": {{metadata}}
}"#,
    )?;

    Ok(())
}

fn select_template_for_endpoint(endpoint: &EndpointInfo) -> &'static str {
    match endpoint.endpoint_type {
        EndpointType::API => "api_connector",
        EndpointType::Log => "log_connector",
        EndpointType::Database => "database_connector",
        EndpointType::MessageQueue => "message_queue_connector",
        EndpointType::Other(_) => "generic_connector",
    }
}

fn prepare_connector_data(endpoint: &EndpointInfo, config: &Config) -> Value {
    let connector_id = Uuid::new_v4().to_string();
    let host = &endpoint.address;
    let port = endpoint.port;

    // Convert metadata to JSON
    let metadata_json = json!(endpoint.metadata);

    // Base connector data common to all types
    let mut data = json!({
        "connector_id": connector_id,
        "name": format!("MCP Connector for {}:{}", host, port),
        "host": host,
        "port": port,
        "metadata": metadata_json,
        "polling_interval": config.connector.default_polling_interval
    });

    // Add specific fields based on endpoint type
    match endpoint.endpoint_type {
        EndpointType::API => {
            let protocol_str = match endpoint.protocol {
                Protocol::HTTP => "http",
                Protocol::HTTPS => "https",
                _ => "http",
            };

            data["protocol"] = json!(protocol_str);
            data["base_path"] = json!("/");
            data["auth_type"] = json!("none");
            data["default_transformation"] = json!("."); // Identity transformation
        }

        EndpointType::Log => {
            let protocol_str = match endpoint.protocol {
                Protocol::TCP => "tcp",
                Protocol::UDP => "udp",
                _ => "tcp",
            };

            data["protocol"] = json!(protocol_str);
            data["format"] = json!("syslog");
            data["min_severity"] = json!("warning");
            data["log_pattern"] =
                json!("%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:severity} %{GREEDYDATA:message}");
            data["time_format"] = json!("yyyy-MM-dd'T'HH:mm:ss.SSSZ");
        }

        EndpointType::Database => {
            // Guess database type from port
            let db_type = match port {
                3306 => "mysql",
                5432 => "postgresql",
                1521 => "oracle",
                27017 => "mongodb",
                _ => "unknown",
            };

            data["db_type"] = json!(db_type);
            data["auth_type"] = json!("basic");
            data["health_query"] = json!(if db_type == "mongodb" {
                "{\"serverStatus\": 1}"
            } else {
                "SELECT 1"
            });
        }

        EndpointType::MessageQueue => {
            let protocol_str = match endpoint.protocol {
                Protocol::MQTT => "mqtt",
                Protocol::AMQP => "amqp",
                _ => "mqtt",
            };

            data["protocol"] = json!(protocol_str);
            data["auth_type"] = json!("none");
            data["default_topic"] = json!("mcp/events");
            data["processing_type"] = json!("passthrough");
        }

        EndpointType::Other(ref service_type) => {
            let protocol_str = match endpoint.protocol {
                Protocol::TCP => "tcp",
                Protocol::UDP => "udp",
                Protocol::HTTP => "http",
                Protocol::HTTPS => "https",
                _ => "tcp",
            };

            data["protocol"] = json!(protocol_str);
            data["service_type"] = json!(service_type);
        }
    }

    data
}

fn generate_manifest(scan_results: &ScanResults, output_path: &Path) -> Result<(), Box<dyn Error>> {
    let manifest = json!({
        "manifest_version": "1.0",
        "generated_timestamp": chrono::Utc::now().to_rfc3339(),
        "connector_count": scan_results.len(),
        "endpoint_summary": {
            "api_count": scan_results.iter().filter(|e| matches!(e.endpoint_type, EndpointType::API)).count(),
            "log_count": scan_results.iter().filter(|e| matches!(e.endpoint_type, EndpointType::Log)).count(),
            "database_count": scan_results.iter().filter(|e| matches!(e.endpoint_type, EndpointType::Database)).count(),
            "message_queue_count": scan_results.iter().filter(|e| matches!(e.endpoint_type, EndpointType::MessageQueue)).count(),
            "other_count": scan_results.iter().filter(|e| matches!(e.endpoint_type, EndpointType::Other(_))).count(),
        }
    });

    let manifest_path = output_path.join("mcp_manifest.json");
    fs::write(manifest_path, serde_json::to_string_pretty(&manifest)?)?;

    Ok(())
}
