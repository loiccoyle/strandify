import { useState, useEffect } from "react";
import {
  ChevronDown,
  ChevronUp,
  Settings2,
  Palette,
  Sliders,
  LucideIcon,
} from "lucide-react";
import { Options as OptionsType } from "../types/options";

interface OptionsProps {
  options: OptionsType;
  onChange: (options: OptionsType) => void;
}

type SectionKey = "pather" | "advanced" | "render";

interface ExpandedSections {
  pather: boolean;
  advanced: boolean;
  render: boolean;
}

interface SectionConfig {
  title: string;
  icon: LucideIcon;
  fields: FieldConfig[];
}

interface FieldConfig {
  section: keyof OptionsType;
  key: string;
  label: string;
  type: "number" | "color";
  min?: number;
  max?: number;
  step?: number;
}

export function Options({ options, onChange }: OptionsProps) {
  const [expandedSections, setExpandedSections] = useState<ExpandedSections>({
    pather: true,
    advanced: false,
    render: true,
  });

  const updateOptions = (
    section: keyof OptionsType,
    key: string,
    value: number | string | null,
  ) => {
    onChange({
      ...options,
      [section]: {
        ...options[section],
        [key]: value,
      },
    });
  };

  const sections: Record<SectionKey, SectionConfig> = {
    pather: {
      title: "Pather Options",
      icon: Settings2,
      fields: [
        {
          section: "pather",
          key: "iterations",
          label: "Iterations",
          type: "number",
          min: 1,
          step: 500,
        },
        {
          section: "pather",
          key: "width",
          label: "Width",
          type: "number",
          min: 0,
          step: 1,
        },
        {
          section: "pather",
          key: "opacity",
          label: "Opacity",
          type: "number",
          min: 0,
          max: 1,
          step: 0.1,
        },
      ],
    },
    advanced: {
      title: "Advanced Settings",
      icon: Sliders,
      fields: [
        {
          section: "pather",
          key: "skipPegWithin",
          label: "Skip Peg Within",
          type: "number",
          min: 0,
        },
        {
          section: "pather",
          key: "beamWidth",
          label: "Beam Search Width",
          type: "number",
          min: 1,
        },
        {
          section: "earlyStop",
          key: "lossThreshold",
          label: "Early Stop Loss Threshold",
          type: "number",
          min: 0,
          max: 1,
          step: 0.01,
        },
        {
          section: "earlyStop",
          key: "maxCount",
          label: "Early Stop Max Count",
          type: "number",
          min: 1,
        },
      ],
    },
    // earlyStop: {
    //   title: "Early Stop Options",
    //   icon: Timer,
    //   fields: [
    //     {
    //       section: "earlyStop",
    //       key: "lossThreshold",
    //       label: "Loss Threshold",
    //       type: "number",
    //       min: 0,
    //       max: 1,
    //       step: 0.01,
    //     },
    //     {
    //       section: "earlyStop",
    //       key: "maxCount",
    //       label: "Max Count",
    //       type: "number",
    //       min: 1,
    //     },
    //   ],
    // },
    render: {
      title: "Render Options",
      icon: Palette,
      fields: [
        {
          section: "render",
          key: "width",
          label: "Width",
          type: "number",
          min: 0.1,
          step: 0.1,
        },
        {
          section: "render",
          key: "opacity",
          label: "Opacity",
          type: "number",
          min: 0,
          max: 1,
          step: 0.1,
        },
        { section: "render", key: "color", label: "Color", type: "color" },
        {
          section: "render",
          key: "bgColor",
          label: "Background Color",
          type: "color",
        },
      ],
    },
  };

  const OptionSection = ({
    sectionKey,
    config,
  }: {
    sectionKey: SectionKey;
    config: SectionConfig;
  }) => {
    const Icon = config.icon;
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg px-4 py-2">
        <button
          onClick={() =>
            setExpandedSections((prev) => ({
              ...prev,
              [sectionKey]: !prev[sectionKey],
            }))
          }
          className="w-full flex items-center justify-between text-gray-900 dark:text-white mb-2"
        >
          <div className="flex items-center gap-2">
            <Icon className="w-5 h-5" />
            <h3 className="font-medium">{config.title}</h3>
          </div>
          {expandedSections[sectionKey] ? (
            <ChevronUp className="w-5 h-5" />
          ) : (
            <ChevronDown className="w-5 h-5" />
          )}
        </button>
        {expandedSections[sectionKey] && (
          <div className="space-y-4 mt-4">
            {config.fields.map((field) => (
              <Field
                key={`${field.section}-${field.key}`}
                field={field}
                value={
                  options[field.section][
                    field.key as keyof (typeof options)[typeof field.section]
                  ]
                }
                onChange={(value) =>
                  updateOptions(field.section, field.key, value)
                }
              />
            ))}
          </div>
        )}
      </div>
    );
  };

  const Field = ({
    field,
    value,
    onChange,
  }: {
    field: FieldConfig;
    value: number | string | null;
    onChange: (value: number | string | null) => void;
  }) => {
    const [localValue, setLocalValue] = useState(value?.toString() ?? "");

    useEffect(() => {
      setLocalValue(value?.toString() ?? "");
    }, [value]);

    const commonInputClasses =
      "w-24 px-2 py-1 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-blue-500 focus:border-blue-500";

    const handleBlur = () => {
      if (field.type === "number") {
        const num = localValue === "" ? null : parseFloat(localValue);
        if (
          localValue === "" ||
          (num !== null &&
            !isNaN(num) &&
            num >= (field.min ?? -Infinity) &&
            num <= (field.max ?? Infinity))
        ) {
          onChange(num);
        } else {
          setLocalValue(value?.toString() ?? "");
        }
      }
    };

    if (field.type === "number") {
      return (
        <div>
          <label className="flex items-center justify-between text-sm text-gray-700 dark:text-gray-300">
            <span>{field.label}</span>
            <input
              type="text"
              inputMode="decimal"
              value={localValue}
              onChange={(e) => setLocalValue(e.target.value)}
              onBlur={handleBlur}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  handleBlur();
                }
              }}
              className={commonInputClasses}
            />
          </label>
        </div>
      );
    }

    return (
      <div>
        <label className="flex items-center justify-between text-sm text-gray-700 dark:text-gray-300">
          <span>{field.label}</span>
          <input
            type="color"
            value={value as string}
            onChange={(e) => onChange(e.target.value)}
            className={`${commonInputClasses} h-8 cursor-pointer`}
          />
        </label>
      </div>
    );
  };

  return (
    <div className="space-y-4">
      {(Object.entries(sections) as [SectionKey, SectionConfig][]).map(
        ([key, config]) => (
          <OptionSection key={key} sectionKey={key} config={config} />
        ),
      )}
    </div>
  );
}

export default Options;
