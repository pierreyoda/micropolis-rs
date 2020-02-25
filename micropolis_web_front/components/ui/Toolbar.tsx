import React, { 
  useState, 
  useCallback,
  FunctionComponent, 
} from "react";

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
    <div className="relative flex flex-row items-start justify-start py-1 px-4">
      {items.map(({ key, label, children }) => (
        <div
          key={key} 
          className="bg-grey-700 text-black-700 pr-8 last:pr-0"
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
              className="bg-grey-700 absolute flex flex-col items-start justify-center py-2"
            >
              {children.map(({ label: childLabel, onClick }) => (
                <div
                  className="text-black flex items-center justify-start"
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
