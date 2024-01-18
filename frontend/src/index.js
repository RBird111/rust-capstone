import React from "react";
import ReactDOM from "react-dom";
import { Provider } from "react-redux";
import { BrowserRouter } from "react-router-dom";

import { ModalProvider, Modal } from "./context/Modal";
import configureStore, { normalize } from "./store";
import * as businessActions from "./store/business";
import * as locationActions from "./store/locations";
import * as reviewActions from "./store/reviews";
import * as sessionActions from "./store/session";
import * as imageActions from "./store/images";
import App from "./App";

import "./index.scss";

const store = configureStore();

if (process.env.NODE_ENV !== "production") {
  window.store = store;
  window.normalize = normalize;
  window.businessActions = businessActions;
  window.imageActions = imageActions;
  window.locationActions = locationActions;
  window.reviewActions = reviewActions;
  window.sessionActions = sessionActions;
}

// Wrap the application with the Modal provider and render the Modal component
// after the App component so that all the Modal content will be layered as
// HTML elements on top of the all the other HTML elements:
function Root() {
  return (
    <ModalProvider>
      <Provider store={store}>
        <BrowserRouter>
          <App />
          <Modal />
        </BrowserRouter>
      </Provider>
    </ModalProvider>
  );
}

ReactDOM.render(
  <React.StrictMode>
    <Root />
  </React.StrictMode>,
  document.getElementById("root")
);
