import { configure } from '@storybook/react';

import "../assets/styles/tailwind.css";

configure(require.context('../components/', true, /\.stories\.tsx$/), module);
