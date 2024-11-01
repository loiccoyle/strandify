interface PatherOptions {
  iterations: number;
  skipPegWithin: number;
  beamWidth: number;
  width: number;
  opacity: number;
}

interface EarlyStopOptions {
  lossThreshold: number | null;
  maxCount: number;
}

interface RenderOptions {
  width: number;
  opacity: number;
  color: string;
  bgColor: string;
}

export interface Options {
  pather: PatherOptions;
  earlyStop: EarlyStopOptions;
  render: RenderOptions;
}
