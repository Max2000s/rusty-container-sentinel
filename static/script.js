async function fetchDockerVersion() {
	const res = await fetch('/api/version');
	const data = await res.json();
	document.getElementById('docker-version').textContent = data.version;
	document.getElementById('api-version').textContent = data.api_version;
}

let logsSocket = null;

function streamContainers() {
	const socket = new WebSocket("ws://localhost:8080/api/ws/containers");

	socket.onmessage = function(event) {
		const containers = JSON.parse(event.data);
		const containerList = document.getElementById('container-list');
		containerList.innerHTML = ''; // Clear previous contents

		containers.forEach(container => {
			const isRunning = container.state.toLowerCase() === 'running';
			const colorClass = isRunning => isRunning ? 'bg-green-600' : 'bg-red-600';

			const div = document.createElement('div');
			div.className = `${container.state.toLowerCase() === 'running' ? 'bg-green-600' : 'bg-red-600'} p-3 rounded-md shadow cursor-pointer`;
			div.innerHTML = `
        <div><b class="text-white">Name:</b> ${container.names.join(', ')}</div>
        <div><b class="text-white">ID:</b> ${container.id.substring(0, 12)}</div>
        <div><b class="text-white">Image:</b> ${container.image}</div>
        <div><b class="text-white">Status:</b> ${container.status}</div>
      `;

			// Attach click event clearly
			div.onclick = () => connectLogs(container.id, container.names[0]);

			containerList.appendChild(div);
		});
	};

	socket.onerror = function(err) {
		console.error('WebSocket Error:', err);
		document.getElementById('container-list').textContent = 'Failed to load containers.';
	};
}

function connectLogs(containerId, containerName) {
	if (logsSocket) logsSocket.close();

	document.getElementById('logs-section').classList.remove('hidden');
	document.getElementById('selected-container').textContent = containerName;
	document.getElementById('container-logs').textContent = '';

	logsSocket = new WebSocket(`ws://localhost:8080/api/ws/logs/${containerId}`);

	logsSocket.onmessage = function(event) {
		const logArea = document.getElementById('container-logs');
		logArea.textContent += event.data;
		logArea.scrollTop = logArea.scrollHeight;
	};

	logsSocket.onerror = function(err) {
		console.error('Log WebSocket Error:', err);
	};
}

// Initialize everything clearly
fetchDockerVersion();
streamContainers();
