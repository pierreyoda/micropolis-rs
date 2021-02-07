import React, {
  useState,
  useCallback,
  FunctionComponent,
} from "react";
import "twin.macro";

export interface ToolbarItem {
  key: string;
  label: string;
  children: ToolbarItemChild[];
  onClick?: () => void;
}

export type ToolbarItemChild = Omit<ToolbarItem, "children">;

export interface ToolbarProps {
  items: ToolbarItem[];
}

const Toolbar: FunctionComponent<ToolbarProps> = ({ items }) => {
  const [openedKey, setOpenedKey] = useState<string | null>(null);

  return (
    <div tw="relative flex flex-row items-start justify-start py-1 px-4">
      {items.map(({ key, label, children }) => (
        <div
          key={key}
          tw="bg-gray-500 text-gray-200 py-2 pr-8 last:pr-0"
          onMouseEnter={useCallback(() => {
            if (openedKey === key) { return; }
            setOpenedKey(key);
          }, [])}
          onMouseLeave={useCallback(() => setOpenedKey(null), [])}
        >
          {label}
          {(children.length > 0 && openedKey === key) && (
            <div
              style={{ top: "1.5rem" }}
              tw="bg-transparent absolute flex flex-col items-start justify-center py-2 mt-1"
            >
              {children.map(({ key: childKey, label: childLabel, onClick }) => (
                <div
                  key={childKey}
                  tw="text-black flex items-center justify-start mt-1"
                  onClick={onClick}
                >
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
