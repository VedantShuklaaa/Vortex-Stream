const {
	VortexStream
  } = require("vortex-stream");
  
  const stream =
	new VortexStream();
  
  stream.trades(
	"binance",
	"BTCUSDT",
	console.log
  );
  
  setInterval(() => {}, 1000);