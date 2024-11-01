import { Download, Pause, Play } from "lucide-react";
import React, {
  useState,
  useEffect,
  useRef,
  useCallback,
  ChangeEvent,
  memo,
} from "react";

interface StringArtDisplayProps {
  svgString: string;
}

const StringArt: React.FC<StringArtDisplayProps> = ({ svgString }) => {
  const parser = new DOMParser();
  const svgDoc = parser.parseFromString(svgString, "image/svg+xml");
  const svgElement = svgDoc.documentElement as unknown as SVGSVGElement;
  const width = svgElement.getAttribute("width") || "500";
  const height = svgElement.getAttribute("height") || "500";

  const rect = svgDoc.querySelector("rect");
  const paths = Array.from(svgDoc.querySelectorAll("path"));
  const totalPaths = paths.length;

  const [pathCount, setPathCount] = useState<number>(0);
  const [isPlaying, setIsPlaying] = useState(false);

  const animationFrameRef = useRef<number | null>(null);
  const startTimeRef = useRef<number | null>(null);

  const handleSliderChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => {
      const value = Number(e.target.value);
      if (value !== pathCount) {
        setPathCount(value);
        setIsPlaying(false); // Stop animation when user interacts with slider
      }
    },
    [pathCount],
  );

  const animatePaths = useCallback(
    (timestamp: number) => {
      if (!startTimeRef.current) startTimeRef.current = timestamp;
      const elapsed = timestamp - startTimeRef.current;

      const remainingPaths = totalPaths - pathCount;
      const duration = 2000 * (remainingPaths / totalPaths); // adjust duration based on remaining paths
      const batchSize = 10; // number of paths to render per frame
      const progress = Math.min(
        (elapsed / duration) * remainingPaths,
        remainingPaths,
      );

      // Increment path count in batches for smoother updates and fewer renders
      const newPathCount =
        pathCount + Math.floor(progress / batchSize) * batchSize;

      if (newPathCount > pathCount) {
        setPathCount(newPathCount);
      }

      if (progress < remainingPaths) {
        animationFrameRef.current = requestAnimationFrame(animatePaths);
      } else {
        setIsPlaying(false);
      }
    },
    [pathCount, totalPaths],
  );

  useEffect(() => {
    if (isPlaying) {
      startTimeRef.current = null; // Reset start time on play
      animationFrameRef.current = requestAnimationFrame(animatePaths);
    } else if (animationFrameRef.current) {
      cancelAnimationFrame(animationFrameRef.current);
      animationFrameRef.current = null;
      startTimeRef.current = null;
    }

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [isPlaying, animatePaths]);

  const handleDownloadSvg = useCallback(() => {
    if (!svgString) return;
    const blob = new Blob([svgString], { type: "image/svg+xml" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "strandify.svg";
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }, [svgString]);

  const handleDownloadPng = useCallback(() => {
    if (!svgString) return;

    // Create an image element
    const img = new Image();
    const blob = new Blob([svgString], { type: "image/svg+xml" });
    const url = URL.createObjectURL(blob);

    img.onload = () => {
      // Create a canvas element
      const canvas = document.createElement("canvas");
      const context = canvas.getContext("2d");
      if (!context) return;

      // Set canvas dimensions
      canvas.width = img.width;
      canvas.height = img.height;

      // Draw the image onto the canvas
      context.drawImage(img, 0, 0);

      // Convert the canvas to a PNG data URL
      canvas.toBlob((pngBlob) => {
        if (pngBlob) {
          const pngUrl = URL.createObjectURL(pngBlob);
          const a = document.createElement("a");
          a.href = pngUrl;
          a.download = "strandify.png";
          document.body.appendChild(a);
          a.click();
          document.body.removeChild(a);
          URL.revokeObjectURL(pngUrl);
        }
      }, "image/png");

      // Clean up the object URL
      URL.revokeObjectURL(url);
    };

    img.src = url; // Start loading the SVG
  }, [svgString]);

  // Memoize the SVG rendering to prevent unnecessary re-renders
  const SvgStringart = memo(() => (
    <svg width={width} height={height}>
      {rect && <g dangerouslySetInnerHTML={{ __html: rect.outerHTML }} />}
      {paths.slice(0, pathCount).map((path, index) => (
        <g key={index} dangerouslySetInnerHTML={{ __html: path.outerHTML }} />
      ))}
    </svg>
  ));

  return (
    <div style={{ textAlign: "center" }}>
      <SvgStringart />
      <div className="flex flex-row w-full items-center mt-4 gap-2">
        <button
          onClick={() => setIsPlaying((prev) => !prev)}
          className="p-2 bg-blue-500 text-white rounded-full"
        >
          {isPlaying ? <Pause /> : <Play />}
        </button>
        <input
          type="range"
          min="0"
          max={totalPaths}
          value={pathCount}
          onChange={handleSliderChange}
          className="flex-grow mr-2"
        />
        <div className="text-gray-800 dark:text-white">
          {pathCount} / {totalPaths}
        </div>
      </div>
      <div className="flex gap-2 items-center justify-center">
        <button
          onClick={handleDownloadSvg}
          className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors flex items-center gap-2"
        >
          <Download className="w-5 h-5" />
          <span>Download SVG</span>
        </button>
        <button
          onClick={handleDownloadPng}
          className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors flex items-center gap-2"
        >
          <Download className="w-5 h-5" />
          <span>Download PNG</span>
        </button>
      </div>
    </div>
  );
};

export default StringArt;
