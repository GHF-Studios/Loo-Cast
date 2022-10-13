namespace LooCast.StateMachine
{
    public class State<T>
    {
        public T ID { get; private set; }
        public string Name { get; private set; }

        public delegate void DelegateNoArg();
        public DelegateNoArg OnEnter;
        public DelegateNoArg OnExit;
        public DelegateNoArg OnUpdate;
        public DelegateNoArg OnFixedUpdate;
        public DelegateNoArg OnPauseableUpdate;
        public DelegateNoArg OnPauseableFixedUpdate;

        public State(T id)
        {
            ID = id;
        }

        public State(T id, string name) : this(id)
        {
            Name = name;
        }

        public State
        (
            T id, 
            DelegateNoArg onEnter, 
            DelegateNoArg onExit = null, 
            DelegateNoArg onUpdate = null, 
            DelegateNoArg onFixedUpdate = null, 
            DelegateNoArg onPauseableUpdate = null, 
            DelegateNoArg onPauseableFixedUpdate = null
        ) : this(id)
        {
            OnEnter = onEnter;
            OnExit = onExit;
            OnUpdate = onUpdate;
            OnFixedUpdate = onFixedUpdate;
            OnPauseableUpdate = onPauseableUpdate;
            OnPauseableFixedUpdate = onPauseableFixedUpdate;
        }

        public State
        (
            T id, 
            string name, 
            DelegateNoArg onEnter, 
            DelegateNoArg onExit = null, 
            DelegateNoArg onUpdate = null, 
            DelegateNoArg onFixedUpdate = null, 
            DelegateNoArg onPauseableUpdate = null, 
            DelegateNoArg onPauseableFixedUpdate = null
        ) : this(id, name)
        {
            OnEnter = onEnter;
            OnExit = onExit;
            OnUpdate = onUpdate;
            OnFixedUpdate = onFixedUpdate;
            OnPauseableUpdate = onPauseableUpdate;
            OnPauseableFixedUpdate = onPauseableFixedUpdate;
        }

        public virtual void Enter()
        {
            OnEnter?.Invoke();
        }

        public virtual void Exit()
        {
            OnExit?.Invoke();
        }

        public virtual void Update()
        {
            OnUpdate?.Invoke();
        }

        public virtual void FixedUpdate()
        {
            OnFixedUpdate?.Invoke();
        }

        public virtual void PauseableUpdate()
        {
            OnPauseableUpdate?.Invoke();
        }

        public virtual void PauseableFixedUpdate()
        {
            OnPauseableFixedUpdate?.Invoke();
        }
    }
}