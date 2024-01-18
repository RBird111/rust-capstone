import { useDispatch } from "react-redux";
import { useModal } from "../../../context/Modal";
import { useHistory } from "react-router-dom";
import { useState } from "react";

import "./ConfirmDelete.scss";
import { getBusiness } from "../../../store/business";
import DefaultButton from "../DefaultButton";
import ReviewForm from "../../ReviewForm";
import BusinessForm from "../../BusinessForm";

const ConfirmDelete = ({ business, thunk, item }) => {
  const dispatch = useDispatch();
  const history = useHistory();

  const { setModalContent, closeModal } = useModal();

  const [yesText, setYesText] = useState("Yes");
  const [noText, setNoText] = useState("No");

  const handleDelete = async (e) => {
    e.preventDefault();

    if (item.body) {
      await dispatch(thunk(item.id));
      await dispatch(getBusiness(business.id));
      closeModal();
    } else {
      history.push("/");
      await dispatch(thunk(item.id));
      closeModal();
    }
  };

  const handleCancel = (e) => {
    e.preventDefault();

    if (item.body)
      setModalContent(<ReviewForm business={business} review={item} />);
    else setModalContent(<BusinessForm business={business} />);
  };

  return (
    <div className="confirm-delete">
      <div className="confirm">
        <p className="sure">Are you sure?</p>

        <div className="btns">
          <div
            className="yes"
            onMouseEnter={(e) => {
              setYesText("Delete");
            }}
            onMouseLeave={(e) => {
              setYesText("Yes");
            }}
          >
            <DefaultButton onClick={handleDelete} text={yesText} />
          </div>

          <div
            className="no"
            onMouseEnter={(e) => {
              setNoText("Go Back");
            }}
            onMouseLeave={(e) => {
              setNoText("No");
            }}
          >
            <DefaultButton onClick={handleCancel} text={noText} />
          </div>
        </div>
      </div>
    </div>
  );
};

export default ConfirmDelete;
