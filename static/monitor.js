// Data Buffers (Hold last 50 data points)
const maxPoints = 50;
const labels = new Array(maxPoints).fill("");


// Chart 1 Data: Load (%)
const cpuData = new Array(maxPoints).fill(0);
const ramData = new Array(maxPoints).fill(0);
const diskData = new Array(maxPoints).fill(0);

// Chart 2 Data: Network (KB/s)
const netRxData = new Array(maxPoints).fill(0);
const netTxData = new Array(maxPoints).fill(0);

// Chart 3: GPU Data (if applicable)
const gpuUtilData = new Array(maxPoints).fill(0);
const gpuTempData = new Array(maxPoints).fill(0);
const gpuMemData = new Array(maxPoints).fill(0);
let gpuDetected = false;

// --- CHART 1 CONFIG (Load) ---
const loadCtx = document.getElementById('loadChart').getContext('2d');
const loadChart = new Chart(loadCtx, {
    type: 'line',
    data: {
        labels: labels,
        datasets: [
            {
                label: 'CPU Usage %',
                data: cpuData,
                borderColor: '#ff6384', // Red
                tension: 0.3,
                borderWidth: 2,
                pointRadius: 0
            },
            {
                label: 'RAM Usage %', // Note: We might need to normalize this to % in the future
                data: ramData,
                borderColor: '#36a2eb', // Blue
                tension: 0.3,
                borderWidth: 2,
                pointRadius: 0
            },
            {
                label: 'Disk Usage %',
                data: diskData,
                borderColor: '#4bc0c0', // Green
                tension: 0.3,
                borderWidth: 2,
                pointRadius: 0,
                borderDash: [5, 5] // Dashed line for Disk
            }
        ]
    },
    options: {
        responsive: true,
        animation: false,
        scales: {
            y: {
                beginAtZero: true,
                max: 100, // Fixed scale for %
                grid: { color: '#333' }
            },
            x: { display: false }
        },
        plugins: { legend: { labels: { color: '#ccc' } } }
    }
});

// --- CHART 2 CONFIG (Network) ---
const netCtx = document.getElementById('netChart').getContext('2d');
const netChart = new Chart(netCtx, {
    type: 'line',
    data: {
        labels: labels,
        datasets: [
            {
                label: 'Download (KB/s)',
                data: netRxData,
                borderColor: '#9966ff', // Purple
                backgroundColor: 'rgba(153, 102, 255, 0.2)',
                fill: true,
                tension: 0.3,
                borderWidth: 2,
                pointRadius: 0
            },
            {
                label: 'Upload (KB/s)',
                data: netTxData,
                borderColor: '#ff9f40', // Orange
                tension: 0.3,
                borderWidth: 2,
                pointRadius: 0
            }
        ]
    },
    options: {
        responsive: true,
        animation: false,
        scales: {
            y: {
                beginAtZero: true,
                // No max - let it scale dynamically with traffic
                grid: { color: '#333' }
            },
            x: { display: false }
        },
        plugins: { legend: { labels: { color: '#ccc' } } }
    }
});

// --- CHART 3 CONFIG (GPU Util) ---
const gpuUtilCtx = document.getElementById('gpuUtilChart').getContext('2d');
const gpuUtilChart = new Chart(gpuUtilCtx, {
    type: 'line',
    data: {
        labels: labels,
        datasets: [{
            label: 'GPU Core Load %',
            data: gpuUtilData,
            borderColor: '#76b900', // NVIDIA Green
            backgroundColor: 'rgba(118, 185, 0, 0.2)',
            fill: true,
            tension: 0.3,
            borderWidth: 2,
            pointRadius: 0
        }]
    },
    options: {
        responsive: true,
        animation: false,
        scales: {
            y: { beginAtZero: true, max: 100, grid: { color: '#333' } },
            x: { display: false }
        },
        plugins: { legend: { labels: { color: '#ccc' } } }
    }
});

// --- CHART 4 CONFIG (GPU Mem/Temp) ---
const gpuMemCtx = document.getElementById('gpuMemChart').getContext('2d');
const gpuMemChart = new Chart(gpuMemCtx, {
    type: 'line',
    data: {
        labels: labels,
        datasets: [
            {
                label: 'VRAM Used (MB)',
                data: gpuMemData,
                borderColor: '#ffce56', // Yellow
                yAxisID: 'y',
                tension: 0.3,
                borderWidth: 2,
                pointRadius: 0
            },
            {
                label: 'Temp (°C)',
                data: gpuTempData,
                borderColor: '#ff6384', // Red
                yAxisID: 'y1',
                tension: 0.3,
                borderWidth: 2,
                pointRadius: 0
            }
        ]
    },
    options: {
        responsive: true,
        animation: false,
        scales: {
            y: {
                type: 'linear',
                display: true,
                position: 'left',
                beginAtZero: true,
                grid: { color: '#333' }
            },
            y1: {
                type: 'linear',
                display: true,
                position: 'right',
                beginAtZero: true,
                grid: { drawOnChartArea: false } // Hide grid lines for second axis
            },
            x: { display: false }
        },
        plugins: { legend: { labels: { color: '#ccc' } } }
    }
});

// --- SSE CONNECTION ---
const eventSource = new EventSource("/events");
const statusIndicator = document.querySelector('.status-indicator');

eventSource.onopen = () => {
    statusIndicator.innerText = "🟢 System Online - Streaming Data";
    statusIndicator.style.color = "#4bc0c0";
};

eventSource.onerror = () => {
    statusIndicator.innerText = "🔴 Connection Lost - Reconnecting...";
    statusIndicator.style.color = "#ff6384";
};

eventSource.onmessage = (event) => {
    const metric = JSON.parse(event.data);

    // Update Data Arrays
    cpuData.push(metric.cpu_usage);
    cpuData.shift();

    ramData.push(metric.ram_usage_mb / 320); 
    ramData.shift();

    diskData.push(metric.disk_usage_percent);
    diskData.shift();

    netRxData.push(metric.net_rx_kb);
    netRxData.shift();
    netTxData.push(metric.net_tx_kb);
    netTxData.shift();

    // Update GPU Arrays
    gpuUtilData.push(metric.gpu_usage);
    gpuUtilData.shift();
    gpuTempData.push(metric.gpu_temp);
    gpuTempData.shift();
    gpuMemData.push(metric.gpu_vram_used_mb);
    gpuMemData.shift();

    // --- DETECTION LOGIC ---
    // If we see valid data (temp > 0 is the most reliable indicator), switch modes
    if (!gpuDetected && metric.gpu_temp > 0) {
        gpuDetected = true;
        document.getElementById('no-gpu-message').style.display = 'none';
        document.getElementById('gpu-stats-container').style.display = 'grid'; // Show grid
    }

    // Refresh Charts
    loadChart.update();
    netChart.update();

    // Only update GPU charts if visible to save performance
    if (gpuDetected) {
        gpuUtilChart.update();
        gpuMemChart.update();
    }
};