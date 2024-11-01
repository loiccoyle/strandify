import { useState, useMemo, useEffect } from "react";
import { Options } from "../types/options";

// Worker creation function
const createWorker = () => {
  console.log("new worker");
  const worker = new Worker(
    new URL("../workers/strandifyWorker.ts", import.meta.url),
    { type: "module" },
  );
  return worker;
};

export function useStrandify() {
  const [isComputing, setIsComputing] = useState(false);
  const [stringArtSvg, setStringArtSvg] = useState<string | null>(null);

  const worker = useMemo(() => createWorker(), []);

  useEffect(() => {
    worker.onmessage = (event) => {
      console.log("message received from worker");
      console.log(event);
      setStringArtSvg(event.data.svg);
      setIsComputing(false);
    };

    worker.onerror = (event) => {
      setIsComputing(false);
      console.error(event);
    };

    // Cleanup worker when component unmounts
    return () => {
      worker.terminate();
    };
  }, [worker]);

  const createStringArt = async (
    canvas: HTMLCanvasElement,
    pegs: Array<{ x: number; y: number }>,
    options: Options,
  ) => {
    setIsComputing(true);
    const imageData = await new Promise<Uint8Array>((resolve) => {
      canvas.toBlob((blob) => {
        blob?.arrayBuffer().then((buffer) => {
          resolve(new Uint8Array(buffer));
        });
      });
    });
    worker.postMessage({ imageData, pegs, options });
  };

  return { stringArtSvg, setStringArtSvg, createStringArt, isComputing };
}
