import {
  computeSvg,
  Peg,
  Yarn,
  PatherConfig,
  EarlyStopConfig,
} from "strandify-wasm";

self.onmessage = async (event) => {
  try {
    const { imageData, pegs, options } = event.data;

    const earlyStopConfig = new EarlyStopConfig(
      options.earlyStop.lossThreshold
        ? options.earlyStop.lossThreshold
        : undefined,
      options.earlyStop.maxCount,
    );

    const patherYarn = new Yarn(
      options.pather.width,
      options.pather.opacity,
      0,
      0,
      0,
    );

    const patherConfig = new PatherConfig(
      options.pather.iterations,
      patherYarn,
      earlyStopConfig,
      5,
      options.pather.skipPegWithin,
      options.pather.beamWidth,
    );
    // Convert hex color to RGB
    const color = options.render.color;
    const r = parseInt(color.slice(1, 3), 16);
    const g = parseInt(color.slice(3, 5), 16);
    const b = parseInt(color.slice(5, 7), 16);

    const renderYarn = new Yarn(
      options.render.width,
      options.render.opacity,
      r,
      g,
      b,
    );

    const computePegs = pegs.map(
      (peg: { x: number; y: number }) => new Peg(peg.x, peg.y),
    );

    const svg = computeSvg(imageData, computePegs, patherConfig, renderYarn);
    self.postMessage({ svg: svg });
  } catch (error) {
    let errorMessage = "An unknown error occurred.";
    if (error instanceof Error) {
      errorMessage = error.message; // If it's a standard Error object
    } else if (typeof error === "string") {
      errorMessage = error; // If the error is a string
    }

    self.postMessage({ error: errorMessage });
  }
};
