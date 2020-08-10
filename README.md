# Dissect &emsp; [![Latest Version]][crates.io] ![Build Status] ![Quality Checks] ![License Checks] ![Security Checks] [![Code Coverage]][codecov.io]

[Build Status]: https://github.com/wayfair-tremor/dissect/workflows/Tests/badge.svg
[Quality Checks]: https://github.com/wayfair-tremor/dissect/workflows/Checks/badge.svg
[License Checks]: https://github.com/wayfair-tremor/dissect/workflows/License%20audit/badge.svg
[Security Checks]: https://github.com/wayfair-tremor/dissect/workflows/Security%20audit/badge.svg
[Code Coverage]: https://codecov.io/gh/wayfair-tremor/dissect/branch/main/graph/badge.svg
[codecov.io]: https://codecov.io/gh/wayfair-tremor/dissect
[Latest Version]: https://img.shields.io/crates/v/dissect.svg
[crates.io]: https://crates.io/crates/dissect

**dissect parser**

---

Dissect parsing inspired by logstash's dissect plugin.

Parses a string into a map. 

## Use as a library

The dissect parser was designed so that KV style parsing could be embedded into [tremor](https://www.tremor.rs)'s [scripting](https://docs.tremor.rs/tremor-script/) language for [extract](https://docs.tremor.rs/tremor-script/extractors/dissect/) operations.

The parser can also be used standalone. A fairly gnarly example of parsing logs
from this libraries tests illustrates better than words can:

```rust
  let pattern = r#"%{syslog_timestamp} %{syslog_hostname} %{?syslog_prog}: %{syslog_program_aux}[%{syslog_pid:int}] %{request_unix_time} %{request_timestamp} %{request_elapsed_time} %{server_addr}:%{server_port:int} %{remote_addr}:%{remote_port:int} "%{response_content_type}" %{response_content_length} %{request_status} %{bytes_sent} %{request_length} "%{url_scheme}" "%{http_host}" "%{request_method} %{request_url} %{request_protocol}" "%{http_referer}" "%{http_user_agent}" "%{http_x_forwarded_for}" "%{http_ttrue_client_ip}" "%{remote_user}" "%{is_bot}" "%{admin_user}" "%{http_via}" "%{response_location}" "%{set_cookie}" "%{http_cookie}" "%{moawsl_info}" "%{php_message}" "%{akamai_edgescape}" "%{uid_info}" "%{geoip_country}" "%{geoip_region}" "%{geoip_city}" "%{geoip_postal}" "%{geoip_dma}" "%{server_id}" "%{txid}" "%{hpcnt}" "%{client_accept}" "%{client_accept_charset}" "%{client_accept_encoding}" "%{client_accept_language}" "%{client_accept_datetime}" "%{client_pragma}" "%{client_transfer_encoding}" "%{client_attdeviceid}" "%{client_wap_profile}" %{weblog_end}"#;
  let p = lex("%{name}%{_}%{_(|)}%{age}");
  assert!(lex(pattern).is_ok());
  assert!(p.is_ok());
```

