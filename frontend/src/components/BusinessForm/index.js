import { useDispatch, useSelector } from "react-redux";
import { useEffect, useState } from "react";
import { useHistory } from "react-router-dom";

import "./BusinessForm.scss";
import { useModal } from "../../context/Modal";
import FormInput, { handleErrors, toInput } from "../FormElements/FormInput";
import FormSelect, { toSelectInput } from "../FormElements/FormSelect";
import {
  createBusiness,
  deleteBusiness,
  updateBusiness,
} from "../../store/business";
import ConfirmDelete from "../FormElements/ConfirmDelete";
import Error from "../FormElements/Error";
import DefaultButton from "../FormElements/DefaultButton";

const BusinessForm = ({ business }) => {
  const dispatch = useDispatch();
  const history = useHistory();

  const { setModalContent, closeModal } = useModal();
  const businessData = useSelector((state) => state.business.currBusiness);

  const [name, setName] = useState(business ? business.name : "");
  const [category, setCategory] = useState(business ? business.category : "");
  const [address, setAddress] = useState(
    business ? business.location.address : ""
  );
  const [city, setCity] = useState(business ? business.location.city : "");
  const [state, setState] = useState(business ? business.location.state : "");
  const [description, setDescription] = useState(
    business ? business.description : ""
  );

  const [errors, setErrors] = useState({});
  const [isSubmitted, setIsSubmitted] = useState(false);

  // Validations
  useEffect(() => {
    setErrors({});
    const errorsObj = {};

    if (!name) errorsObj.name = "Must give a name";
    else if (name.length > 40)
      errorsObj.name = "Name must be less than 40 characters";

    if (!address) errorsObj.address = "Must give a address";
    else if (address.length > 40)
      errorsObj.address = "Address must be less than 40 characters";

    if (!city) errorsObj.city = "Must give a city";
    else if (city.length > 40)
      errorsObj.city = "City must be less than 40 characters";

    if (!state) errorsObj.state = "Must give a state";
    else if (state.length > 40)
      errorsObj.state = "State must be less than 40 characters";

    if (!description) errorsObj.description = "Must fill out description";
    else if (description.length > 1000)
      errorsObj.description = "Description must be less than 1000 characters";

    if (!category) errorsObj.category = "Must select category";
    else if (category.length > 40)
      errorsObj.category = "Category must be less than 40 characters";

    setErrors(errorsObj);
  }, [address, category, city, description, name, state]);

  const handleSubmit = async (e) => {
    e.preventDefault();

    setIsSubmitted(true);

    if (Object.values(errors).length === 0) {
      const formData = business ? { ...businessData } : {};

      formData.name = name;
      formData.description = description;
      formData.category = category;
      formData.address = address;
      formData.city = city;
      formData.state = state;

      let data;
      if (business) {
        data = await dispatch(updateBusiness(formData));
      } else {
        data = await dispatch(createBusiness(formData));
      }

      if (data.errors) {
        const errorsObj = {};

        for (const error of data.errors) {
          const [name, message] = error.split(" : ");
          errorsObj[name] = message;
        }

        return setErrors(errorsObj);
      }

      closeModal();
      history.push(`/business/${data.id}`);
    }
  };

  return (
    <div className="business-form">
      {business && (
        <div
          className="delete"
          title="Click to delete business"
          onClick={() =>
            setModalContent(
              <ConfirmDelete
                item={business}
                thunk={deleteBusiness}
                business={business}
              />
            )
          }
        >
          <i className="fa-solid fa-trash" />
        </div>
      )}

      <h1>{business ? "Update " : "Add a "}Business</h1>

      <form onSubmit={handleSubmit}>
        <FormInput
          input={toInput("Business Name", name, setName)}
          handleErrors={handleErrors(isSubmitted, errors.name)}
        />

        <FormSelect
          input={toSelectInput(category, setCategory, [
            "automotive",
            "home services",
            "restaurant",
            "shopping",
          ])}
          handleErrors={handleErrors(isSubmitted, errors.category)}
        />

        <div className="d-error">
          {isSubmitted && errors.location_id && (
            <Error error={errors.location_id} />
          )}
        </div>

        <FormInput
          input={toInput("Address", address, setAddress)}
          handleErrors={handleErrors(isSubmitted, errors.address)}
        />

        <FormInput
          input={toInput("City", city, setCity)}
          handleErrors={handleErrors(isSubmitted, errors.city)}
        />

        <FormInput
          input={toInput("State", state, setState)}
          handleErrors={handleErrors(isSubmitted, errors.state)}
        />

        <div className="d-error">
          {isSubmitted && errors.description && (
            <Error error={errors.description} />
          )}
        </div>
        <textarea
          placeholder="Enter a description of your business..."
          value={description}
          onChange={(e) => setDescription(e.target.value)}
        />

        <DefaultButton text={`${business ? "Update" : "Add"} Business`} />
      </form>
    </div>
  );
};

export default BusinessForm;
