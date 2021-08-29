import React, { useState, FunctionComponent } from "react";

export interface ToolbarItem {
  key: string;
  label: string;
  children: readonly ToolbarItemChild[];
  onClick?: () => void;
}

export type ToolbarItemChild = Omit<ToolbarItem, "children">;

export interface ToolbarProps {
  items: readonly ToolbarItem[];
}

const Toolbar: FunctionComponent<ToolbarProps> = ({ items }) => {
  const [openedKey, setOpenedKey] = useState<string | null>(null);

  return (
    <div className="relative flex flex-row items-start justify-start w-full px-4 py-1 bg-gray-500">
      {items.map(({ key, label, children }) => (
        <div
          key={key}
          className="py-2 pr-8 text-gray-200 last:pr-0"
          onMouseEnter={() => {
            if (openedKey === key) {
              return;
            }
            setOpenedKey(key);
          }}
          onMouseLeave={() => setOpenedKey(null)}
        >
          {label}
          {children.length > 0 && openedKey === key && (
            <div
              style={{ top: "1.5rem" }}
              className="absolute flex flex-col items-start justify-center py-2 mt-1 bg-transparent"
            >
              {children.map(({ key: childKey, label: childLabel, onClick }) => (
                <div key={childKey} className="flex items-center justify-start mt-1 text-black" onClick={onClick}>
                  {childLabel}
                </div>
              ))}
            </div>
          )}
        </div>
      ))}
    </div>
  );
};

export default Toolbar;
