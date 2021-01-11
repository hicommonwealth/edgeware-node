# Ensure edgeware is up
if ! curl -s localhost:9615/metrics -o /dev/null; then
  echo "Could not query edgeware metrics endpoint"
  exit 1
fi

# Install prometheus
sudo apt-get install prometheus -y

# Configure prometheus
sudo echo "" >> /etc/prometheus/prometheus.yml
sudo echo "  - job_name: substrate_node" >> /etc/prometheus/prometheus.yml
sudo echo "    scrape_interval: 5s" >> /etc/prometheus/prometheus.yml
sudo echo "    static_configs:" >> /etc/prometheus/prometheus.yml
sudo echo "      - targets: ['127.0.0.1:9615']" >> /etc/prometheus/prometheus.yml

if ! test -f /usr/bin/prometheus; then
  echo "Prometheus installed in an unexpected location."
  echo "Binary should be '/usr/bin/prometheus'"
  exit 1
fi

# Create service
sudo tee -a /etc/systemd/system/prometheus.service << END
[Unit]
  Description=Prometheus Monitoring
  Wants=network-online.target
  After=network-online.target

[Service]
  User=prometheus
  Group=prometheus
  Type=simple
  ExecStart=/usr/bin/prometheus \
  --config.file /etc/prometheus/prometheus.yml \
  --storage.tsdb.path /var/lib/prometheus/ \
  --web.console.templates=/etc/prometheus/consoles \
  --web.console.libraries=/etc/prometheus/console_libraries
  ExecReload=/bin/kill -HUP $MAINPID

[Install]
  WantedBy=multi-user.target
END

# Enable and run the service
sudo systemctl daemon-reload
sudo systemctl enable prometheus
sudo systemctl start prometheus

# Verify prometheus installed
if ! curl -s localhost:9090 -o /dev/null; then
  echo "Could not query prometheus!"
  exit 1
else
  echo "Prometheus installed successfully!"
  exit 0
fi
