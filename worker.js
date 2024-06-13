self.onmessage = function(event) {
    console.log('Message received from main script:', event.data);
    self.postMessage('Hello from the worker!');
};