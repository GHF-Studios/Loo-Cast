using System.Collections.Generic;

namespace LooCast.StateMachine
{
    public class FiniteStateMachine<T>
    {
        protected Dictionary<T, State<T>> states;
        protected State<T> currentState;

        public FiniteStateMachine()
        {
            states = new Dictionary<T, State<T>>();
        }

        #region Current State Callbacks
        public void Update()
        {
            currentState?.Update();
        }

        public void FixedUpdate()
        {
            currentState?.FixedUpdate();
        }

        public void PauseableUpdate()
        {
            currentState?.PauseableUpdate();
        }

        public void PauseableFixedUpdate()
        {
            currentState?.PauseableFixedUpdate();
        }
        #endregion

        public void Add(State<T> state)
        {
            states.Add(state.ID, state);
        }

        public void Add(T stateID, State<T> state)
        {
            states.Add(stateID, state);
        }

        public State<T> GetState(T stateID)
        {
            if (states.ContainsKey(stateID))
            {
                return states[stateID];
            }
            return null;
        }

        public void SetCurrentState(T stateID)
        {
            State<T> state = states[stateID];
            SetCurrentState(state);
        }

        public State<T> GetCurrentState()
        {
            return currentState;
        }

        public void SetCurrentState(State<T> state)
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