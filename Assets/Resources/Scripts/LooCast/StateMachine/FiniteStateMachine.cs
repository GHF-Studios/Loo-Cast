using System.Collections.Generic;

namespace LooCast.StateMachine
{
    using Core;

    public class FiniteStateMachine<T> : ExtendedMonoBehaviour
    {
        protected Dictionary<T, State<T>> states;
        protected State<T> currentState;

        public FiniteStateMachine()
        {
            states = new Dictionary<T, State<T>>();
        }

        #region Current State Callbacks
        private void Update()
        {
            currentState?.Update();
        }

        private void FixedUpdate()
        {
            currentState?.FixedUpdate();
        }

        protected override void PauseableUpdate()
        {
            currentState?.PauseableUpdate();
        }

        protected override void PauseableFixedUpdate()
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