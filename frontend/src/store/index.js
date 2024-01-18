import { createStore, combineReducers, applyMiddleware, compose } from "redux";
import thunk from "redux-thunk";
import session from "./session";
import businessReducer from "./business";
import imageReducer from "./images";
import locationReducer from "./locations";
import reviewReducer from "./reviews";

// Helper function that handles failed promises
export const handleErrors = async (response) => {
  const errors = await response.json();
  return errors;
};

// Helper function that normalizes data
export const normalize = (data) => {
  // Check to see if data is an object
  switch (typeof data) {
    case "object": {
      // If object is null return
      if (data === null) return null;

      // Check to see if data is an array
      switch (Array.isArray(data)) {
        // If it's an array then normalize each item
        case true:
          return data.reduce((acc, item) => {
            item = normalize(item);
            acc[item.id] = item;
            return acc;
          }, {});

        // Otherwise its a POJO
        default:
          // So we normalize the entries of the POJO
          return Object.entries(data).reduce((acc, [k, v]) => {
            v = normalize(v);
            acc[k] = v;
            return acc;
          }, {});
      }
    }
    // If data is undefined
    case "undefined":
      return null;

    // If it's not an object then just return it to the caller
    default:
      return data;
  }
};

// ---MAIN REDUCER --- \\
const rootReducer = combineReducers({
  session,
  business: businessReducer,
  images: imageReducer,
  location: locationReducer,
  reviews: reviewReducer,
});

let enhancer;

if (process.env.NODE_ENV === "production") {
  enhancer = applyMiddleware(thunk);
} else {
  const logger = require("redux-logger").default;
  const composeEnhancers =
    window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose;
  enhancer = composeEnhancers(applyMiddleware(thunk, logger));
}

const configureStore = (preloadedState) => {
  return createStore(rootReducer, preloadedState, enhancer);
};

export default configureStore;
