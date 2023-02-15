using System.Collections.Generic;

namespace LooCast.StateMachine
{
    public class FiniteStateMachine<StateEnumType> where StateEnumType : System.Enum
    {
        protected Dictionary<StateEnumType, State<StateEnumType>> states;
        protected State<StateEnumType> currentState;

        public FiniteStateMachine()
        {
            states = new Dictionary<StateEnumType, State<StateEnumType>>();
        }

        public void Update()
        {
            currentState?.Update();
        }

        public void FixedUpdate()
        {
            currentState?.FixedUpdate();
        }

        public void Add(State<StateEnumType> state)
        {
            states.Add(state.ID, state);
        }

        public void Add(StateEnumType stateID, State<StateEnumType> state)
        {
            states.Add(stateID, state);
        }

        public State<StateEnumType> GetState(StateEnumType stateID)
        {
            if (states.ContainsKey(stateID))
            {
                return states[stateID];
            }
            return null;
        }

        public void SetCurrentState(StateEnumType stateID)
        {
            State<StateEnumType> state = states[stateID];
            SetCurrentState(state);
        }

        public State<StateEnumType> GetCurrentState()
        {
            return currentState;
        }

        public void SetCurrentState(State<StateEnumType> state)
        {
            if (currentState == state)
            {
                return;
            }

            if (currentState != null)
            {
                currentState.Exit();
            }

            currentState = state;

            if (currentState != null)
            {
                currentState.Enter();
            }
        }
    }
}