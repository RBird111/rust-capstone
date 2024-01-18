import { handleErrors, normalize } from ".";

// ---TYPES--- \\
const GET_LOCATION = "locations/GET_LOCATION";
const GET_ALL_LOCATIONS = "locations/GET_ALL_LOCATIONS";
const CREATE_LOCATION = "locations/CREATE_LOCATION";
const UPDATE_LOCATION = "locations/UPDATE_LOCATION";
const DELETE_LOCATION = "locations/DELETE_LOCATION";

// ---ACTIONS--- \\
const _getLocation = (location) => ({
  type: GET_LOCATION,
  location,
});

const _getAllLocations = (locations) => ({
  type: GET_ALL_LOCATIONS,
  locations,
});

const _createLocation = (location) => ({
  type: CREATE_LOCATION,
  location,
});

const _updateLocation = (location) => ({
  type: UPDATE_LOCATION,
  location,
});

const _deleteLocation = (locationId) => ({
  type: DELETE_LOCATION,
  locationId,
});

// ---ACTION DISPATCHERS--- \\
export const getLocation = (locationId) => async (dispatch) => {
  const response = await fetch(`/api/location/${locationId}`);

  if (!response.ok) return await handleErrors(response);

  const { location } = await response.json();
  dispatch(_getLocation(location));

  return location;
};

export const getAllLocations = () => async (dispatch) => {
  const response = await fetch(`/api/location`);

  if (!response.ok) return await handleErrors(response);

  const { locations } = await response.json();
  dispatch(_getAllLocations(locations));

  return locations;
};

export const createLocation = (locationData) => async (dispatch) => {
  const response = await fetch(`/api/location`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(locationData),
  });

  if (!response.ok) return await handleErrors(response);

  const { location } = await response.json();
  dispatch(_createLocation(location));

  return location;
};

export const updateLocation = (locationData) => async (dispatch) => {
  const response = await fetch(`/api/location/${locationData.id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(locationData),
  });

  if (!response.ok) return await handleErrors(response);

  const { location } = await response.json();
  dispatch(_updateLocation(location));

  return location;
};

export const deleteLocation = (locationId) => async (dispatch) => {
  const response = await fetch(`/api/location/${locationId}`, {
    method: "DELETE",
  });

  if (!response.ok) return await handleErrors(response);

  const { message } = await response.json();
  dispatch(_deleteLocation(locationId));

  return message;
};

// ---REDUCER--- \\
const initialState = { currLocation: {}, allLocations: {} };

const locationReducer = (state = initialState, action) => {
  switch (action.type) {
    case GET_LOCATION: {
      const newState = normalize(state);

      newState.allLocations[action.location.id] = normalize(action.location);
      newState.currLocation = normalize(action.location);

      return newState;
    }

    case GET_ALL_LOCATIONS: {
      const newState = normalize(state);

      newState.allLocations = normalize(action.locations);

      return newState;
    }

    case CREATE_LOCATION: {
      const newState = normalize(state);

      newState.currLocation = normalize(action.location);
      newState.allLocations[action.location.id] = normalize(action.location);

      return newState;
    }

    case UPDATE_LOCATION: {
      const newState = normalize(state);

      newState.currLocation = normalize(action.location);
      newState.allLocations[action.location.id] = normalize(action.location);

      return newState;
    }

    case DELETE_LOCATION: {
      const newState = normalize(state);

      newState.currLocation = {};
      delete newState.allLocations[action.locationId];

      return newState;
    }
    default:
      return state;
  }
};

export default locationReducer;
