import { normalize, handleErrors } from ".";

// ---TYPES--- \\
const SET_USER = "session/SET_USER";
const REMOVE_USER = "session/REMOVE_USER";
const UPDATE_USER = "session/UPDATE_USER";

// ---ACTIONS--- \\
const setUser = (user) => ({
  type: SET_USER,
  payload: user,
});

const removeUser = () => ({
  type: REMOVE_USER,
});

const _updateUser = (user) => ({
  type: UPDATE_USER,
  user,
});

// ---ACTION DISPATCHERS--- \\
export const authenticate = () => async (dispatch) => {
  const response = await fetch("/api/auth", {
    headers: {
      "Content-Type": "application/json",
    },
  });

  if (response.ok) {
    const data = await response.json();
    if (data.errors) {
      return;
    }

    dispatch(setUser(data));
  }
};

export const login =
  ({ credential, password }) =>
  async (dispatch) => {
    const response = await fetch("/api/auth/login", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        credential,
        password,
      }),
    });

    if (response.ok) {
      const data = await response.json();
      dispatch(setUser(data));
      return null;
    } else if (response.status < 500) {
      const data = await response.json();
      if (data.errors) {
        return data.errors;
      }
    } else {
      return ["An error occurred. Please try again."];
    }
  };

export const logout = () => async (dispatch) => {
  const response = await fetch("/api/auth/logout", {
    headers: {
      "Content-Type": "application/json",
    },
  });

  if (response.ok) {
    dispatch(removeUser());
  }
};

export const signUp = (user) => async (dispatch) => {
  const response = await fetch("/api/auth/signup", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(user),
  });

  if (response.ok) {
    const data = await response.json();
    dispatch(setUser(data));
    return null;
  } else if (response.status < 500) {
    const data = await response.json();
    if (data.errors) {
      return data.errors;
    }
  } else {
    return ["An error occurred. Please try again."];
  }
};

export const updateUser = (userData) => async (dispatch) => {
  const response = await fetch(`/api/user/curr`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(userData),
  });

  if (response.ok) {
    const data = await response.json();
    await dispatch(_updateUser(data));
    return null;
  } else {
    const data = await response.json();
    if (data.errors) {
      return data.errors;
    }
  }
};

export const deleteUser = (userId) => async (dispatch) => {
  const response = await fetch(`/api/user/curr`, {
    method: "DELETE",
  });

  if (!response.ok) return await handleErrors(response);

  await response.json();
  dispatch(removeUser());

  return null;
};

// ---REDUCER--- \\
const initialState = { user: null };

const reducer = (state = initialState, action) => {
  switch (action.type) {
    case SET_USER:
      return { user: normalize(action.payload) };
    case REMOVE_USER:
      return { user: null };
    case UPDATE_USER:
      return { user: normalize(action.user) };
    default:
      return state;
  }
};

export default reducer;
