import { Meta } from "@storybook/react";
import { select, boolean, number, color } from "@storybook/addon-knobs";

import LoaderSpinner, { loaderSpinnerTypes } from "../LoaderSpinner";

const printPascalCase = (pascal: string): string =>
  pascal.replace(/([A-Z])/g, " $1");

export default {
  title: "Common/LoaderSpinner",
  component: LoaderSpinner,
} as Meta;

export const Showcase = () => (
  <div className="flex flex-col items-center w-full h-full overflow-y-auto bg-pink-300">
    {loaderSpinnerTypes.map((type) => (
      <div key={type} title={printPascalCase(type)} className="mb-8">
        <LoaderSpinner type={type} color={color("Color", "#eaeaea")} />
      </div>
    ))}
  </div>
);

export const CustomSpinner = () => (
  <LoaderSpinner
    type={select(
      "Type",
      loaderSpinnerTypes.reduce(
        (acc, type) => ({
          ...acc,
          [printPascalCase(type)]: type,
        }),
        {}
      ),
      loaderSpinnerTypes[0]
    )}
    visible={boolean("Visible?", true)}
    width={number("Width (px)", 80, { min: 1, max: 1000, step: 1 })}
    height={number("Height (px)", 80, { min: 1, max: 1000, step: 1 })}
    color={color("Color", "#48bb78")}
  />
);
