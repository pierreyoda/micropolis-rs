import React from "react";
import { storiesOf } from "@storybook/react";
import {
  withKnobs,
  select,
  boolean,
  number,
  color,
} from "@storybook/addon-knobs";

import LoaderSpinner, { loaderSpinnerTypes } from "../LoaderSpinner";
import StoryWrapper from "./StoryWrapper";

const printPascalCase = (pascal: string): string => pascal.replace(/([A-Z])/g, " $1");

storiesOf("common/LoaderSpinner", module)
  .addDecorator(withKnobs)
  .add("Showcase", () => (
    <StoryWrapper noPadding>
      <div className="w-full h-full flex flex-col items-center overflow-y-auto bg-pink-300">
        {loaderSpinnerTypes.map(type => (
          <div key={type} title={printPascalCase(type)} className="mb-8">
            <LoaderSpinner type={type} color={color("Color", "#eaeaea")} />
          </div>
        ))}
      </div>
    </StoryWrapper>
  ))
  .add("Custom Spinner", () => (
    <StoryWrapper>
      <LoaderSpinner
        type={select(
          "Type",
          loaderSpinnerTypes.reduce(
            (acc, type) => ({
              ...acc,
              [printPascalCase(type)]: type,
            }),
            {},
          ),
          loaderSpinnerTypes[0]
        )}
        visible={boolean("Visible?", true)}
        width={number("Width (px)", 80, { min: 1, max: 1000, step: 1 })}
        height={number("Height (px)", 80, { min: 1, max: 1000, step: 1 })}
        color={color("Color", "#48bb78")}
      />
    </StoryWrapper>
  ));
