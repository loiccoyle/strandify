import { Circle, Square, Minus, MousePointer, Eraser } from "lucide-react";
import { BrushType } from "../types//brushType";

interface ControlsProps {
  brushType: string;
  setBrushType: (type: BrushType) => void;
  pegCount: number;
  setPegCount: (count: number) => void;
  onClearPegs: () => void;
  onAutoPegs: () => void;
  totalPegs: number;
}

export function Controls({
  brushType,
  setBrushType,
  pegCount,
  setPegCount,
  onClearPegs,
  onAutoPegs,
  totalPegs,
}: ControlsProps) {
  return (
    <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-lg space-y-4">
      <div className="grid grid-cols-5 gap-2">
        {[
          { type: BrushType.Single, Icon: MousePointer, label: "Single" },
          { type: BrushType.Line, Icon: Minus, label: "Line" },
          { type: BrushType.Box, Icon: Square, label: "Box" },
          { type: BrushType.Circle, Icon: Circle, label: "Circle" },
          { type: BrushType.Eraser, Icon: Eraser, label: "Eraser" },
        ].map(({ type, Icon: Icon, label }) => (
          <div
            key={type}
            onClick={() => setBrushType(type)}
            className={`sm:p-4 p-2 rounded-lg transition-colors cursor-pointer ${
              brushType === type
                ? "bg-blue-100 dark:bg-blue-900 border-2 border-blue-500"
                : "bg-gray-100 dark:bg-gray-700 border-2 border-gray-300 dark:border-gray-600 hover:bg-gray-200 dark:hover:bg-gray-600"
            }`}
          >
            <div className="flex flex-col items-center">
              <Icon
                className={`w-5 h-5 mb-1 ${brushType === type ? "text-blue-500" : "text-gray-600 dark:text-gray-400"}`}
              />
              <span
                className={`text-xs font-medium ${
                  brushType === type
                    ? "text-gray-800 dark:text-gray-200"
                    : "text-gray-600 dark:text-gray-400"
                }`}
              >
                {label}
              </span>
            </div>
          </div>
        ))}
      </div>

      <div className="flex items-end gap-4 ">
        <div className="flex-1">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            Tool Pegs
          </label>
          <input
            type="number"
            value={pegCount}
            onChange={(e) => setPegCount(parseInt(e.target.value) || 50)}
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-blue-500 focus:border-blue-500"
            min="2"
            max="1000"
          />
        </div>

        <div className="flex flex-col">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            Total Pegs: {totalPegs}
          </label>
          <div className="flex gap-2">
            <button
              onClick={onClearPegs}
              className="px-2 sm:px-4 py-2 bg-red-100 dark:bg-red-900 border-2 border-red-500 text-red-700 dark:text-red-300 rounded-lg hover:bg-red-200 dark:hover:bg-red-800 transition-colors"
            >
              Clear Pegs
            </button>
            <button
              onClick={onAutoPegs}
              className="px-2 sm:px-4 py-2 bg-blue-100 dark:bg-blue-900 border-2 border-blue-500 text-blue-700 dark:text-blue-300 rounded-lg hover:bg-blue-200 dark:hover:bg-blue-800 transition-colors"
            >
              Auto Pegs
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
