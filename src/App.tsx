import { useState, useRef, useCallback, useEffect } from "react";
import {
  Download,
  Trash2,
  Wand2,
  Image as ImageIcon,
  Loader,
} from "lucide-react";
import { Canvas } from "./components/Canvas";
import { ThemeProvider } from "./contexts/ThemeContext";
import { Controls } from "./components/Controls";
import { Options } from "./components/Options";
import { useStrandify } from "./hooks/useStrandify";
import { BrushType } from "./types/brushType";
import { Options as OptionsType } from "./types/options";
import StringArt from "./components/StringArt";
import Header from "./components/Header";
import Footer from "./components/Footer";

export default function App() {
  const [image, setImage] = useState<HTMLImageElement | null>(null);
  const [pegs, setPegs] = useState<Array<{ x: number; y: number }>>([]);
  const [brushType, setBrushType] = useState<BrushType>(BrushType.Circle);
  const [pegCount, setPegCount] = useState(500);
  const [options, setOptions] = useState<OptionsType>({
    pather: {
      iterations: 4000,
      skipPegWithin: 10,
      beamWidth: 1,
      width: 1,
      opacity: 0.2,
    },
    earlyStop: {
      lossThreshold: null,
      maxCount: 1000,
    },
    render: {
      width: 1,
      opacity: 0.2,
      color: "#000000",
      bgColor: "#ffffff",
    },
  });

  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { stringArtSvg, setStringArtSvg, createStringArt, isComputing } =
    useStrandify();

  const handleImageUpload = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      if (!file) return;

      const reader = new FileReader();
      reader.onload = (event) => {
        const img = new Image();
        img.onload = () => setImage(img);
        img.src = event.target?.result as string;
      };
      reader.readAsDataURL(file);
    },
    [],
  );

  const handleRemoveImage = useCallback(() => {
    setImage(null);
    setPegs([]);
    setStringArtSvg(null);
  }, [setStringArtSvg, setPegs, setImage]);

  const handleClearPegs = useCallback(() => {
    setPegs([]);
  }, []);

  const handleAutoPegs = useCallback(() => {
    if (!canvasRef.current) return;
    const canvas = canvasRef.current;
    const newPegs = [];
    const count = pegCount;

    if (brushType === BrushType.Box) {
      for (let i = 0; i < count; i++) {
        const t = i / count;
        const perimeter = 2 * (canvas.width + canvas.height);
        const dist = t * perimeter;

        let x, y;
        if (dist < canvas.width) {
          x = dist;
          y = 0;
        } else if (dist < canvas.width + canvas.height) {
          x = canvas.width;
          y = dist - canvas.width;
        } else if (dist < 2 * canvas.width + canvas.height) {
          x = canvas.width - (dist - canvas.width - canvas.height);
          y = canvas.height;
        } else {
          x = 0;
          y = canvas.height - (dist - 2 * canvas.width - canvas.height);
        }

        newPegs.push({ x, y });
      }
    } else {
      const centerX = canvas.width / 2;
      const centerY = canvas.height / 2;
      const radius = Math.min(canvas.width, canvas.height) / 2 - 10;

      for (let i = 0; i < count; i++) {
        const angle = (i / count) * 2 * Math.PI;
        const x = centerX + radius * Math.cos(angle);
        const y = centerY + radius * Math.sin(angle);
        newPegs.push({ x, y });
      }
    }

    setPegs(newPegs);
  }, [brushType, pegCount]);

  const handleCompute = useCallback(async () => {
    if (!canvasRef.current || pegs.length === 0) return;
    createStringArt(canvasRef.current, pegs, options);
  }, [pegs, createStringArt, options]);

  const handleDownloadSvg = useCallback(() => {
    if (!stringArtSvg) return;
    const blob = new Blob([stringArtSvg], { type: "image/svg+xml" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "strandify.svg";
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }, [stringArtSvg]);

  useEffect(() => {
    if (stringArtSvg === null) return;

    const parser = new DOMParser();
    const svgDoc = parser.parseFromString(stringArtSvg, "image/svg+xml");
    const svgElement = svgDoc.documentElement as unknown as SVGSVGElement;

    const rect = svgDoc.querySelector("rect");

    const paths = Array.from(svgDoc.querySelectorAll("path"));
    for (let i = 0; i < paths.length; i++) {
      const path = paths[i];
      path.setAttribute("stroke", options.render.color);
      path.setAttribute("stroke-width", options.render.width.toString());
      path.setAttribute("opacity", options.render.opacity.toString());
    }
    rect?.setAttribute("fill", options.render.bgColor);

    setStringArtSvg(svgElement.outerHTML);
  }, [options.render, stringArtSvg, setStringArtSvg]);

  return (
    <ThemeProvider>
      <div className="min-h-screen bg-gray-100 dark:bg-gray-900 sm:p-8 p-2">
        <div className="w-full sm:max-w-7xl sm:mx-auto space-y-8">
          <Header />

          <div className="grid grid-cols-1 gap-8">
            <div className="space-y-4">
              <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-lg">
                <div className="flex gap-4 mb-4 sm:text-lg text-sm justify-center">
                  <label className="flex-1 flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg cursor-pointer hover:bg-blue-600 transition-colors">
                    <ImageIcon className="w-5 h-5" />
                    <span>Select Image</span>
                    <input
                      type="file"
                      accept="image/*"
                      className="hidden"
                      onChange={handleImageUpload}
                    />
                  </label>
                  <button
                    onClick={handleRemoveImage}
                    className="px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors flex items-center gap-2"
                  >
                    <Trash2 className="w-5 h-5" />
                    <span>Remove</span>
                  </button>
                </div>

                <Canvas
                  ref={canvasRef}
                  image={image}
                  pegs={pegs}
                  setPegs={setPegs}
                  brushType={brushType}
                  pegCount={pegCount}
                />
              </div>

              <Controls
                brushType={brushType}
                setBrushType={setBrushType}
                pegCount={pegCount}
                setPegCount={setPegCount}
                onClearPegs={handleClearPegs}
                onAutoPegs={handleAutoPegs}
              />

              <Options options={options} onChange={setOptions} />
            </div>

            <div className="space-y-4">
              <div className="flex gap-4">
                <button
                  onClick={handleCompute}
                  disabled={isComputing || pegs.length === 0}
                  className="flex-1 flex items-center justify-center gap-2 px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <Wand2 className="w-5 h-5" />
                  <span>Generate String Art</span>
                </button>

                {stringArtSvg && (
                  <button
                    onClick={handleDownloadSvg}
                    className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors flex items-center gap-2"
                  >
                    <Download className="w-5 h-5" />
                    <span>Download SVG</span>
                  </button>
                )}
              </div>

              {isComputing ? (
                <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-lg flex align-center justify-center">
                  <Loader className="w-8 h-8 text-blue-500 animate-spin mr-2" />
                  <span className="text-lg font-semibold dark:text-white">
                    Building string art...
                  </span>
                </div>
              ) : (
                stringArtSvg && (
                  <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-lg flex align-center justify-center">
                    <StringArt svgString={stringArtSvg} />
                  </div>
                )
              )}
            </div>
          </div>
          <Footer />
        </div>
      </div>
    </ThemeProvider>
  );
}
