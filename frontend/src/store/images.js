import { handleErrors, normalize } from ".";

// ---TYPES--- \\
const GET_IMAGE = "images/GET_IMAGE";
const GET_ALL_IMAGES = "images/GET_ALL_IMAGES";
const GET_USER_IMAGES = "images/GET_USER_IMAGES";
const UPLOAD_IMAGE = "images/UPLOAD_IMAGE";
const DELETE_IMAGE = "images/DELETE_IMAGE";

// ---ACTIONS--- \\
const _getImage = (image) => ({
  type: GET_IMAGE,
  image,
});

const _getAllImages = (images) => ({
  type: GET_ALL_IMAGES,
  images,
});

const _getUserImages = (images) => ({
  type: GET_USER_IMAGES,
  images,
});

const _uploadImage = (image) => ({
  type: UPLOAD_IMAGE,
  image,
});

const _deleteImage = (imageId) => ({
  type: DELETE_IMAGE,
  imageId,
});

// ---ACTION DISPATCHERS--- \\
export const getImage = (imageId) => async (dispatch) => {
  const response = await fetch(`/api/images/${imageId}`);

  if (!response.ok) return await handleErrors(response);

  const { image } = await response.json();
  dispatch(_getImage(image));

  return image;
};

export const getAllImages = () => async (dispatch) => {
  const response = await fetch(`/api/images`);

  if (!response.ok) return await handleErrors(response);

  const { images } = await response.json();
  dispatch(_getAllImages(images));

  return images;
};

export const getUserImages = () => async (dispatch) => {
  const response = await fetch(`/api/images/curr`);

  if (!response.ok) return await handleErrors(response);

  const { images } = await response.json();
  dispatch(_getUserImages(images));

  return images;
};

export const uploadImage = (imageData) => async (dispatch) => {
  const response = await fetch(`/api/images`, {
    method: "POST",
    body: imageData,
  });

  if (!response.ok) return await handleErrors(response);

  const { image } = await response.json();
  dispatch(_uploadImage(image));

  return image;
};

export const deleteImage = (imageId) => async (dispatch) => {
  const response = await fetch(`/api/images/${imageId}`, {
    method: "DELETE",
  });

  if (!response.ok) return await handleErrors(response);

  const { message } = await response.json();
  dispatch(_deleteImage(imageId));

  return message;
};

// ---REDUCER--- \\
const initialState = { currImage: {}, allImages: {}, userImages: {} };

const imageReducer = (state = initialState, action) => {
  switch (action.type) {
    case GET_IMAGE: {
      const newState = normalize(state);

      newState.currImage = normalize(action.image);

      return newState;
    }

    case GET_ALL_IMAGES: {
      const newState = normalize(state);

      newState.allImages = normalize(action.images);

      return newState;
    }

    case GET_USER_IMAGES: {
      const newState = normalize(state);

      newState.userImages = normalize(action.images);

      return newState;
    }

    case UPLOAD_IMAGE: {
      const newState = normalize(state);

      newState.currImage = normalize(action.image);
      newState.allImages[action.image.id] = normalize(action.image);
      newState.userImages[action.image.id] = normalize(action.image);

      return newState;
    }

    case DELETE_IMAGE: {
      const newState = normalize(state);

      delete newState.userImages[action.imageId];
      delete newState.allImages[action.imageId];

      return newState;
    }

    default:
      return state;
  }
};

export default imageReducer;
