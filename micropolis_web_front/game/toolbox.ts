export const toolboxActionsIDs = [
  "bulldozer",
  "eraser",
  "wire",
  "road",
  "rail",
  "query",
  "residential",
  "commercial",
  "industrial",
  "firestation",
  "policestation",
  "coal",
  "nuclear",
  "airport",
  "seaport",
  "stadium",
] as const;
export type ToolboxActionID = typeof toolboxActionsIDs[number];
