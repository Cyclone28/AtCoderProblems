import { Button, Col, Row } from "reactstrap";
import React from "react";
import { useHistory } from "react-router-dom";
import { connect, PromiseState } from "react-refetch";
import { VirtualContestTable } from "../VirtualContestTable";
import { VirtualContestInfo } from "../types";
import { CONTEST_JOINED, CONTEST_MY } from "../ApiUrl";

interface Props {
  ownedContestsGet: PromiseState<VirtualContestInfo[] | null>;
  joinedContestsGet: PromiseState<VirtualContestInfo[] | null>;
}
const InnerMyContestList: React.FC<Props> = (props) => {
  const history = useHistory();
  const joinedContests = props.joinedContestsGet.fulfilled
    ? props.joinedContestsGet.value
    : [];
  const ownedContests = props.ownedContestsGet.fulfilled
    ? props.ownedContestsGet.value
    : [];
  return (
    <>
      <Row className="my-2">
        <Col sm="12">
          <Button
            color="success"
            onClick={(): void => {
              history.push({ pathname: "/contest/create" });
            }}
          >
            Create New Contest
          </Button>
        </Col>
      </Row>
      <Row className="my-2">
        <Col sm="12">
          <h2>My Contests</h2>
        </Col>
      </Row>
      <Row className="my-2">
        <Col sm="12">
          <VirtualContestTable
            contests={
              ownedContests
                ? ownedContests.sort(
                    (a, b) =>
                      b.start_epoch_second +
                      b.duration_second -
                      a.start_epoch_second -
                      a.duration_second
                  )
                : []
            }
          />
        </Col>
      </Row>

      <Row className="my-2">
        <Col sm="12">
          <h2>Joined Contests</h2>
        </Col>
      </Row>
      <Row className="my-2">
        <Col sm="12">
          <VirtualContestTable
            contests={
              joinedContests
                ? joinedContests.sort(
                    (a, b) =>
                      b.start_epoch_second +
                      b.duration_second -
                      a.start_epoch_second -
                      a.duration_second
                  )
                : []
            }
          />
        </Col>
      </Row>
    </>
  );
};

export const MyContestList = connect<unknown, Props>(() => ({
  ownedContestsGet: {
    url: CONTEST_MY,
  },
  joinedContestsGet: {
    url: CONTEST_JOINED,
  },
}))(InnerMyContestList);
