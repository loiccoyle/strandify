import React, { useState, ChangeEvent } from "react";

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
  const [pathCount, setPathCount] = useState<number>(paths.length);

  const handleSliderChange = (e: ChangeEvent<HTMLInputElement>) => {
    setPathCount(Number(e.target.value));
  };

  const SvgStringart: React.FC = () => {
    return (
      <svg
        width={width}
        height={height}
        dangerouslySetInnerHTML={{
          __html: `
            ${rect ? rect.outerHTML : ""}
            ${paths
              .slice(0, pathCount)
              .map((path) => path.outerHTML)
              .join("")}
          `,
        }}
      />
    );
  };

  return (
    <div style={{ textAlign: "center" }}>
      <SvgStringart />
      <div className="flex flex-row w-full items-center mt-4">
        <input
          type="range"
          min="0"
          max={paths.length}
          value={pathCount}
          onChange={handleSliderChange}
          className="flex-grow mr-2"
        />
        <div className="text-gray-800 dark:text-white">
          {pathCount} / {paths.length}
        </div>
      </div>
    </div>
  );
};

export default StringArt;
