self.onmessage = function(event) {
    console.log(`Main's thread message: `, event.data);
    self.postMessage('Worker reporting!');
};
