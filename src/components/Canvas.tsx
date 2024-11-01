import { forwardRef, useEffect, useRef, useState } from "react";
import { BrushType } from "../types/brushType";
import { drawCanvas } from "../utils/canvas";
import { lineCoords, rectangleCoords, circleCoords } from "strandify-wasm";

interface CanvasProps {
  image: HTMLImageElement | null;
  pegs: Array<{ x: number; y: number }>;
  setPegs: (pegs: Array<{ x: number; y: number }>) => void;
  brushType: BrushType;
  pegCount: number;
}

export const Canvas = forwardRef<HTMLCanvasElement, CanvasProps>(
  ({ image, pegs, setPegs, brushType, pegCount }, ref) => {
    const [isDragging, setIsDragging] = useState(false);
    const [draggedPegIndex, setDraggedPegIndex] = useState(-1);
    const [isDrawing, setIsDrawing] = useState(false);
    const [isErasing, setIsErasing] = useState(false);
    const [startPos, setStartPos] = useState({ x: 0, y: 0 });
    const [selectionBox, setSelectionBox] = useState<{
      x: number;
      y: number;
      width: number;
      height: number;
    } | null>(null);
    const contextRef = useRef<CanvasRenderingContext2D | null>(null);
    const containerRef = useRef<HTMLDivElement>(null);

    function previewBrush(
      startX: number,
      startY: number,
      endX: number,
      endY: number,
    ) {
      if (contextRef.current === null) return;
      const ctx = contextRef.current;
      const pegs = brushPegs(startX, startY, endX, endY);
      ctx.strokeStyle = "rgba(255, 0, 0, 0.5)";
      ctx.fillStyle = "rgba(255, 0, 0, 0.5)";

      pegs.forEach((peg) => {
        ctx.beginPath();
        ctx.arc(peg.x, peg.y, 5, 0, Math.PI * 2);
        ctx.fill();
      });

      ctx.beginPath();
      ctx.moveTo(pegs[0].x, pegs[0].y);
      pegs.slice(1).forEach((peg) => ctx.lineTo(peg.x, peg.y));
      ctx.closePath();
      ctx.stroke();
    }

    function brushPegs(
      startX: number,
      startY: number,
      endX: number,
      endY: number,
    ): Array<{ x: number; y: number }> {
      if (ref === null || !("current" in ref) || ref.current === null)
        return [];
      if (isNaN(pegCount) || pegCount < 2) return [];

      let shapeCoords;
      switch (brushType) {
        case BrushType.Line:
          shapeCoords = lineCoords(startX, startY, endX, endY, pegCount);
          break;
        case BrushType.Box:
          shapeCoords = rectangleCoords(
            Math.min(startX, endX),
            Math.min(startY, endY),
            Math.abs(startX - endX),
            Math.abs(startY - endY),
            pegCount,
          );
          break;
        case BrushType.Circle:
          shapeCoords = circleCoords(
            startX,
            startY,
            Math.hypot(endX - startX, endY - startY),
            pegCount,
          );
          break;
        default:
          return [];
      }

      const x_coords = shapeCoords.get_x();
      const y_coords = shapeCoords.get_y();

      // need to create a new array, for some reason the wasm one doesn't like map
      const out: Array<{ x: number; y: number }> = [];
      x_coords.forEach((x, i) =>
        out.push({
          x: Math.min(x, ref.current!.width - 1),
          y: Math.min(y_coords[i], ref.current!.height - 1),
        }),
      );
      return out;
    }

    // on first mount adjust size
    useEffect(() => {
      resizeCanvas();
    }, []);

    useEffect(() => {
      if (ref && "current" in ref && ref.current) {
        contextRef.current = ref.current.getContext("2d");
        resizeCanvas();
      }
      // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [ref, image]);

    useEffect(() => {
      if (ref && "current" in ref && ref.current) {
        drawCanvas(ref.current, image, pegs, selectionBox);
      }
    }, [ref, image, pegs, selectionBox]);

    useEffect(() => {
      window.addEventListener("resize", resizeCanvas);
      return () => window.removeEventListener("resize", resizeCanvas);
      // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [image]);

    const resizeCanvas = () => {
      if (!ref || !("current" in ref) || !ref.current || !containerRef.current)
        return;

      const canvas = ref.current;
      const container = containerRef.current;
      const maxWidth = container.clientWidth;
      const maxHeight = window.innerHeight * 0.6;

      let width, height;

      if (image) {
        const imageAspectRatio = image.width / image.height;
        const containerAspectRatio = maxWidth / maxHeight;

        if (imageAspectRatio > containerAspectRatio) {
          width = maxWidth;
          height = maxWidth / imageAspectRatio;
        } else {
          height = maxHeight;
          width = maxHeight * imageAspectRatio;
        }
      } else {
        width = maxWidth;
        height = maxWidth * (9 / 16);
      }

      canvas.width = width;
      canvas.height = height;

      const scaleX = width / canvas.width;
      const scaleY = height / canvas.height;
      if (scaleX !== 1 || scaleY !== 1) {
        setPegs(
          pegs.map((peg) => ({
            x: peg.x * scaleX,
            y: peg.y * scaleY,
          })),
        );
      }

      drawCanvas(canvas, image, pegs);
    };

    const handleMouseDown = (e: React.MouseEvent<HTMLCanvasElement>) => {
      if (!ref || !("current" in ref) || !ref.current) return;

      const canvas = ref.current;
      const rect = canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;

      setStartPos({ x, y });

      if (brushType === BrushType.Eraser) {
        setIsErasing(true);
        setSelectionBox({ x, y, width: 0, height: 0 });
        return;
      }

      const draggedPeg = pegs.findIndex(
        (peg) => Math.hypot(x - peg.x, y - peg.y) < 5,
      );

      if (draggedPeg !== -1) {
        setIsDragging(true);
        setDraggedPegIndex(draggedPeg);
        return;
      }

      if (brushType === BrushType.Single) {
        setPegs([...pegs, { x, y }]);
      } else {
        setIsDrawing(true);
      }
    };

    const handleMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
      if (!ref || !("current" in ref) || !ref.current) return;

      const canvas = ref.current;
      const rect = canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;

      if (isDragging && draggedPegIndex !== -1) {
        const newPegs = [...pegs];
        newPegs[draggedPegIndex] = { x, y };
        setPegs(newPegs);
      } else if (isErasing && selectionBox) {
        setSelectionBox({
          x: Math.min(startPos.x, x),
          y: Math.min(startPos.y, y),
          width: Math.abs(x - startPos.x),
          height: Math.abs(y - startPos.y),
        });
      } else if (isDrawing) {
        drawCanvas(canvas, image, pegs, null);
        previewBrush(startPos.x, startPos.y, x, y);
      }
    };

    const handleMouseUp = (e: React.MouseEvent<HTMLCanvasElement>) => {
      if (!ref || !("current" in ref) || !ref.current) return;
      const canvas = ref.current;
      const rect = canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;

      if (isErasing && brushType === BrushType.Eraser && selectionBox) {
        const newPegs = pegs.filter((peg) => {
          return !(
            peg.x >= selectionBox.x &&
            peg.x <= selectionBox.x + selectionBox.width &&
            peg.y >= selectionBox.y &&
            peg.y <= selectionBox.y + selectionBox.height
          );
        });
        setPegs(newPegs);
      } else if (isDrawing) {
        setPegs([...pegs, ...brushPegs(startPos.x, startPos.y, x, y)]);
      }

      setSelectionBox(null);
      setIsDragging(false);
      setIsDrawing(false);
      setIsErasing(false);
      setDraggedPegIndex(-1);
      if (ref && "current" in ref && ref.current) {
        drawCanvas(ref.current, image, pegs, null);
      }
    };

    return (
      <div ref={containerRef} className="w-full">
        <canvas
          ref={ref}
          className="mx-auto border border-gray-200 dark:border-gray-600 cursor-crosshair"
          onMouseDown={handleMouseDown}
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
        />
      </div>
    );
  },
);
