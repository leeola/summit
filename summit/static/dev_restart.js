const restartUri = "/dev/watch-restart";
var connectedOnce = false;
var attemptCount = 0;
var timeoutState = 0;
var eventSource;

function setupEventSource() {
  eventSource = new EventSource(restartUri);
  eventSource.onopen = function() {
    if (connectedOnce) {
      // Resetting the timeout state shouldn't be needed, but this is JS and Browserland so who
      // knows. By resetting, we prevent DX where the next disconnect takes a long time to reconnect.
      timeoutState = 0; 
      attemptCount = 0;
      location.reload();
    }
    connectedOnce = true;
  };
  eventSource.onerror = function(e) {
    eventSource.close();
    let timeout = timeoutState;
    if (timeoutState == 0) {
      timeout += 1500;
    }
    setTimeout(setupEventSource, timeout);
    console.log("attempting reconnect", {restartUri, timeout, attemptCount});
    // A super simple backoff, serving to make it a bit slower on reconnect attempts the longer
    // the server is down, but while also not growing crazy fast like a mul or pow.
    timeoutState += 50;
    attemptCount += 1;
  };
}

console.log("attempting initial connection", {restartUri});
setupEventSource();
